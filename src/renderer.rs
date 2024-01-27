use std::hash::{Hash, Hasher};

use egui::ImageSource;
use tidal_rs::model::{Album, Artist, Track};

use crate::{cache::CacheManager, song::Song};

pub trait Drawable {
    fn id(&self) -> usize;

    fn get_title(&self) -> String;

    fn get_texture(&self) -> ImageSource;

    fn get_track(&self) -> Option<Track> {
        None
    }

    fn get_album(&self) -> Option<Album> {
        None
    }
}


impl Song {
    pub fn get_artist(&self) -> String {
        self.artist.clone()
    }
}


impl Drawable for Track {
    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_track(&self) -> Option<Track> {
        Some(self.clone())
    }

    fn get_texture(&self) -> ImageSource {
        let cover_id = self.album.as_ref().and_then(|album| Some(album.cover.clone()));

        if let Some(cover_id) = cover_id {
            let url = format!("https://resources.tidal.com/images/{}/80x80.jpg",cover_id.replace("-", "/"));
            return ImageSource::Uri(url.into());
        }

        CacheManager::get_default_cover()
    }

    fn id(&self) -> usize {
        self.id
    }
}

impl Drawable for Song {
    fn id(&self) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish() as usize
    }

    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_texture(&self) -> ImageSource {
        let cover_id = self.tidal_track.as_ref().and_then(|track| track.album.as_ref().and_then(|album| Some(album.cover.clone())));

        if let Some(cover_id) = cover_id {
            let url = format!("https://resources.tidal.com/images/{}/80x80.jpg",cover_id.replace("-", "/"));
            return ImageSource::Uri(url.into());
        }

        CacheManager::get_default_cover()
    }

    fn get_track(&self) -> Option<Track> {
        self.tidal_track.clone()
    }


}

impl Drawable for Artist {
    fn get_title(&self) -> String {
        self.name.clone()
    }

    fn id(&self) -> usize {
        self.id
    }

    fn get_texture(&self) -> ImageSource {
        let cover_id = self.picture.clone();

        if let Some(cover_id) = cover_id {
            let url = format!("https://resources.tidal.com/images/{}/320x320.jpg",cover_id.replace("-", "/"));
            return ImageSource::Uri(url.into());
        }

        CacheManager::get_default_cover()
    }
}

impl Drawable for Album {
    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_texture(&self) -> ImageSource {
        let cover_id = self.cover.clone();

            let url = format!("https://resources.tidal.com/images/{}/80x80.jpg",cover_id.replace("-", "/"));
            return ImageSource::Uri(url.into());
    }

    fn id(&self) -> usize {
        self.id
    }

    fn get_album(&self) -> Option<Album> {
            Some(self.clone())
    }
}