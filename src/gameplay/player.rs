use crate::{
    assets::Fonts,
    components::{common::Health, common::ShipBundle},
    gameplay::{
        gamelogic::{game_not_paused, Allegiance, PlayerLevel, Targettable, WillTarget},
        loot::{Cargo, Magnet},
        physics::{BaseGlyphRotation, Collider, Physics},
        GameState,
    },
    screens::AppState,
    ship::engine::Engine,
    util::{Colour, RenderLayer},
    AppSet, CameraShake, MainCamera,
};
use bevy::input::mouse::MouseWheel;
use bevy::window::WindowMode;
use bevy::{
    app::App,
    ecs::{system::RunSystemOnce, world::Command},
    prelude::*,
};
use std::f32::consts::PI;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<IsPlayer>();

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
            .run_if(in_state(AppState::InGame)),
    );
    app.add_systems(
        Update,
        (player_control, level_up_system)
            .chain()
            .in_set(AppSet::Update)
            .distributive_run_if(game_not_paused)
            .distributive_run_if(in_state(AppState::InGame)),
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

// Simple components
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct IsPlayer;

// Spawn the player
fn spawn_player(In(config): In<SpawnPlayer>, mut commands: Commands, player_assets: Res<Fonts>) {
    commands.spawn((
        Name::new("Player"),
        ShipBundle {
            glyph: Text2dBundle {
                text: Text::from_section(
                    "中",
                    TextStyle {
                        font: player_assets.primary.clone(),
                        font_size: 20.0,
                        color: Colour::PLAYER,
                    },
                )
                .with_justify(JustifyText::Center),
                transform: Transform::from_translation(Vec3 {
                    x: 100.0,
                    y: 100.0,
                    z: RenderLayer::Player.as_z(),
                }),
                ..default()
            },
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
            targettable: Targettable(Allegiance::Friend),
            will_target: WillTarget(vec![Allegiance::Enemy]),
            ..default()
        },
        BaseGlyphRotation {
            rotation: Quat::from_rotation_z(PI / 2.0),
        },
        IsPlayer,
        Cargo::default(),
        Magnet::default(),
        StateScoped(AppState::InGame),
    ));
    // println!("spawning Player");
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
    if key_input.just_pressed(KeyCode::Space) {
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
            next_state.set(GameState::Selection);
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

fn toggle_fullscreen(mut window_query: Query<&mut Window>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::F11) {
        let mut window = window_query.single_mut();
        window.mode = match window.mode {
            WindowMode::Windowed => WindowMode::SizedFullscreen,
            WindowMode::BorderlessFullscreen => WindowMode::Windowed,
            WindowMode::SizedFullscreen => WindowMode::Windowed,
            WindowMode::Fullscreen => WindowMode::Windowed,
        };
    }
}
