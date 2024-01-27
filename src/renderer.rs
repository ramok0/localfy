use egui::ImageSource;
use tidal_rs::model::{Album, Artist, Track};

use crate::{cache::CacheManager, song::Song};

pub trait Drawable {
    fn get_title(&self) -> String;

    fn get_texture(&self) -> ImageSource;
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

    fn get_texture(&self) -> ImageSource {
        let cover_id = self.album.as_ref().and_then(|album| Some(album.cover.clone()));

        if let Some(cover_id) = cover_id {
            let url = format!("https://resources.tidal.com/images/{}/80x80.jpg",cover_id.replace("-", "/"));
            return ImageSource::Uri(url.into());
        }

        CacheManager::get_default_cover()
    }
}

impl Drawable for Song {
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


}

impl Drawable for Artist {
    fn get_title(&self) -> String {
        self.name.clone()
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

}