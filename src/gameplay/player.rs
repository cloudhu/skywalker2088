use crate::assets::player_assets::PlayerAssets;
use crate::components::character::CharacterType;
use crate::{
    components::health::{Health, Spacecraft},
    gameplay::{
        gamelogic::{game_not_paused, Allegiance, PlayerLevel, Targettable, WillTarget},
        loot::{Cargo, Magnet},
        physics::{BaseRotation, Collider, Physics},
        GameStates,
    },
    screens::AppStates,
    ship::engine::Engine,
    util::RenderLayer,
    AppSet, CameraShake, MainCamera,
};
use bevy::input::mouse::MouseWheel;
use bevy::window::{PrimaryWindow, WindowMode};
use bevy::{
    app::App,
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
};
use std::f32::consts::PI;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlayerComponent>();

    app.add_systems(
        Update,
        (
            pause_control,
            zoom_control,
            handle_mouse_wheel_input,
            toggle_fullscreen,
        )
            .chain()
            .in_set(AppSet::Update)
            .run_if(in_state(AppStates::Game)),
    );
    app.add_systems(
        Update,
        (player_control, level_up_system)
            .chain()
            .in_set(AppSet::Update)
            .distributive_run_if(game_not_paused)
            .distributive_run_if(in_state(AppStates::Game)),
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
    pub max_health: usize,
    pub max_shield: usize,
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
        max_health: usize,
        max_shield: usize,
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
        world
            .run_system_once_with(self, spawn_player)
            .expect("Can not spawn player");
    }
}

// Simple components
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerComponent;

// Spawn the player
fn spawn_player(
    In(config): In<SpawnPlayer>,
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
) {
    commands
        .spawn((
            Spacecraft,
            Sprite::from_image(player_assets.get_asset(&CharacterType::Captain)),
            Transform::from_translation(Vec3 {
                x: 100.0,
                y: 100.0,
                z: RenderLayer::Player.as_z(),
            }),
            Physics::new(config.drag),
            Engine::new_with_steering(config.power, config.max_speed, config.steering_factor),
            Health::new(config.max_health, config.max_shield, 2.0),
            Collider {
                radius: config.radius,
            },
            Targettable(Allegiance::Friend),
            WillTarget(vec![Allegiance::Enemy]),
            BaseRotation {
                rotation: Quat::from_rotation_z(-PI / 2.0),
            },
            Name::new("Player"),
            PlayerComponent,
            Cargo::default(),
            Magnet::default(),
        ))
        .insert(StateScoped(AppStates::Game));
    info!("Player spawned");
}

pub fn player_control(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Transform, &mut Engine), (With<PlayerComponent>, With<Engine>)>,
    primary_window: Single<&Window, With<PrimaryWindow>>,
    q_camera: Single<(&Camera, &GlobalTransform)>,
) {
    for (trans, mut engine) in query.iter_mut() {
        // Collect directional input.
        let mut intent = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            intent.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            intent.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            intent.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            intent.x += 1.0;
        }

        if mouse_button_input.pressed(MouseButton::Left) {
            let (main_camera, main_camera_transform) = *q_camera;
            let world_cursor_pos = primary_window.cursor_position().and_then(|cursor_pos| {
                main_camera
                    .viewport_to_world_2d(main_camera_transform, cursor_pos)
                    .ok()
            });
            if world_cursor_pos.is_some() {
                engine.target = world_cursor_pos;
                // info!("Player controlled at {:?}", engine.target);
            }
        } else if intent != Vec2::ZERO {
            let player_pos = trans.translation.clone();
            engine.target =
                Option::from(Vec2::new(player_pos.x + intent.x, player_pos.y + intent.y));
            // info!("Player moved to  {:?}", engine.target);
        } else {
            engine.target = None;
        }
    }
}

pub fn pause_control(
    key_input: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameStates>>,
    mut change_game_state: ResMut<NextState<GameStates>>,
    mut query: Query<&mut CameraShake>,
) {
    if key_input.just_pressed(KeyCode::Space) {
        match game_state.get() {
            GameStates::Playing => change_game_state.set(GameStates::Paused),
            GameStates::Paused => change_game_state.set(GameStates::Playing),
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
    mut query: Query<&mut Cargo, With<PlayerComponent>>,
    mut next_state: ResMut<NextState<GameStates>>,
) {
    for mut cargo in &mut query {
        if cargo.amount >= level.required_cargo_to_level() {
            cargo.amount -= level.required_cargo_to_level();
            level.value += 1;
            next_state.set(GameStates::Selection);
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

    if key_input.just_pressed(KeyCode::PageUp) {
        if let Ok(mut projection) = camera_q.get_single_mut() {
            projection.scale = (projection.scale - scale_factor).max(1.);
        }
    }

    if key_input.just_pressed(KeyCode::PageDown) {
        if let Ok(mut projection) = camera_q.get_single_mut() {
            projection.scale = (projection.scale + scale_factor).min(3.);
        }
    }
}

/// 处理鼠标滚轮事件
fn handle_mouse_wheel_input(
    mut mouse_wheel_input: EventReader<MouseWheel>,
    mut camera_q: Query<
        &mut OrthographicProjection,
        (With<OrthographicProjection>, With<MainCamera>),
    >,
) {
    for event in mouse_wheel_input.read() {
        if let Ok(mut projection) = camera_q.get_single_mut() {
            projection.scale = (projection.scale + event.y).clamp(1., 3.);
        }
    }
}

fn toggle_fullscreen(mut window: Single<&mut Window>, keys: Res<ButtonInput<KeyCode>>) {
    if !window.focused {
        return;
    }

    if keys.just_pressed(KeyCode::F11) {
        window.mode = match window.mode {
            WindowMode::Windowed => WindowMode::SizedFullscreen(MonitorSelection::Current),
            WindowMode::BorderlessFullscreen(MonitorSelection::Current) => WindowMode::Windowed,
            WindowMode::SizedFullscreen(MonitorSelection::Current) => WindowMode::Windowed,
            WindowMode::Fullscreen(MonitorSelection::Current) => WindowMode::Windowed,
            _ => WindowMode::Windowed,
        };
    }
}
