use std::{
    path::PathBuf,
    fs::File,
    sync::{ Arc, Mutex },
    collections::{hash_map::DefaultHasher, HashMap},
    hash::Hasher,
    io::BufRead, time::Instant,
};

use std::hash::Hash;

use tidal_rs::model::{Album, Track};

use crate::{ app::AppImpl, playlist::{DecodedPlaylist, Playlist, PlaylistDescriptor}, song::Song };

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CachedHashSong {
    pub hash: u64,
    pub song: Song,
}

#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct AlbumHashed {
    pub album: Album,
    pub tracks: Vec<u64>,
}

#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct AlbumWithSongs {
    pub album: Album,
    pub tracks: Vec<Song>,
}

impl PartialEq for AlbumHashed {
    fn eq(&self, other: &Self) -> bool {
        self.album.id == other.album.id
    }
}

impl PartialEq for CachedHashSong {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl From<Song> for CachedHashSong {
    fn from(value: Song) -> Self {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);

        CachedHashSong {
            hash: hasher.finish(),
            song: value,
        }
    }
}
impl From<&Song> for CachedHashSong {
    fn from(value: &Song) -> Self {
        CachedHashSong::from(value.clone())
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TrackHashMap(pub HashMap<u64, Song>);

impl Hash for TrackHashMap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hasher = DefaultHasher::new();
        self.0.keys().collect::<Vec<&u64>>().hash(&mut hasher);
        hasher.finish().hash(state);
    }
}

impl Default for TrackHashMap {
    fn default() -> Self {
        TrackHashMap(HashMap::new())
    }
}

#[derive(Clone, Debug, Hash, serde::Serialize, serde::Deserialize)]
pub struct DatabaseDataContainer {
    #[serde(default)]
    pub tracks: TrackHashMap,
    #[serde(default)]
    pub playlists: Vec<Playlist>,
    #[serde(default)]
    pub albums: Vec<AlbumHashed>,
}



impl DatabaseDataContainer {
    pub fn tracks(&self) -> Vec<Song> {
        self.tracks.0.values().cloned().collect()
    }

    pub fn tracks_full(&self) -> &HashMap<u64, Song> {
        &self.tracks.0
    }
}

impl Default for DatabaseDataContainer {
    fn default() -> Self {
        DatabaseDataContainer {
            tracks: TrackHashMap::default(),
            playlists: Vec::new(),
            albums: Vec::new()
        }
    }
}

pub type DatabaseData = std::sync::Arc<std::sync::Mutex<DatabaseDataContainer>>;

#[derive(Clone)]
pub struct Database 
{
    pub inner: Arc<DatabaseImpl>,
    last_save_hash: u64,
    last_save_time: std::time::Instant
}

impl Drop for Database {
    fn drop(&mut self) {
        self.flush().unwrap();
    }
}

impl Database {
    pub fn new() -> Self {
        let database = DatabaseImpl::new();
        let hash = database.hash();

        Database {
            inner: Arc::new(database),
            last_save_hash: hash,
            last_save_time: Instant::now()
        }
    }

    pub fn raw(&self) -> Arc<DatabaseImpl> {
        self.inner.clone()
    } 

    pub fn last_save_hash(&self) -> u64 {
        self.last_save_hash
    }

    pub fn last_save_time(&self) -> Instant {
        self.last_save_time
    }

    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        let str = {
            let data = self.inner.data.lock().unwrap();
            serde_json::to_string(&data.clone()).unwrap()
        };

        // let mut e = ZlibEncoder::new(Vec::new(), Compression::best());
        // e.write_all(str.as_bytes())?;
        std::fs::write(DatabaseImpl::get_database_path(), str)?;

        self.last_save_hash = self.inner.hash();
        Ok(())
    }

    pub fn songs(&self) -> SongController {
        SongController {
            database: self.inner.clone(),
        }
    }

    pub fn playlists(&self) -> PlaylistController {
        PlaylistController {
            database: self.inner.clone(),
        }
    }

    pub fn albums(&self) -> AlbumController {
        AlbumController {
            database: self.inner.clone(),
        }
    }
}

