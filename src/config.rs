use bevy::prelude::*;
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct GameConfig {
    pub online: bool,
    pub bgm_volume: f32,
    pub sfx_volume: f32,
    pub player_name: String,
    pub language: String,
    pub fullscreen: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            online: false,
            bgm_volume: 0.3,
            sfx_volume: 0.6,
            player_name: "".to_string(),
            language: "English".to_string(),
            fullscreen: false,
        }
    }
}

impl GameConfig {
    pub fn set_lang(&mut self, lang: &str) {
        self.language = lang.to_string();
    }
}

#[allow(dead_code)]
fn startup(mut pkv: ResMut<PkvStore>, mut config: ResMut<GameConfig>) {
    if let Ok(v) = pkv.get::<String>("config") {
        if let Ok(deserialized) = serde_json::from_str(v.as_str()) {
            *config = deserialized;
        }
    } else if let Ok(serialized) = serde_json::to_string(&config.into_inner()) {
        if let Err(err) = pkv.set::<String>("config", &serialized) {
            warn!("Failed to save config: {}", err);
        }
    } else {
        warn!("Failed to serialize config");
    }
}

#[allow(dead_code)]
fn on_change(mut pkv: ResMut<PkvStore>, config: Res<GameConfig>) {
    if config.is_changed() {
        if let Ok(serialized) = serde_json::to_string(&config.into_inner()) {
            if let Err(err) = pkv.set::<String>("config", &serialized) {
                warn!("Failed to save config: {}", err);
            }
        } else {
            warn!("Failed to serialize config");
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(GameConfig::default())
        .insert_resource(PkvStore::new("CloudGames", "skywalker2088"));
    //#[cfg(any(not(debug_assertions), target_arch = "wasm32", feature = "save"))]
    app.add_systems(Startup, startup);
    //#[cfg(any(not(debug_assertions), target_arch = "wasm32", feature = "save"))]
    app.add_systems(Update, on_change);
}
