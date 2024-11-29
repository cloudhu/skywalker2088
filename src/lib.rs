pub mod audio;
mod components;
mod config;

#[cfg(feature = "dev")]
mod dev_tools;
mod enemy;
mod gameplay;

mod animation;
mod arena;
pub mod assets;
pub mod camera;
mod collision;
mod loot;
mod options;
mod player;
mod run;
mod screens;
mod ship;
pub mod spawnable;
mod stats;
mod theme;
mod ui;
mod util;
pub mod weapon;

use crate::animation::SpriteAnimationPlugin;
use crate::arena::ArenaPlugin;
use crate::camera::CameraPlugin;
use crate::loot::LootPlugin;
use crate::options::display::DisplayConfig;
use crate::options::generate_config_files;
use crate::player::PlayerPlugin;
use crate::run::RunPlugin;
use crate::spawnable::SpawnablePlugin;
use crate::ui::UiPlugin;
use crate::weapon::WeaponPlugin;
use bevy::window::WindowMode;
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_kira_audio::{AudioPlugin, AudioSettings};
use bevy_parallax::ParallaxPlugin;
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};

/// Used by a physics engine to translate physics calculations to graphics
const PHYSICS_PIXELS_PER_METER: f32 = 10.0;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // pushes rust errors to the browser console
        #[cfg(target_arch = "wasm32")]
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        #[cfg(not(target_arch = "wasm32"))]
        generate_config_files();

        let display_config = get_display_config();

        // Order new `AppStep` variants by adding them here:
        app.configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        )
        .insert_resource(AudioSettings {
            sound_capacity: 8192,
            command_capacity: 4096,
        });

        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Skywalker2088".to_string(),
                        canvas: Some("#bevy".to_string()),
                        resolution: (display_config.width, display_config.height).into(),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        resizable: true,
                        mode: if display_config.fullscreen {
                            WindowMode::SizedFullscreen
                        } else {
                            WindowMode::Windowed
                        },
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(Color::srgb(0.04, 0.005, 0.04)))
        .add_plugins(ShapePlugin)
        .add_plugins(ParallaxPlugin)
        .add_plugins(AudioPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(ArenaPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(SpawnablePlugin)
        .add_plugins(RunPlugin)
        .add_plugins(LootPlugin)
        .add_plugins(SpriteAnimationPlugin)
        .add_plugins(WeaponPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(collision::CollisionPlugin)
        .add_plugins(
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PHYSICS_PIXELS_PER_METER)
                .in_fixed_schedule(),
        );

        // Add other plugins.
        app.add_plugins((
            options::plugin,
            config::plugin,
            assets::plugin,
            stats::plugin,
            screens::plugin,
            theme::plugin,
            gameplay::plugin,
            ship::plugin,
            enemy::plugin,
            audio::plugin,
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
        #[cfg(feature = "dev")]
        app.add_plugins(RapierDebugRenderPlugin::default());
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

#[cfg(not(target_arch = "wasm32"))]
fn get_display_config() -> DisplayConfig {
    use ron::de::from_str;
    use std::{env::current_dir, fs::read_to_string};

    let config_path = current_dir().unwrap().join("config");

    from_str::<DisplayConfig>(&read_to_string(config_path.join("display.ron")).unwrap()).unwrap()
}

#[cfg(target_arch = "wasm32")]
fn get_display_config() -> DisplayConfig {
    DisplayConfig {
        width: 1280.0,
        height: 1024.0,
        fullscreen: false,
    }
}
