//! Application paths. Made by KebiLab

use directories::ProjectDirs;
use std::path::PathBuf;

pub struct AppPaths {
    pub root: PathBuf,
    pub config: PathBuf,
    pub profiles_dir: PathBuf,
    pub logs_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub models_dir: PathBuf,
    pub assets_dir: PathBuf,
}

impl AppPaths {
    pub fn new() -> Self {
        let dirs = ProjectDirs::from("lab", "KebiLab", "KebiControl")
            .expect("ProjectDirs");
        let root = dirs.config_dir().to_path_buf();
        let cache_root = dirs.cache_dir().to_path_buf();

        let profiles_dir = root.join("profiles");
        let logs_dir = cache_root.join("logs");
        let models_dir = cache_root.join("models");
        let cache_dir = cache_root.join("cache");
        let assets_dir = root.join("assets");
        let config = root.join("config.toml");

        for p in [&root, &profiles_dir, &logs_dir, &models_dir, &cache_dir, &assets_dir] {
            let _ = std::fs::create_dir_all(p);
        }

        Self { root, config, profiles_dir, logs_dir, cache_dir, models_dir, assets_dir }
    }

    pub fn log_file(&self) -> PathBuf {
        self.logs_dir.join("kebicontrol.log")
    }
}

impl Default for AppPaths {
    fn default() -> Self {
        Self::new()
    }
}