#[derive(Clone)]
pub struct DatabaseImpl {
    pub data: Arc<Mutex<DatabaseDataContainer>>,
}

pub struct SongController {
    database: Arc<DatabaseImpl>,
}

pub struct PlaylistController {
    database: Arc<DatabaseImpl>,
}



pub struct AlbumController {
    database: Arc<DatabaseImpl>,
}

impl AlbumController {
    pub fn add_album(&self, album: &Album, tracks:Vec<Song>) {
        let mut data: std::sync::MutexGuard<'_, DatabaseDataContainer> = self.database.data.lock().unwrap();
        
        let hashed_songs = tracks.iter().map(|song| CachedHashSong::from(song).hash).collect::<Vec<u64>>();

        let album_with_tracks = AlbumHashed {
            album: album.clone(),
            tracks: hashed_songs,
        };

        if data.albums.contains(&album_with_tracks) {
            data.albums.iter_mut().find(|x| x.album.id == album_with_tracks.album.id).unwrap().tracks = album_with_tracks.tracks.clone();
        } else {
            data.albums.push(album_with_tracks);
        }
    }

    pub fn get_albums(&self, app:Arc<AppImpl>) -> Vec<AlbumWithSongs> {
        let data = self.database.data
        .lock()
        .unwrap();

            data.albums
            .iter()
            .map(|album| {
                AlbumWithSongs { album: album.album.clone(), tracks: album.tracks.iter().filter_map(|item| data.tracks.0.get(&item).cloned()).collect::<Vec<Song>>() }
            })
            .collect::<Vec<AlbumWithSongs>>()
    }

    pub fn get_album_tracks(&self, app:Arc<AppImpl>, album: &Album) -> Vec<Song>
    {
        let data = self.database.data
        .lock()
        .unwrap();

        let album = data.albums.iter().find(|x| x.album.id == album.id).unwrap();
        album.tracks.iter().filter_map(|item| data.tracks.0.get(&item).cloned()).collect::<Vec<Song>>()
    }

    pub fn resolve_album(&self, album:&Album) -> Vec<Song>
    {
        let data = self.database.data
        .lock()
        .unwrap();

        let album = data.albums.iter().find(|x| x.album.id == album.id).unwrap();
        album.tracks.iter().filter_map(|item| data.tracks.0.get(&item).cloned()).collect::<Vec<Song>>()
    }


}

impl PlaylistController {
    pub fn add_playlist(&self, playlist: &Playlist) {
        let mut data: std::sync::MutexGuard<'_, DatabaseDataContainer> = self.database.data.lock().unwrap();

        data.playlists.push(playlist.clone());
    }

    pub fn push_to_playlist(&self, playlist: &PlaylistDescriptor, songs: &Vec<Song>) {
        let mut data: std::sync::MutexGuard<'_, DatabaseDataContainer> = self.database.data.lock().unwrap();
        if let Some(playlist_index) =  data.playlists.iter().position(|p| p.name == playlist.name) {
            songs.iter().for_each(|song| {
                let cached_song = CachedHashSong::from(song);
                if !data.playlists[playlist_index].songs.contains(&cached_song.hash) {
                    data.playlists[playlist_index].songs.push(cached_song.hash);
                }
            });
        }
    }

    pub fn unhash_playlist_songs(&self, descriptor: &PlaylistDescriptor) -> Option<DecodedPlaylist> {
        let data: std::sync::MutexGuard<'_, DatabaseDataContainer> = self.database.data.lock().unwrap();
        let playlist = data.playlists.iter().find(|p| p.name == descriptor.name)?;
        let songs = playlist.songs.iter().filter_map(|item| data.tracks.0.get(&item).cloned()).collect::<Vec<Song>>();
        
        Some(DecodedPlaylist {
            descriptor: descriptor.clone(),
            songs
        })
    }

