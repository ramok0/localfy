use std::{sync::{Arc, Mutex}, collections::{VecDeque, HashMap}, time::{Duration, Instant}, env, path::PathBuf};

use tidal_rs::model::{Track, PlaybackManifest, Album, AudioQuality};
use tokio::task;
use tokio::io::AsyncWriteExt;

use crate::{app::AppImpl};
use crate::song::Song;

#[derive(Clone)]
pub struct Download {
    pub track:Track,
    pub manifest:PlaybackManifest,
    pub path:PathBuf,
    pub app:Arc<AppImpl>
}

#[derive(Clone, PartialEq)]
pub enum DownloadStatus {
    None,
    Queued,
    Downloading,
    Finished,
    Failed(String)
}

impl ToString for DownloadStatus {
    fn to_string(&self) -> String {
        match self {
            DownloadStatus::None => "None".to_string(),
            DownloadStatus::Queued => "Queued".to_string(),
            DownloadStatus::Downloading => "Downloading".to_string(),
            DownloadStatus::Finished => "Finished".to_string(),
            DownloadStatus::Failed(Message) => format!("Failed with message : {}", Message)
        }
    }

}

#[derive(Clone)]
pub struct DataRate {
    bytes_per_second: f32,
}

impl DataRate {
    fn new(bytes_per_second: f32) -> Self {
        DataRate {
            bytes_per_second,
        }
    }

    pub fn bytes_per_second(&self) -> f32 {
        self.bytes_per_second
    }

    pub fn as_kbps(&self) -> f64 {
        (self.bytes_per_second as f64 * 8.0) / 1000.0
    }

    pub fn as_mbps(&self) -> f64 {
        (self.bytes_per_second as f64 * 8.0) / 1000000.0
    }
}


#[derive(Clone)]
pub struct DownloadState {
    pub   download:Download,
    pub   downloaded:usize,
    pub   total_size:usize,
    pub   speed:DataRate,
    pub    progress:f32,
    pub    status:DownloadStatus,
    pub started_at:Instant
}

impl DownloadState {
    pub fn new(download:Download, total_size:usize) -> Self {
        DownloadState {
            download,
            downloaded: 0,
            total_size,
            speed: DataRate::new(0.0),
            progress: 0.0,
            status: DownloadStatus::Downloading,
            started_at:Instant::now()
        }
    }
}

impl Download {
    pub fn new(app:Arc<AppImpl>, track:Track, manifest:PlaybackManifest, path:Option<PathBuf>) -> Self {
        let path = path.unwrap_or(Self::old_file_path(track.clone(), manifest.clone()));

        Download {
            app,
            track,
            manifest,
            path 
        }
    }

    pub fn track(&self) -> &Track {
        &self.track
    }

    pub fn manifest(&self) -> &PlaybackManifest {
        &self.manifest
    }

    pub fn old_file_path(track:Track, manifest:PlaybackManifest) -> PathBuf {
        let filename = format!("{} - {}.{}", track.get_artist().name, track.title, manifest.mime_type.get_file_extension());
        
        if let Ok(programdata) = env::var("PROGRAMDATA") {
            let path = format!("{}\\Localfy\\{}", programdata, filename);

            return PathBuf::from(path);
        }

        PathBuf::from(filename)
    }

    pub fn on_finished(&self) {
        let mut database = self.app.database.lock().unwrap();
        database.songs().add_song(Song::new_with_track(self.path.clone(), self.track.clone()));
        database.flush();
    }
}

pub struct DownloadManager {
    download_queue:Arc<Mutex<VecDeque<Download>>>,
    download_state:Arc<Mutex<HashMap<Track, DownloadState>>>,
    max_concurrency:usize
}

impl DownloadManager {
    pub fn new(max_concurrency: usize) -> Self {
        DownloadManager {
            download_queue:Arc::new(Mutex::new(VecDeque::new())),
            download_state:Arc::new(Mutex::new(HashMap::new())),
            max_concurrency
        }
    }

    pub fn enqueue(&self, download:Download) -> () {
        self.download_queue.lock().unwrap().push_back(download);
    }

    pub fn get_queue(&self) -> VecDeque<Download>
    {
        self.download_queue.lock().unwrap().clone()
    }

    pub fn get_downloads(&self) -> Vec<DownloadState>
    {
        self.download_state.lock().unwrap().values().cloned().collect()
    }

    pub fn remove_download(&self, download:Download) {
        self.download_queue.lock().unwrap().retain(|x| x.track != download.track);
    }

