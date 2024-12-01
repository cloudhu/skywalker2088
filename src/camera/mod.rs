//! Exposes a plugin that sets up the 2D/3D perspective/camera and shakes the camera when an event
//! is emitted.
use self::screen_shake::{
    add_trauma_system, screen_shake_on_player_damage_system, screen_shake_system,
};
use crate::components::events::ScreenShakeEvent;
use crate::components::states::AppStates;
use crate::options::resources::GameParametersResource;
use crate::util::RenderLayer;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::core_pipeline::bloom::BloomPrefilterSettings;
use bevy::core_pipeline::core_2d::{Camera2d, Camera2dBundle};
use bevy::core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping};
use bevy::ecs::system::{Commands, Res};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{in_state, EventWriter, IntoSystemConfigs, IsDefaultUiCamera, UVec2};
use bevy::render::camera::{Camera, ClearColorConfig};
use bevy::transform::components::Transform;
use bevy::utils::default;
use bevy_parallax::{CreateParallaxEvent, LayerData, LayerSpeed, ParallaxCameraComponent};
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::geometry::GeometryBuilder;
use bevy_prototype_lyon::shapes;

mod screen_shake;

pub(super) struct CameraPlugin;

/// Sets up a 2d perspective/camera of the 3d world. When this plugin is enabled, one can send
/// `thetawave_interface::camera::ScreenShakeEvent` to jolt the screen, for example, when a player
/// takes damage.
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScreenShakeEvent>();
        app.add_systems(Startup, setup_cameras_system);
        app.add_systems(
            Update,
            (
                screen_shake_system,
                add_trauma_system,
                screen_shake_on_player_damage_system,
            )
                .run_if(in_state(AppStates::Game)),
        );
    }
}

/// Setup 2D camera for sprites used in gameplay
fn setup_cameras_system(
    mut commands: Commands,
    game_parameters: Res<GameParametersResource>,
    mut create_parallax: EventWriter<CreateParallaxEvent>,
) {
    // setup cameras
    // 2d camera for sprites
    let camera_2d = Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, game_parameters.camera_z),
        camera_2d: Camera2d,
        camera: Camera {
            order: 1,
            hdr: true, // 1. HDR is required for bloom
            clear_color: ClearColorConfig::None,
            ..default()
        },
        tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
        ..default()
    };
    // Spawn the Camera
    let camera = commands
        .spawn((
            camera_2d,
            BloomSettings {
                // 3. Enable bloom for the camera
                prefilter_settings: BloomPrefilterSettings {
                    threshold: 1.0,
                    threshold_softness: 0.2,
                },
                ..BloomSettings::OLD_SCHOOL
            },
            screen_shake::ScreenShakeComponent {
                trauma: 0.0,
                trauma_decay: 1.,
                shake_intensity: Vec3 {
                    x: 60.,
                    y: 60.,
                    z: 0.1,
                },
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