    pub fn get_playlists(&self) -> Vec<PlaylistDescriptor> {
        self.database.data
            .lock()
            .unwrap()
            .playlists.iter().map(|playlist| {
                PlaylistDescriptor {
                    id: playlist.id.clone(),
                    name: playlist.name.clone(),
                    image: None //TODO: impl image
                }
            }).collect::<Vec<PlaylistDescriptor>>()
            .clone()
    }

    pub fn remove_from_playlist(&self, playlist:&PlaylistDescriptor, song:&Song) {
        let mut data: std::sync::MutexGuard<'_, DatabaseDataContainer> = self.database.data.lock().unwrap();
        if let Some(playlist_index) =  data.playlists.iter().position(|p| p.name == playlist.name) {
            let cached_song = CachedHashSong::from(song);
            if let Some(song_index) = data.playlists[playlist_index].songs.iter().position(|x| x == &cached_song.hash) {
                data.playlists[playlist_index].songs.remove(song_index);
            }
        }
    }

    pub fn remove_playlist(&self, playlist: &PlaylistDescriptor) -> Result<(), std::io::Error> {
        let mut data: std::sync::MutexGuard<'_, DatabaseDataContainer> = self.database.data.lock().unwrap();
        if let Some(index) = data.playlists.iter().position(|p| p.name == playlist.name) {
            data.playlists.remove(index);
        } else {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Playlist not found"));
        }

        Ok(())
    }
}

impl SongController {
    pub fn add_song(&self, song: Song) {
        let mut data: std::sync::MutexGuard<'_, DatabaseDataContainer> = self.database.data.lock().unwrap();
        let cached_song = CachedHashSong::from(&song);
        data.tracks.0.insert(cached_song.hash, cached_song.song);
    }

    pub fn get_songs(&self) -> Vec<Song> {
        self.database.data
            .lock()
            .unwrap()
            .tracks()
    }

    pub fn remove_song(&self, song: Song) -> Result<(), std::io::Error> {

        let cached_song = CachedHashSong::from(&song);
        {
            let mut data: std::sync::MutexGuard<'_, DatabaseDataContainer> = self.database.data.lock().unwrap();
           if data.tracks.0.remove_entry(&cached_song.hash).is_none() {
                return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Song not found"));
            }
        }

        Ok(())
    }
}



impl DatabaseImpl {
    fn get_database_path() -> PathBuf {
        if let Ok(program_data) = std::env::var("PROGRAMDATA") {
            let path_buf = PathBuf::from(program_data).join("Localfy").join("localfy.db");

            if !path_buf.exists() {
                std::fs::create_dir_all(path_buf.parent().unwrap()).unwrap();
            }

            return path_buf;
        }

        PathBuf::from("localfy.db")
    }

    pub fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.data.lock().unwrap().hash(&mut hasher);
        hasher.finish()
    }

    pub fn read_database(
        file: &File,
        data: &mut DatabaseDataContainer
    ) -> Result<(), std::io::Error> {
        let mut reader = std::io::BufReader::new(file);
        
        // Check if the file is empty
        if reader.fill_buf()?.is_empty() {
            println!("buffer is empty");
            return Ok(());
        }

        

        // let mut d = ZlibDecoder::new(reader.buffer());
        // let mut s = String::new();
    
        // d.read_to_string(&mut s)?;

        let config = serde_json::from_reader(reader);
        if let Ok(config) = config {
            *data = config;
        }
    
        return Ok(());
    }

    pub fn new() -> Self {
        let path = DatabaseImpl::get_database_path();
        

        let mut data_container = DatabaseDataContainer::default();
        if let Ok(file) = File::options().read(true).open(&path) {
            DatabaseImpl::read_database(&file, &mut data_container).unwrap();
        }

        DatabaseImpl {
            data: Arc::new(Mutex::new(data_container))
        }
    }
}

unsafe impl Send for DatabaseImpl {}
unsafe impl Sync for DatabaseImpl {}
