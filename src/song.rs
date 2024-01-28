use std::{path::PathBuf, sync::Arc};
use tidal_rs::model::Track;

use crate::{app::AppImpl, gui::model::UserLocation};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq)]
pub struct Song {
    pub path:PathBuf,
    pub title:String,
    pub artist:String,
    pub album:String,
    pub tidal_track:Option<Track>
}

impl Song {
    pub fn on_clicked(&self, app:Arc<AppImpl>, from:UserLocation) {
        println!("on clicked event");
        match from {
            UserLocation::Home => { //Play the song, then set the radio associated with the song in the playlist
                println!("bjr");
                dbg!(app.player.set_media(&self));
            },
            UserLocation::Playlist(playlist) => {
                let _ = app.player.set_media(&self);

                let mut queue = app.player.queue();

                if queue.get_playlist() != &playlist.songs {
                    queue.set_playlist(&playlist.songs);

                    if let Some(index) = playlist.songs.iter().position(|x| x == self) {
                        queue.current_index = Some(index);
                    }
                }

            },
            UserLocation::Album(page, album) => {
            },
            UserLocation::Artist(page, artist) => {
            }
        }
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