mod asset_tracking;
pub mod audio;
mod components;
// mod demo;
#[cfg(feature = "dev")]
mod dev_tools;
mod enemy;
mod gameplay;

mod screens;
mod ship;
mod theme;
mod util;

use bevy::core_pipeline::bloom::{BloomCompositeMode, BloomSettings};
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::{
    asset::AssetMetaCheck,
    audio::{AudioPlugin, Volume},
    prelude::*,
};
use bevy_parallax::{
    CreateParallaxEvent, LayerData, LayerSpeed, ParallaxCameraComponent, ParallaxPlugin,
};
use bevy_prototype_lyon::prelude::{GeometryBuilder, ShapeBundle, ShapePlugin};
use bevy_prototype_lyon::shapes;
use util::RenderLayer;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Order new `AppStep` variants by adding them here:
        app.configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        );

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
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::new(0.3),
                    },
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::srgb(0.04, 0.005, 0.04)))
        .add_plugins(ShapePlugin)
        .add_plugins(ParallaxPlugin);

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);

        // Add other plugins.
        app.add_plugins((
            asset_tracking::plugin,
            // demo::plugin,
            screens::plugin,
            theme::plugin,
            gameplay::plugin,
            ship::plugin,
            enemy::plugin,
        ));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
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
