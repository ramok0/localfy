

#[derive(Clone, Debug, serde::Serialize, Hash, serde::Deserialize)]
pub struct Playlist {
    pub name:String,
    pub songs:Vec<u64> // Vec to hashes of songs to gain performance and reduce database size
}