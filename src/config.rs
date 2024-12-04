use crate::components::input::{InputsResource, MainMenuExplorer, MenuAction, PlayerAction};
use bevy::asset::ron::from_str;
use bevy::prelude::*;
use bevy_pkv::PkvStore;
use leafwing_input_manager::prelude::{ActionState, InputManagerPlugin, InputMap};
use leafwing_input_manager::InputManagerBundle;
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
fn startup_config(mut pkv: ResMut<PkvStore>, mut config: ResMut<GameConfig>) {
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
    app.add_plugins(InputManagerPlugin::<MenuAction>::default());

    app.insert_resource(InputsResource::from(get_input_bindings()));
    //#[cfg(any(not(debug_assertions), target_arch = "wasm32", feature = "save"))]
    app.add_systems(Startup, startup_config);
    app.add_systems(Startup, spawn_menu_explorer_system);
    //#[cfg(any(not(debug_assertions), target_arch = "wasm32", feature = "save"))]
    app.add_systems(Update, on_change);
}

/// Spawns entity to track navigation over menus
pub fn spawn_menu_explorer_system(mut commands: Commands, inputs_res: Res<InputsResource>) {
    commands
        .spawn(InputManagerBundle::<MenuAction> {
            action_state: ActionState::default(),
            input_map: inputs_res.menu.clone(),
        })
        .insert(MainMenuExplorer);
}

#[derive(Deserialize)]
pub struct InputBindings {
    pub menu_keyboard: Vec<(MenuAction, KeyCode)>,
    pub menu_gamepad: Vec<(MenuAction, GamepadButtonType)>,
    pub player_keyboard: Vec<(PlayerAction, KeyCode)>,
    pub player_gamepad: Vec<(PlayerAction, GamepadButtonType)>,
    pub player_mouse: Vec<(PlayerAction, MouseButton)>,
}

impl From<InputBindings> for InputsResource {
    fn from(bindings: InputBindings) -> Self {
        InputsResource {
            menu: InputMap::new(bindings.menu_keyboard)
                .insert_multiple(bindings.menu_gamepad)
                .to_owned(),
            player_keyboard: InputMap::new(bindings.player_keyboard)
                .insert_multiple(bindings.player_mouse)
                .to_owned(),
            player_gamepad: InputMap::new(bindings.player_gamepad),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn get_input_bindings() -> InputBindings {
    use std::{env::current_dir, fs::read_to_string};

    let config_path = current_dir().unwrap().join("config");
    // info!("config path: {:?}", config_path);
    from_str::<InputBindings>(&read_to_string(config_path.join("input.ron")).unwrap()).unwrap()
}

#[cfg(target_arch = "wasm32")]
pub(super) fn get_input_bindings() -> InputBindings {
    use bevy::asset::ron::de::from_bytes;
    from_bytes::<InputBindings>(include_bytes!("input.ron")).unwrap()
}
