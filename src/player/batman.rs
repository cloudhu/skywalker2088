use crate::asset_tracking::LoadResource;
use crate::components::health::Health;
use crate::gameplay::animation::{AnimationIndices, AnimationTimer};
use crate::gameplay::gamelogic::{
    game_not_paused, Allegiance, PlayerLevel, Targettable, WillTarget,
};
use crate::gameplay::physics::{BaseGlyphRotation, Collider, Physics};
use crate::gameplay::GameState;
use crate::screens::AppState;
use crate::ship::engine::Engine;
use crate::ship::platform::{DespawnWithScene, ExplodesOnDespawn, HitFlash};
use crate::util::RenderLayer;
use crate::{AppSet, CameraShake, MainCamera};
use bevy::app::App;
use bevy::ecs::system::RunSystemOnce;
use bevy::ecs::world::Command;
use bevy::prelude::*;
use bevy::render::texture::{ImageLoaderSettings, ImageSampler};
use std::f32::consts::PI;

#[derive(Component, Default)]
pub struct Cargo {
    pub amount: u32,
    pub bonus_chance: f32,
}

#[derive(Component)]
pub struct Magnet {
    pub range: f32,
    pub strength: f32,
}

impl Magnet {
    pub fn default() -> Magnet {
        Magnet {
            range: 500.0,
            strength: 5.0,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<IsPlayer>();
    app.load_resource::<PlayerAssets>();
    app.add_systems(
        Update,
        (pause_control, zoom_control)
            .chain()
            .in_set(AppSet::Update)
            .run_if(in_state(AppState::Gameplay)),
    );
    app.add_systems(
        Update,
        (player_control, level_up_system)
            .chain()
            .in_set(AppSet::Update)
            .distributive_run_if(game_not_paused)
            .distributive_run_if(in_state(AppState::Gameplay)),
    );
}

/// A command to spawn the player character.
#[derive(Debug)]
pub struct SpawnPlayer {
    /// 可配置的飞船参数.
    pub max_speed: f32,
    pub drag: f32,
    pub power: f32,
    pub steering_factor: f32,
    pub max_health: i32,
    pub max_shield: i32,
    pub radius: f32,
}

impl Default for SpawnPlayer {
    fn default() -> Self {
        SpawnPlayer::new(16.0, 5.0, 8.0, 10.0, 100, 100, 10.0)
    }
}

impl SpawnPlayer {
    pub fn new(
        max_speed: f32,
        drag: f32,
        power: f32,
        steering_factor: f32,
        max_health: i32,
        max_shield: i32,
        radius: f32,
    ) -> SpawnPlayer {
        SpawnPlayer {
            max_speed,
            drag,
            power,
            steering_factor,
            max_health,
            max_shield,
            radius,
        }
    }
}

impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_player);
    }
}

// Bundles
#[derive(Bundle, Default)]
pub struct ShipBundle {
    pub physics: Physics,
    pub engine: Engine,
    pub health: Health,
    pub collider: Collider,
    pub targettable: Targettable,
    pub will_target: WillTarget,
    pub despawn_with_scene: DespawnWithScene,
    pub explodes_on_despawn: ExplodesOnDespawn,
    pub hit_flash: HitFlash,
}

// Simple components
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct IsPlayer;

#[derive(Resource, Asset, Reflect, Clone)]
pub struct PlayerAssets {
    // This #[dependency] attribute marks the field as a dependency of the Asset.
    // This means that it will not finish loading until the labeled asset is also loaded.
    #[dependency]
    pub ufo: Handle<Image>,
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
}

impl PlayerAssets {
    pub const PATH_UFO: &'static str = "images/ufo.png";
    pub const PATH_STEP_1: &'static str = "audio/sound_effects/step1.ogg";
    pub const PATH_STEP_2: &'static str = "audio/sound_effects/step2.ogg";
    pub const PATH_STEP_3: &'static str = "audio/sound_effects/step3.ogg";
    pub const PATH_STEP_4: &'static str = "audio/sound_effects/step4.ogg";
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            ufo: assets.load_with_settings(
                PlayerAssets::PATH_UFO,
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve the pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            steps: vec![
                assets.load(PlayerAssets::PATH_STEP_1),
                assets.load(PlayerAssets::PATH_STEP_2),
                assets.load(PlayerAssets::PATH_STEP_3),
                assets.load(PlayerAssets::PATH_STEP_4),
            ],
        }
    }
}

