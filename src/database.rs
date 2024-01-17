use std::path::PathBuf;
use polodb_core::{Database, Collection, bson::{doc, Document, Bson}};
use serde::{Serialize, Deserialize};
use tidal_rs::model::Track;


pub struct DatabaseWrapper {
    pub database:Database
}

pub struct SongController {
    pub collection:Collection<Song>
}

impl SongController {
    pub fn add_song(&self, song:Song) {
        let _ = self.collection.insert_one(song);
    }

    pub fn get_songs(&self) -> Vec<Song> {
        if let Ok(cursor) = self.collection.find(doc! {}) {
            return cursor.map(|x| x.unwrap()).collect();
        } else {
            return vec![];
        }
    }

    pub fn remove_song(&self, song:Song) -> Result<(), polodb_core::Error>{
        self.collection.delete_one(doc! {
            "path": song.path.to_str().unwrap()
        })?;

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq)]
pub struct Song {
    pub path:PathBuf,
    pub title:String,
    pub artist:String,
    pub album:String,
    pub tidal_track:Option<Track>
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

impl DatabaseWrapper {
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

    pub fn new() -> Self {
        DatabaseWrapper {
            database:Database::open_file(DatabaseWrapper::get_database_path()).expect("Failed to open database"),
        }
    }

    pub fn songs(&self) -> SongController {
        SongController {
            collection:self.database.collection("songs")
        }
    }
}