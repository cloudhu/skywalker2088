//! Exposes a plugin and CLI to configure a small number of settings for the game.
use bevy::{
    app::{App, Startup},
    core_pipeline::{core_2d::Camera2d, tonemapping::Tonemapping},
    log::error,
    prelude::*,
    render::camera::Camera,
    state::state::OnEnter,
};
use leafwing_input_manager::prelude::InputManagerPlugin;

pub(super) mod display;
mod input;
pub mod resources;

use self::display::set_window_icon;
use crate::assets::game_assets::AppStates;
use crate::components::input::{InputsResource, MenuAction};
use crate::options::resources::GameParametersResource;
use input::get_input_bindings;
use ron::de::from_bytes;
use serde::Deserialize;
use std::default::Default;
use std::env::current_dir;
use std::fs::{DirBuilder, File};
use std::io::Write;
use std::path::PathBuf;

use self::input::spawn_menu_explorer_system;

pub const DEFAULT_OPTIONS_PROFILE_ID: usize = 0;

/// The 'model' of the Options Sqlite table.
/// Defaults the least graphically intense options.
#[derive(Debug, Default, Clone, Deserialize, Resource)]
pub struct GameOptions {
    pub bloom_enabled: bool,
    pub bloom_intensity: f32,
    pub tutorials_enabled: bool,
}
#[cfg_attr(all(not(target_arch = "wasm32")), derive(argh::FromArgs))]
#[derive(Default, Debug, PartialEq, Eq)]
/// Options used to start Thetawave. As many of these as possible are inferred/have "sensible"
/// defaults.
pub struct GameInitCLIOptions {
    #[cfg_attr(all(not(target_arch = "wasm32")), argh(option))]
    /// the directory that is used for `bevy::asset::AssetPlugin`. This is generally
    /// 'EXECUTABLE_DIR/assets/' or 'CARGO_MANIFEST_DIR/assets'.
    pub assets_dir: Option<PathBuf>,
    #[cfg_attr(all(not(target_arch = "wasm32")), argh(switch, short = 'a'))]
    /// whether to use instructions, serial port IO, etc. specific to deploying on an arcade
    /// machine. This should almost never be enabled.
    pub arcade: bool,
}
impl GameInitCLIOptions {
    pub fn from_environ_on_supported_platforms_with_default_fallback() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            argh::from_env()
        }
    }
}

fn apply_game_options_system(
    mut game_options: ResMut<GameOptions>,
    mut camera_2d_query: Query<
        (&mut Camera, &mut Tonemapping),
        (With<Camera2d>, Without<Camera3d>),
    >,
) {
    if let (Ok((mut camera_2d, mut tonemapping_2d)),) = (camera_2d_query.get_single_mut(),) {
        camera_2d.hdr = game_options.bloom_enabled;

        if game_options.bloom_enabled {
            *tonemapping_2d = Tonemapping::TonyMcMapface;
        } else {
            *tonemapping_2d = Tonemapping::None;
            game_options.bloom_intensity = 0.0;
        }
    } else {
        error!("Failed to get singleton 2d and 3d cameras to apply game opts");
    }
}

/// Includes systems to change configuration settings about the game.
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<MenuAction>::default());

    app.insert_resource(InputsResource::from(get_input_bindings()));
    app.insert_resource(GameOptions::default());
    app.insert_resource(
        from_bytes::<GameParametersResource>(include_bytes!(
            "../../assets/data/game_parameters.ron"
        ))
        .unwrap(),
    );
    app.add_systems(Startup, spawn_menu_explorer_system);

    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Startup, set_window_icon);

    app.add_systems(OnEnter(AppStates::MainMenu), apply_game_options_system);
}

/// Creates config file in config directory from config file in this directory
macro_rules! confgen {
    ( $($filename:expr),* ) => {
        {
            let conf_dir = current_dir().unwrap().join("config");
            if !conf_dir.is_dir() {
                DirBuilder::new()
                    .create(conf_dir.clone())
                    .expect("Confgen failed: could not create config dir.");
            }

            $({
                let default = include_bytes!($filename);
                let file_path = conf_dir.join($filename);
                let mut file = File::create(file_path)
                    .expect(concat!("Confgen failed: could not create config file ", $filename, "."));
                file.write_all(default)
                    .expect(concat!("Confgen failed: could not write config file ", $filename, "."));
            })*
        }
    }
}

/// Generates the display config file
pub(super) fn generate_config_files() {
    confgen!("display.ron");
    confgen!("input.ron");
}

#[cfg(all(test, not(target_arch = "wasm32"), feature = "dev_native"))]
mod cli_tests {
    use argh::FromArgs;
    #[test]
    fn test_cli_parse_asset_path_dir() {
        assert_eq!(
            super::GameInitCLIOptions::from_args(
                &["skywalker2088"],
                &["--assets-dir", "myassets/"]
            )
            .unwrap()
            .assets_dir,
            Some(std::path::PathBuf::from("myassets/"))
        );
    }
}
