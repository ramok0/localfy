use std::path::PathBuf;


use tidal_rs::model::AudioQuality;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Configuration {
    pub refresh_token:Option<String>,
    pub base_download_folder:Option<PathBuf>,
    pub quality: Option<AudioQuality>,
    #[serde(default)]
    pub max_concurrency: usize
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            refresh_token:None,
            base_download_folder: None,
            quality: None,
            max_concurrency: 10
        }
    }
}

impl Configuration {
    pub fn set_refresh_token(&mut self, refresh_token:String) -> () {
        self.refresh_token = Some(refresh_token);
    }

    pub fn get_refresh_token(&self) -> Option<String> {
        self.refresh_token.clone()
    }

    pub fn max_concurrency(&self) -> usize {
        self.max_concurrency
    }

    pub fn set_max_concurrency(&mut self, max_concurrency:usize) -> () {
        self.max_concurrency = max_concurrency;
    }

    pub fn get_base_download_folder(&self) -> PathBuf {
        if let Some(base_download_folder) = &self.base_download_folder {
            return base_download_folder.to_path_buf();
        } else if let Ok(program_data) = std::env::var("PROGRAMDATA") {
            let path_buf = PathBuf::from(program_data)
                .join("Localfy")
                .join("Downloads");

            if !path_buf.exists() {
                std::fs::create_dir_all(path_buf.parent().unwrap()).unwrap();
            }

            return path_buf;
        }




        PathBuf::from("Downloads")
    }

    fn get_path() -> PathBuf {
        if let Ok(program_data) = std::env::var("PROGRAMDATA") {
            let path_buf = PathBuf::from(program_data)
                .join("Localfy")
                .join("config.json");

            if !path_buf.exists() {
                std::fs::create_dir_all(path_buf.parent().unwrap()).unwrap();
            }

            return path_buf;
        }

        PathBuf::from("config.json")
    }

    pub fn new() -> Self {
        let path = Self::get_path();

        if !path.exists() {
            //créer le fichier
            let config = Configuration::default();

            config.flush();

            return config;
        } else {
            let file = std::fs::File::open(path).unwrap();
            let reader = std::io::BufReader::new(file);

            let config: Configuration = serde_json::from_reader(reader).unwrap();

            return config;
        }
    }
    pub fn flush(&self) -> () {
        let path = Configuration::get_path();

        let file = std::fs::File::create(path).unwrap();
        let writer = std::io::BufWriter::new(file);

        serde_json::to_writer(writer, self).unwrap();
    }
}

impl Drop for Configuration {
    fn drop(&mut self) {
        self.flush();
    }
}
