//! User profile. Made by KebiLab

use crate::command::Aliases;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    #[serde(default)]
    pub aliases: Aliases,
    #[serde(default)]
    pub apps: AppBookmarks,
    /// Custom phrases mapped to a "kind" tag (e.g. "кеби стоп" -> "cancel").
    #[serde(default)]
    pub custom_phrases: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub tts_voice: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppBookmarks {
    /// Friendly name -> executable path
    pub items: std::collections::HashMap<String, String>,
}

impl Default for Profile {
    fn default() -> Self {
        let mut apps = AppBookmarks::default();
        apps.items.insert("браузер".into(), "msedge".into());
        apps.items.insert("дискорд".into(), "Discord".into());
        apps.items.insert("стим".into(), "steam".into());
        apps.items.insert("проводник".into(), "explorer".into());
        apps.items.insert("терминал".into(), "wt".into());
        apps.items.insert("настройки".into(), "ms-settings:".into());
        apps.items.insert("калькулятор".into(), "calc".into());
        apps.items.insert("блокнот".into(), "notepad".into());

        let mut aliases = Aliases::default();
        aliases.0.insert("ютуб".into(), "https://youtube.com".into());
        aliases.0.insert("гугл".into(), "https://google.com".into());
        aliases.0.insert("гитхаб".into(), "https://github.com".into());
        aliases.0.insert("редидит".into(), "https://reddit.com".into());
        aliases.0.insert("твиттер".into(), "https://x.com".into());

        Self {
            name: "default".into(),
            aliases,
            apps,
            custom_phrases: Default::default(),
            tts_voice: None,
        }
    }
}

impl Profile {
    pub fn load_from(path: &Path) -> Result<Self> {
        let s = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&s)?)
    }

    pub fn save_to(&self, path: &Path) -> Result<()> {
        let s = toml::to_string_pretty(self)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, s)?;
        Ok(())
    }
}
