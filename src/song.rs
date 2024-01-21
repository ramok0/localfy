use std::path::PathBuf;
use tidal_rs::model::Track;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Hash, PartialEq)]
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