//! App state. Made by KebiLab

use kebi_core::{AppPaths, Config, Profile};
use kebi_llm::LlmClient;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    pub config: parking_lot::Mutex<Config>,
    pub profile: parking_lot::Mutex<Profile>,
    pub paths: AppPaths,
    pub llm: Arc<Mutex<LlmClient>>,
}

impl AppState {
    pub fn new(config: Config, profile: Profile, paths: AppPaths, llm: Arc<Mutex<LlmClient>>) -> Self {
        Self {
            config: parking_lot::Mutex::new(config),
            profile: parking_lot::Mutex::new(profile),
            paths,
            llm,
        }
    }
    pub fn config(&self) -> Config { self.config.lock().clone() }
    pub fn profile(&self) -> Profile { self.profile.lock().clone() }
}
