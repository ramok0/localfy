use std::{sync::Arc, hash::{Hash, Hasher}, collections::hash_map::DefaultHasher, time::Instant};

use egui::{ ColorImage, TextureOptions, TextureId };
use tidal_rs::model::{ self, Track, Album, Artist, SearchResult, SearchType };

use crate::{database::Song, cache::CacheManager};

#[derive(Clone, PartialEq)]
pub enum Event {
    SearchResult(DrawableSearchResult),
    SongArray(DrawableSongArray)
}

pub enum Pages {
    Home,
    Search,
    Downloads,
    Settings
}

type EventManager = (tokio::sync::mpsc::Sender<Event>, tokio::sync::mpsc::Receiver<Event>);

pub trait Drawable<T> {
    async fn get_texture_from_url(
        item: &T,
        ctx: &egui::Context,
        cache_manager:Arc<tokio::sync::Mutex<CacheManager>>
    ) -> (egui::TextureHandle, egui::TextureId) {
        let cover = Self::fetch_cover(&item, cache_manager.clone()).await.unwrap_or(include_bytes!("../../assets/placeholder.jpeg").to_vec());
        let raw_image = load_image_from_memory(&cover).unwrap();
        let texture = ctx.load_texture(
            Self::get_item_name(&item),
            raw_image,
            TextureOptions::default()
        );
        let texture_id = TextureId::from(&texture);

        (texture, texture_id)
    }

    async fn fetch_cover(item: &T, cache_manager:Arc<tokio::sync::Mutex<CacheManager>>) -> Result<Vec<u8>, tidal_rs::error::Error> {
        if let Some(cache_id) = Self::get_draw_item_id(item) {
            let mut cache_manager = cache_manager.lock().await;
            
            if let Some(cached_item) = cache_manager.get(cache_id.clone()) {
                println!("Found cover using cache");
                return Ok(cached_item.data.clone());
            }

            let url = Self::get_url(item);
            let bytes = reqwest
                ::get(&url).await
                .map_err(|e| tidal_rs::error::Error::Reqwest(e))?
                .bytes().await
                .map_err(|_| tidal_rs::error::Error::ParseError)?;
            
            let data = bytes.to_vec();

            //TODO : Add error handling
            let _ = cache_manager.add(cache_id, data.clone()).map_err(|_| tidal_rs::error::Error::ParseError);

            return Ok(data);
        }

        return Err(tidal_rs::error::Error::NotFound);
    }

    fn get_item(&self) -> T;
    fn get_item_name(item: &T) -> String;
    fn get_url(item: &T) -> String;

    //gets the id of the item that will be drawn (ex cover)
    fn get_draw_item_id(item: &T) -> Option<String>;

    fn get_title(&self) -> String {
        Self::get_item_name(&self.get_item())
    }

    fn get_texture(&self) -> &egui::TextureHandle;
    fn get_texture_id(&self) -> egui::TextureId;
}

#[derive(Clone, PartialEq)]
pub struct DrawableTrack {
    track: Track,
    texture: egui::TextureHandle,
    texture_id: egui::TextureId,
}

#[derive(Clone, PartialEq)]
pub struct DrawableAlbum {
    album: model::Album,
    texture: egui::TextureHandle,
    texture_id: egui::TextureId,
}

#[derive(Clone, PartialEq)]
pub struct DrawableArtist {
    pub artist: model::Artist,
    texture: egui::TextureHandle,
    texture_id: egui::TextureId,
}

impl Drawable<Track> for DrawableTrack {
    fn get_item(&self) -> Track {
        self.track.clone()
    }

    fn get_draw_item_id(item: &Track) -> Option<String> {
        item.album.as_ref().and_then(|x| Some(x.cover.clone()))
    }

    fn get_texture_id(&self) -> egui::TextureId {
        self.texture_id
    }

    fn get_item_name(item: &Track) -> String {
        item.title.clone()
    }

