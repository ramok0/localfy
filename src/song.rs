use std::{collections::hash_map::DefaultHasher, hash::{self, Hash, Hasher}, path::PathBuf, sync::Arc};
use egui::Response;
use tidal_rs::model::{Album, Track};

use crate::{app::{self, App, AppImpl}, gui::model::{Pages, UserLocation}, playlist::Playlist, renderer::Drawable};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq)]
pub struct Song {
    pub path:PathBuf,
    pub title:String,
    pub artist:String,
    pub album:String,
    pub tidal_track:Option<Track>
}

pub fn resolve_hashed_songs(app:Arc<app::AppImpl>, album:&Album) -> Vec<u64> {
    let raw = app.database().raw();
    let tracks = raw.data.lock().unwrap().albums.iter().find(|x| x.album.id == album.id).and_then(|x| Some(x.tracks.clone()));

    tracks.unwrap_or(vec![]) 
}

impl Song {
    pub fn resolve(app:Arc<AppImpl>, track:&Track) -> Option<Song> {
        let raw = app.database().raw();
        let data = raw.data.lock().unwrap();
        let result = data.tracks.0.iter().find(|x| x.1.tidal_track.as_ref() == Some(track));

        result.and_then(|x| Some(x.1.clone()))
    }

    

    pub fn on_clicked(&self, app:Arc<AppImpl>, from:UserLocation) {
        println!("on clicked event");
        match from {
            UserLocation::Home => { //Play the song, then set the radio associated with the song in the playlist
                let _ = app.player.set_media(&self);
            },
            UserLocation::Playlist(playlist) => {
                let _ = app.player.set_media(&self);

                let mut queue = app.player.queue();

                //verifier si la playlist actuelle a besoin d'etre changée
                //current hashed queue
                let hashed_queue = queue.get_playlist().iter().map(|song| {
                    let mut hasher = DefaultHasher::new();
                    song.hash(&mut hasher);
                    hasher.finish()
                }).collect::<Vec<u64>>();

                let hashed_playlist = {
                    playlist.hash_songs(app.clone())
                };

                if hashed_queue != hashed_playlist {
                    let songs = {
                        app.database().playlists().unhash_playlist_songs(&playlist)
                    };

                    if let Some(resolved_playlist) = songs {
                        queue.set_playlist(&resolved_playlist.songs);

                        let mut hasher = DefaultHasher::new();
                        self.hash(&mut hasher);
                        let hash = hasher.finish();
    
                        if let Some(index) = hashed_playlist.iter().position(|x| x == &hash) {
                            queue.current_index = Some(index);
                        }
                    }
                }

            },
            UserLocation::Album(album) => {
                let _ = app.player.set_media(&self);
                let mut queue = app.player.queue();

                let hashed_queue = queue.get_playlist().iter().map(|song| {
                    let mut hasher = DefaultHasher::new();
                    song.hash(&mut hasher);
                    hasher.finish()
                }).collect::<Vec<u64>>();

                let hashed_album = {
                    resolve_hashed_songs(app.clone(), &album)
                };

                if hashed_queue != hashed_album {
                    let songs = {
                        app.database().albums().get_album_tracks(app.clone(), &album)
                    };


                        queue.set_playlist(&songs);

                        let mut hasher = DefaultHasher::new();
                        self.hash(&mut hasher);
                        let hash = hasher.finish();
    
                        if let Some(index) = hashed_album.iter().position(|x| x == &hash) {
                            queue.current_index = Some(index);
                        }
                    
                }
            },
            UserLocation::Artist(artist) => {
            }
        }
    }

    pub fn context_menu(&self, response:Response, ui:&mut egui::Ui, application:&app::App,) {
        response.context_menu(|ui| {
            if ui.button("Add to playlist").clicked() { //todo: add to playlist
            }
    
            if application.gui_settings.page == Pages::Playlist {
                if let UserLocation::Playlist(playlist) = &application.gui_settings.location {
    
    
                    if ui.button("Remove from playlist").clicked() {
                        application.app.database().playlists().remove_from_playlist(playlist, &self);
                        ui.close_menu();
                    }
                }
            }
    
            if ui.button("Add to queue").clicked() {
                let mut queue = application.app.player.queue();
                queue.add_to_queue(&self);
            }
    
            let mixes = self.tidal_track.as_ref().and_then(|track| track.mixes.as_ref().and_then(|mixes| Some(mixes.clone())));
            
            //bouton pour impoter la radio liée au son dans les playlists
    
            if let Some(mix) = mixes.as_ref().and_then(|mixes| mixes.track_mix.clone()) {
                if ui.button("Add radio to playlist").clicked() {
                    let tx = application.gui_settings.event_manager.0.clone();
                    let app = application.app.clone();
                    let client = application.app.tidal_client.clone();
                    let track_name = self.get_title();
                    let quality = application.app.get_quality_or_highest_avaliable();
    
                    tokio::spawn(async move {
                        if let Ok(tracks) = client.media().get_mixes_items(&mix, None).await {
    
                            let playlist = Playlist {
                                id: mix.to_string(),
                                name: format!("Radio - {}", track_name),
                                image: None,
                                songs: vec![]
                            };
    
                            app.database().playlists().add_playlist(&playlist);
    
                            for track in tracks {
                                app.download_manager.enqueue_single(app.clone(), quality, track.clone(), Some(&playlist)).await;        
                            }

                            
                        }
                    });
                }
            }
        });

        

    }

}

impl Song {
    pub fn new(path:PathBuf, title:String, artist:String, album:String) -> Self {
        Song {
            path,
            title,
            artist,
            album,
            tidal_track:None
        }
    }

    pub fn new_with_track(path:PathBuf, tidal_track:Track) -> Self {
        Song {
            path,
            title:tidal_track.title.clone(),
            artist:tidal_track.get_artist().name.clone(),
            album:tidal_track.album.clone().map(|x| x.title).unwrap_or("Unknown".to_string()),
            tidal_track:Some(tidal_track)
        }
    }
}