// Spawn the player
fn spawn_player(
    In(config): In<SpawnPlayer>,
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // A texture atlas is a way to split one image with a grid into multiple
    // sprites. By attaching it to a [`SpriteBundle`] and providing an index, we
    // can specify which section of the image we want to see. We will use this
    // to animate our player character. You can learn more about texture atlases in
    // this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs Some(UVec2::splat(1))
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(91), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 3 };

    commands.spawn((
        Name::new("Player"),
        ShipBundle {
            physics: Physics::new(config.drag),
            engine: Engine::new_with_steering(
                config.power,
                config.max_speed,
                config.steering_factor,
            ),
            health: Health::new(config.max_health, config.max_shield),
            collider: Collider {
                radius: config.radius,
            },
            targettable: Targettable(Allegiance::PLAYER),
            will_target: WillTarget(vec![Allegiance::ENEMY]),
            ..default()
        },
        SpriteBundle {
            texture: player_assets.ufo.clone(),
            transform: Transform::from_translation(Vec3 {
                x: 90.0,
                y: 90.0,
                z: RenderLayer::Player.as_z(),
            }),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        BaseGlyphRotation {
            rotation: Quat::from_rotation_z(PI / 2.0),
        },
        IsPlayer,
        Cargo::default(),
        Magnet::default(),
        StateScoped(AppState::Gameplay),
    ));
    println!("spawning Player");
}

pub fn player_control(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut query: Query<(&IsPlayer, &mut Engine), (With<IsPlayer>, With<Engine>)>,
) {
    for (_, mut engine) in &mut query {
        if mouse_button_input.pressed(MouseButton::Left) {
            // Calculate current position to mouse position
            let (camera, camera_transform) = camera_q.single();
            let window = windows.get_single().expect("no primary window");

            engine.target = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin.truncate());
            // println!("Player controlled at {:?}", engine.target);
        } else {
            engine.target = None;
        }
    }
}

pub fn pause_control(
    key_input: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut change_game_state: ResMut<NextState<GameState>>,
    mut query: Query<&mut CameraShake>,
) {
    if key_input.just_pressed(KeyCode::Escape) {
        match game_state.get() {
            GameState::Running => change_game_state.set(GameState::Paused),
            GameState::Paused => change_game_state.set(GameState::Running),
            _ => (),
        }
    }

    // Debug camera shake
    if key_input.just_pressed(KeyCode::KeyR) {
        for mut shake in &mut query {
            shake.trauma = 5.0;
        }
    }
}

pub fn level_up_system(
    mut level: ResMut<PlayerLevel>,
    mut query: Query<&mut Cargo, With<IsPlayer>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for mut cargo in &mut query {
        if cargo.amount >= level.required_cargo_to_level() {
            cargo.amount -= level.required_cargo_to_level();
            level.value += 1;
            // next_state.set(GameState::Selection);
        }
    }
}

pub fn zoom_control(
    key_input: Res<ButtonInput<KeyCode>>,
    mut camera_q: Query<
        &mut OrthographicProjection,
        (With<OrthographicProjection>, With<MainCamera>),
    >,
) {
    let scale_factor = 0.25;

    if key_input.just_pressed(KeyCode::NumpadAdd) {
        if let Ok(mut projection) = camera_q.get_single_mut() {
            projection.scale = (projection.scale - scale_factor).max(1.);
        }
    }

    if key_input.just_pressed(KeyCode::NumpadSubtract) {
        if let Ok(mut projection) = camera_q.get_single_mut() {
            projection.scale = (projection.scale + scale_factor).min(3.);
        }
    }
}