    fn get_url(item: &Track) -> String {
        if item.album.is_some() {
            format!(
                "{}/{}/80x80.jpg",
                "https://resources.tidal.com/images",
                item.album.as_ref().unwrap().cover.replace("-", "/")
            )
        } else {
            "https://media.discordapp.net/attachments/1146883003402952797/1196115483477413928/281202_folder_512x512.png?ex=65b6745a&is=65a3ff5a&hm=e6121be910cad9e16938bb69d6a214a3a1c40e525af647ef1cf1efa2146d805a&=&format=webp&quality=lossless&width=80&height=80".to_string()
        }
    }

    fn get_texture(&self) -> &egui::TextureHandle {
        &self.texture
    }

    fn get_title(&self) -> String {
        format!("{} - {}", self.track.get_artist().name, self.track.title)
    }
}

impl Drawable<Album> for DrawableAlbum {
    fn get_item(&self) -> Album {
        self.album.clone()
    }

    fn get_draw_item_id(item: &Album) -> Option<String> {
        Some(item.cover.clone())
    }

    fn get_texture_id(&self) -> egui::TextureId {
        self.texture_id
    }

    fn get_item_name(item: &Album) -> String {
        item.title.clone()
    }

    fn get_url(item: &Album) -> String {
        format!(
            "{}/{}/80x80.jpg",
            "https://resources.tidal.com/images",
            item.cover.replace("-", "/")
        )
    }

    fn get_texture(&self) -> &egui::TextureHandle {
        &self.texture
    }
}

impl Drawable<Artist> for DrawableArtist {
    fn get_item(&self) -> Artist {
        self.artist.clone()
    }

    fn get_draw_item_id(item: &Artist) -> Option<String> {
        item.picture.clone()
    }

    fn get_texture_id(&self) -> egui::TextureId {
        self.texture_id
    }

    fn get_item_name(item: &Artist) -> String {
        item.name.clone()
    }

    fn get_url(item: &Artist) -> String {
        if item.picture.is_some() {
            format!(
                "{}/{}/320x320.jpg",
                "https://resources.tidal.com/images",
                item.picture.as_ref().unwrap().replace("-", "/")
            )
        } else {
            "https://media.discordapp.net/attachments/1146883003402952797/1196115483477413928/281202_folder_512x512.png?ex=65b6745a&is=65a3ff5a&hm=e6121be910cad9e16938bb69d6a214a3a1c40e525af647ef1cf1efa2146d805a&=&format=webp&quality=lossless&width=80&height=80".to_string()
        }
    }

    fn get_texture(&self) -> &egui::TextureHandle {
        &self.texture
    }
}

impl DrawableTrack {
    pub async fn from_track(ctx: &egui::Context, track: Track, cache_manager:Arc<tokio::sync::Mutex<CacheManager>>) -> Self {
        let (texture, texture_id) = Self::get_texture_from_url(&track, ctx, cache_manager).await;
        Self {
            track,
            texture,
            texture_id,
        }
    }
}

impl DrawableAlbum {
    pub async fn from_album(ctx: &egui::Context, album: Album, cache_manager:Arc<tokio::sync::Mutex<CacheManager>>) -> Self {
        let (texture, texture_id) = Self::get_texture_from_url(&album, ctx, cache_manager).await;
        Self {
            album,
            texture,
            texture_id,
        }
    }
}

