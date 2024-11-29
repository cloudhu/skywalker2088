use crate::components::player::PlayerComponent;
use crate::{
    gameplay::{
        gamelogic::{game_not_paused, PlayerLevel},
        loot::Cargo,
        GameStates,
    },
    screens::AppStates,
    ship::engine::Engine,
    AppSet,
};
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::WindowMode;

pub(super) fn plugin(app: &mut App) {
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

pub fn player_control(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut query: Query<(&PlayerComponent, &mut Engine), (With<PlayerComponent>, With<Engine>)>,
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
    game_state: Res<State<GameStates>>,
    mut change_game_state: ResMut<NextState<GameStates>>,
) {
    if key_input.just_pressed(KeyCode::Space) {
        match game_state.get() {
            GameStates::Playing => change_game_state.set(GameStates::Paused),
            GameStates::Paused => change_game_state.set(GameStates::Playing),
            _ => (),
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
        (With<OrthographicProjection>, With<Camera2d>),
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
        (With<OrthographicProjection>, With<Camera2d>),
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