    pub async fn enqueue_single(&self, app:Arc<AppImpl>, quality:AudioQuality, track:Track) -> Result<(), tidal_rs::error::Error>
    {
        let caracteres_interdits = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
        let mut normalize_string = |x:String| -> String {
                x.chars()
                .map(|c| if caracteres_interdits.contains(&c) { '_' } else { c })
                .collect::<String>()
        };

        let manifest = app.tidal_client.media().get_highest_quality_avaliable_stream_url(track.id, quality).await?;
        let mut base_path = app.configuration.lock().unwrap().get_base_download_folder().join(track.get_artist().name);

        if track.album.is_some() {
            let album_name = track.album.as_ref().unwrap().title.clone();
         

            base_path = base_path.join(normalize_string(album_name));
        }

        if !base_path.exists() {
            
            if std::fs::create_dir_all(&base_path).is_err() {
                dbg!("Failed to create directory : {}", base_path.clone());
            }
        }

        let title = track.title.clone();
        let mime_type = manifest.mime_type.clone();
        let download = Download::new(app.clone(), track, manifest, Some(base_path.join(normalize_string(format!("{}.{}", title.replace("/", "-").replace("\\", "-"), mime_type.get_file_extension())))));

        self.enqueue(download);

        Ok(())
    }

    pub async fn enqueue_album(&self, app:Arc<AppImpl>, album:Album, quality:AudioQuality) -> Result<(), tidal_rs::error::Error>
    {
        let tracks = app.tidal_client.media().get_album_tracks(album.id, None).await.unwrap_or(vec![]);

        for track in tracks {
            self.enqueue_single(app.clone(), quality, track).await?;
        }

        Ok(())
    }

    pub fn work(&self) {
        let queue = Arc::clone(&self.download_queue);
        let download_state = Arc::clone(&self.download_state);

        for _ in 0..self.max_concurrency {
            let queue = Arc::clone(&queue);
            let download_state = Arc::clone(&download_state);

            task::spawn(async move {
                let client = reqwest::Client::new();
                loop {
                    let download = {
                        let mut queue = queue.lock().unwrap();
                        queue.pop_front()
                    };

                    match download {
                        Some(download) => {
                            let url = &download.manifest.urls[0];
                            
                            if let Ok(mut response) = client.get(url).send().await {
                                let total_size = response.content_length().unwrap_or(0) as usize;
                                let mut downloaded = 0;
                                let mut on_last_second_downloaded:(usize, Instant) = (0, Instant::now());
                                let folder = download.path.parent().unwrap();
                                if !folder.exists() {
                                    std::fs::create_dir_all(folder).unwrap();
                                }

                                let state = DownloadState::new(download.clone(), total_size);
                                {
                                    let mut download_state = download_state.lock().unwrap();
                                    download_state.insert(state.download.track.clone(), state);
                                }

                                let file_result = tokio::fs::File::create(download.path.clone()).await;

                                if file_result.is_err() {
                                    let mut download_state = download_state.lock().unwrap();
                                    let state = download_state.get_mut(&download.track).unwrap();
                                    dbg!(download.path.clone());
                                    state.status = DownloadStatus::Failed(file_result.unwrap_err().to_string());
                                    continue;
                                }

                                let mut file = file_result.unwrap();
       

                                while let Some(chunk) = response.chunk().await.unwrap() {
                                    downloaded += chunk.len() as u64;
                                    on_last_second_downloaded.0 += chunk.len();

                                    file.write_all(&chunk).await.unwrap();

                                    //calculate speed, eta and progress, then update the state
                                    {
                                        let mut download_state = download_state.lock().unwrap();
                                        let state = download_state.get_mut(&download.track).unwrap();

                                        state.downloaded = downloaded as usize;
                                      
                                        //calculer la vitesse
                                        let elapsed = state.started_at.elapsed();
                                        //bytes per second
                                        state.speed = DataRate::new(on_last_second_downloaded.0 as f32 / on_last_second_downloaded.1.elapsed().as_secs_f32());
                                        state.progress = downloaded as f32 / total_size as f32;
                                        state.status = DownloadStatus::Downloading;    

                                        if on_last_second_downloaded.1.elapsed().as_secs() >= 1 {
                                            on_last_second_downloaded.1 = Instant::now();
                                            on_last_second_downloaded.0 = 0;
                                        }
                                    };
                                }

                                {
                                    let mut download_state = download_state.lock().unwrap();
                                    let state = download_state.get_mut(&download.track).unwrap();

                                    state.status = DownloadStatus::Finished;
                                    download.on_finished();
                                }
                            }
                        }
                        None => (),
                    }

                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            });
        }
    }
}