impl DrawableArtist {
    pub async fn from_artist(ctx: &egui::Context, artist: model::Artist, cache_manager:Arc<tokio::sync::Mutex<CacheManager>>) -> Self {
        let (texture, texture_id) = Self::get_texture_from_url(&artist, ctx, cache_manager).await;
        Self {
            artist,
            texture,
            texture_id,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct DrawableSearchResult {
    pub track: Vec<DrawableTrack>,
    pub album: Vec<DrawableAlbum>,
    pub artist: Vec<DrawableArtist>,
}

impl DrawableSearchResult {
    pub fn new() -> Self {
        Self {
            track: Vec::new(),
            album: Vec::new(),
            artist: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.track.is_empty() && self.album.is_empty() && self.artist.is_empty()
    }

    pub async fn from_search_result(ctx: &egui::Context, item: &SearchResult, cache_manager:Arc<tokio::sync::Mutex<CacheManager>>) -> Self {
        let mut track: Vec<DrawableTrack> = Vec::with_capacity(item.tracks.len());
        let mut album: Vec<DrawableAlbum> = Vec::with_capacity(item.albums.len());
        let mut artist: Vec<DrawableArtist> = Vec::with_capacity(item.artists.len());
        for track_item in item.tracks.iter() { 
            track.push(DrawableTrack::from_track(ctx, track_item.clone(), cache_manager.clone()).await);
        }

        for album_item in item.albums.iter() {
            album.push(DrawableAlbum::from_album(ctx, album_item.clone(), cache_manager.clone()).await);
        }

        for artist_item in item.artists.iter() {
            artist.push(DrawableArtist::from_artist(ctx, artist_item.clone(), cache_manager.clone()).await);
        }

        Self {
            track,
            album,
            artist,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct DrawableSong {
    pub song: Song,
    texture: egui::TextureHandle,
    texture_id: egui::TextureId,
}

impl Drawable<Song> for DrawableSong {
    fn get_item(&self) -> Song {
        self.song.clone()
    }

    fn get_draw_item_id(item: &Song) -> Option<String> {
        item.tidal_track.as_ref().and_then(|x| Some(x.album.as_ref().unwrap().cover.clone()))
    }

    fn get_texture_id(&self) -> egui::TextureId {
        self.texture_id
    }

    fn get_item_name(item: &Song) -> String {
        item.title.clone()
    }

    fn get_url(item: &Song) -> String {
        if item.tidal_track.is_some() {
            DrawableTrack::get_url(&item.tidal_track.as_ref().unwrap())
        } else {
            "https://media.discordapp.net/attachments/1146883003402952797/1196115483477413928/281202_folder_512x512.png?ex=65b6745a&is=65a3ff5a&hm=e6121be910cad9e16938bb69d6a214a3a1c40e525af647ef1cf1efa2146d805a&=&format=webp&quality=lossless&width=80&height=80".to_string()
        }
    }

    fn get_texture(&self) -> &egui::TextureHandle {
        &self.texture
    }
    

}

impl DrawableSong {
    pub async fn from_song(ctx: &egui::Context, song: Song, cache_manager:Arc<tokio::sync::Mutex<CacheManager>>) -> Self {
        let (texture, texture_id) = Self::get_texture_from_url(&song, ctx, cache_manager).await;
        Self {
            song,
            texture,
            texture_id,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct DrawableSongArray {
   pub songs: Vec<DrawableSong>,
   pub hash: u64,
   pub created_at: Instant
}

impl DrawableSongArray {
    pub fn new() -> Self {
        Self {
            songs: Vec::new(),
            hash: 0,
            created_at: Instant::now()
        }
    }

    pub async fn from_song_array(ctx: &egui::Context, songs: Vec<Song>, cache_manager:Arc<tokio::sync::Mutex<CacheManager>>) -> Self {
        let mut drawable_songs = Vec::with_capacity(songs.len());

        let mut hasher = DefaultHasher::new();
        songs.hash(&mut hasher);

        for song in songs {
            drawable_songs.push(DrawableSong::from_song(ctx, song, cache_manager.clone()).await);
        }

        Self {
            songs: drawable_songs,
            hash: hasher.finish(),
            created_at: Instant::now()
        }
    }
}

pub struct GuiInput {
    pub search_query: String,
    pub event_manager: EventManager,
    pub search_results: DrawableSearchResult,
    pub search_type: SearchType,
    pub song_array: DrawableSongArray,
    pub requested_song_array:bool,
    pub page:Pages
}

impl Default for GuiInput {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            event_manager: tokio::sync::mpsc::channel(20),
            search_results: DrawableSearchResult::new(),
            search_type: SearchType::Track,
            song_array: DrawableSongArray::new(),
            requested_song_array: false,
            page: Pages::Home
        }
    }
}

fn load_image_from_memory(image_data: &[u8]) -> Result<ColorImage, image::ImageError> {
    let image = image::load_from_memory(image_data)?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()))
}