use std::{collections::hash_map::DefaultHasher, fs::File, hash::{Hash, Hasher}, io::Read, path::PathBuf, sync::Arc};
use std::io::Write;
use egui::{ahash::{HashMap, HashMapExt}, load::Bytes, ImageSource};
use image::EncodableLayout;

pub struct CachedObject {
    pub data:Vec<u8>,
    pub path:PathBuf
}

impl CachedObject {
    pub fn new(data:Vec<u8>, path:PathBuf) -> Self {
        CachedObject {
            data,
            path
        }
    }
}

pub struct CacheManager {
    items: HashMap<u64, CachedObject>   
}

impl CacheManager {
    pub fn new() -> Self {
        let mut result = CacheManager {
            items: HashMap::new()
        };

        let base_path = Self::get_base_path();
        println!("Base path : {:?}", base_path);
        if base_path.exists() {
            if let Ok(entries) = std::fs::read_dir(base_path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if let Ok(hash) = path.file_name().unwrap().to_str().unwrap().parse::<u64>() {
                            let data = std::fs::read(&path).unwrap();

                            result.items.insert(hash, CachedObject::new(data, path));
                        }
                    }
                }
            }
        }

        result
    }

    pub fn source_from_id<'a>(id:String) -> Option<ImageSource<'a>>
    {
        let id = id.parse::<u64>().ok()?;
        let path = Self::get_path_for_hash(id);

        dbg!(&path);

        if path.exists() {
            let data = std::fs::read(&path).unwrap();

            Some(ImageSource::Bytes { uri: format!("bytes://{id}").into(), bytes: Bytes::Shared(Arc::from(data.as_bytes())) })
        } else {
            None
        }
    }

    pub fn get_default_cover() -> ImageSource<'static> {
        ImageSource::Bytes { uri: "bytes://defaultcover.svg".into(), bytes: Bytes::Static(include_bytes!("../assets/missing.svg")) }
    }

    fn get_base_path() -> PathBuf {
        if let Ok(program_data) = std::env::var("PROGRAMDATA") {
            let path_buf = PathBuf::from(program_data)
                .join("Localfy")
                .join("cache");

            if !path_buf.exists() {
                std::fs::create_dir_all(&path_buf).unwrap();
            }

            return path_buf;
        } else {
            PathBuf::from("")
        }
    }

    fn get_path_for_hash(hash:u64) -> PathBuf {
        Self::get_base_path().join(hash.to_string())
    }

    pub fn add(&mut self, key:impl Hash, data:Vec<u8>) -> std::io::Result<()> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        let hash = hasher.finish();
        let object = self.create_cache_file(hash, data);
        self.items.insert(hash, object?);

        Ok(())
    }

    pub fn create_cache_file(&mut self, key:u64, data:Vec<u8>) -> std::io::Result<CachedObject> {
        let path = Self::get_path_for_hash(key);
        let mut file = File::create(&path)?;

        file.write_all(&data)?;

        Ok(CachedObject::new(data, path))
    }

    pub fn get(&self, key:impl Hash) -> Option<&CachedObject> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);


        self.items.get(&hasher.finish())
    }
}