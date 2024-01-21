use std::sync::Arc;

use crate::{song::Song, database::DatabaseData, gui::model::DrawableSong, cache::CacheManager};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Playlist {
    pub name:String,
    pub songs:Vec<u64> // Vec to hashes of songs to gain performance and reduce database size
}


pub struct DrawablePlaylist {
    pub name:String,
    pub songs:Vec<DrawableSong>
}

impl DrawablePlaylist {
    pub async fn new(playlist:&Playlist, database:DatabaseData, ctx:&egui::Context, cache_manager:Arc<tokio::sync::Mutex<CacheManager>>) -> Self {
        let name = playlist.name.clone();

        let songs = {
            let data = database.lock().unwrap();
            playlist.songs.iter().map(|hash| {
                let song = data.tracks().iter().find(|x| x.hash == *hash).unwrap();
                song.song.clone()
            }).collect::<Vec<Song>>()
        };

        let mut drawable_songs:Vec<DrawableSong> = Vec::with_capacity(songs.len());
        for song in songs {
            drawable_songs.push(DrawableSong::from_song(ctx, song, cache_manager.clone()).await);
        }

        DrawablePlaylist {
            name,
            songs:drawable_songs
        }
    }
}