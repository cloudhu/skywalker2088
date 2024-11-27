pub mod audio;
mod components;
mod config;

#[cfg(feature = "dev")]
mod dev_tools;
mod enemy;
mod gameplay;

pub mod assets;
mod options;
mod screens;
mod ship;
mod theme;
mod util;

use crate::options::display::DisplayConfig;
use crate::options::generate_config_files;
use crate::screens::AppStates;
use bevy::core_pipeline::bloom::{BloomCompositeMode, BloomSettings};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::window::WindowMode;
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_kira_audio::{AudioPlugin, AudioSettings};
use bevy_parallax::{
    CreateParallaxEvent, LayerData, LayerSpeed, ParallaxCameraComponent, ParallaxPlugin,
};
use bevy_prototype_lyon::prelude::{GeometryBuilder, ShapeBundle, ShapePlugin};
use bevy_prototype_lyon::shapes;
use bevy_rapier2d::plugin::{RapierConfiguration, TimestepMode};
use bevy_rapier2d::prelude::{NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};
use util::RenderLayer;

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
        .add_plugins(
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PHYSICS_PIXELS_PER_METER)
                .in_fixed_schedule(),
        );

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera).add_systems(
            OnEnter(AppStates::InGame),
            setup_physics.in_set(GameEnterSet::Initialize),
        );

        // Add other plugins.
        app.add_plugins((
            options::plugin,
            config::plugin,
            assets::plugin,
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum GameEnterSet {
    Initialize,
    BuildLevel,
    SpawnPlayer,
    BuildUi,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum GameUpdateSet {
    Enter,
    Level,
    Spawn,
    NextLevel,
    UpdateUi,
    Movement,
    Abilities,
    SetTargetBehavior, // TODO: replace with more general set
    ExecuteBehavior,
    ContactCollision,
    IntersectionCollision,
    ApplyDisconnectedBehaviors,
    ChangeState,
    Cleanup,
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

#[derive(Component)]
pub struct MainCamera;
#[derive(Component)]
pub struct CameraShake {
    pub trauma: f32,
    pub decay: f32,
}

impl Default for CameraShake {
    fn default() -> Self {
        Self {
            trauma: 0.0,
            decay: 20.0,
        }
    }
}

fn spawn_camera(mut commands: Commands, mut create_parallax: EventWriter<CreateParallaxEvent>) {
    // Spawn the Camera
    let camera = commands
        .spawn((
            Name::new("Camera"),
            Camera2dBundle {
                camera: Camera {
                    hdr: true, // 1. HDR is required for bloom
                    ..default()
                },
                tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
                ..default()
            },
            MainCamera,
            CameraShake::default(),
            BloomSettings {
                // 3. Enable bloom for the camera
                intensity: 0.15,
                composite_mode: BloomCompositeMode::Additive,
                ..default()
            },
            // Render all UI to this camera.
            // Not strictly necessary since we only use one camera,
            // but if we don't use this component, our UI will disappear as soon
            // as we add another camera. This includes indirect ways of adding cameras like using
            // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
            // for debugging. So it's good to have this here for future-proofing.
            IsDefaultUiCamera,
        ))
        .insert(ParallaxCameraComponent::default())
        .id();

    // Setup parallax
    create_parallax.send(CreateParallaxEvent {
        layers_data: vec![
            LayerData {
                speed: LayerSpeed::Bidirectional(0.95, 0.95),
                path: "background/black.png".to_string(),
                tile_size: UVec2::new(1024, 1024),
                scale: Vec2::splat(5.0),
                z: RenderLayer::Background.as_z_with_offset(-10.),
                ..default()
            },
            LayerData {
                speed: LayerSpeed::Bidirectional(0.9, 0.9),
                path: "background/stars-tile.png".to_string(),
                tile_size: UVec2::new(1024, 1024),
                z: RenderLayer::Background.as_z(),
                ..default()
            },
        ],
        camera,
    });

    // Spawn a shape so that the shape loop always runs (fixes bug with library cleaning itself up)
    commands.spawn((ShapeBundle {
        path: GeometryBuilder::build_as(&shapes::Line(Vec2::ZERO, Vec2::ZERO)),
        ..default()
    },));
}

// setup rapier
fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.timestep_mode = TimestepMode::Fixed {
        dt: 1.0 / 60.0,
        substeps: 1,
    };
    rapier_config.physics_pipeline_active = true;
    rapier_config.query_pipeline_active = true;
    rapier_config.gravity = Vec2::ZERO;
}
