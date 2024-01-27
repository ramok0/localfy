use std::{sync::{atomic::AtomicBool, Arc}, hash::{Hash, Hasher}, collections::hash_map::DefaultHasher, time::Instant};

use egui::{ ColorImage, TextureOptions, TextureId };
use tidal_rs::model::{ self, Album, Artist, DeviceAuth, SearchResult, SearchType, Track };

use crate::cache::CacheManager;
use crate::song::Song;
use super::page::RenderablePage;

#[derive(PartialEq)]
pub enum Event {
    SearchResult(SearchResult),
    SongArray(Vec<Song>),
    DeviceCode(Option<DeviceAuth>),
    LogonWithTidal
}

pub enum Pages {
    Home,
    Search,
    Downloads,
    Settings
}

type EventManager = (tokio::sync::mpsc::Sender<Event>, tokio::sync::mpsc::Receiver<Event>);

pub enum UserLocation {
    Home,
    Playlist(Box<dyn RenderablePage>),
    Artist(Box<dyn RenderablePage>),
    Album(Box<dyn RenderablePage>)
}

pub struct GuiInput {
    pub search_query: String,
    pub event_manager: EventManager,
    pub search_results: SearchResult,
    pub search_type: SearchType,
    pub requested_song_array:bool,
    pub page:Pages,
    pub last_songs_update:Option<Instant>,
    pub is_searching:bool,
    pub location:UserLocation,
    pub should_restart:bool,
    pub is_logging_in:bool,
    pub device_code:Option<DeviceAuth>
}

impl Default for GuiInput {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            event_manager: tokio::sync::mpsc::channel(20),
            search_results: SearchResult::new(),
            search_type: SearchType::Track,
            requested_song_array: false,
            page: Pages::Home,
            last_songs_update: None,
            is_searching: false,
            location: UserLocation::Home,
            should_restart: false,
            is_logging_in: false,
            device_code: None
        }
    }
}