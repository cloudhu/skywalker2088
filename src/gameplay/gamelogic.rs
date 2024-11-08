use crate::gameplay::GameState;
use crate::player::batman::IsPlayer;
use crate::screens::AppState;
use crate::ship::platform::DespawnWithScene;
use crate::util::Math;
use crate::{AppSet, CameraShake, MainCamera};
use bevy::app::App;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy_parallax::{ParallaxMoveEvent, ParallaxSystems};
use std::fmt;

#[derive(Resource, Default)]
pub struct GameTime(pub Stopwatch);

#[derive(Resource)]
pub struct Points {
    pub value: u32,
}

impl fmt::Display for Points {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Resource)]
pub struct PlayerLevel {
    pub value: u32,
}

impl PlayerLevel {
    pub fn required_cargo_to_level(&self) -> u32 {
        self.value * 4 // TODO make exponential?
    }
}

#[derive(Component, Copy, Clone)]
pub struct Damage {
    pub amount: i32,
    pub is_crit: bool,
}

#[derive(Event)]
pub struct TakeDamageEvent {
    pub entity: Entity,
    pub damage: Damage,
}

#[derive(PartialEq)]
pub enum Allegiance {
    PLAYER,
    ENEMY,
}

#[derive(Component)]
pub struct Targettable(pub Allegiance);

impl Default for Targettable {
    fn default() -> Self {
        Targettable(Allegiance::ENEMY)
    }
}

#[derive(Component)]
pub struct WillTarget(pub Vec<Allegiance>);

impl Default for WillTarget {
    fn default() -> Self {
        WillTarget(vec![Allegiance::PLAYER])
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Gameplay), setup_new_game);
    app.add_systems(OnExit(AppState::Gameplay), reset_game);
    app.add_systems(
        Update,
        (
            game_time_system,
            camera_follow.before(ParallaxSystems),
            // bullet_system,
            // bullet_collision_system,
            // combat_system,
            // laser_render_system,
            // explosion_render_system,
            // expanding_collider_system,
            // death_system,
            // loot_magnet_system,
            // loot_cargo_collision,
            // seeker_system,
        )
            .chain()
            .in_set(AppSet::TickTimers)
            .distributive_run_if(game_not_paused)
            .distributive_run_if(in_state(AppState::Gameplay)),
    );
}
fn setup_new_game(mut commands: Commands) {
    // Set the start time
    commands.insert_resource(GameTime::default());

    // Create point count
    commands.insert_resource(Points { value: 0 });

    // Start player at level 0 so they get immediate selection
    commands.insert_resource(PlayerLevel { value: 0 });
}

pub fn game_not_paused(game_state: Res<State<GameState>>) -> bool {
    *game_state.get() != GameState::Paused && *game_state.get() != GameState::Selection
}

fn game_time_system(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    game_time.0.tick(time.delta());
}

fn reset_game(
    mut commands: Commands,
    query: Query<Entity, With<DespawnWithScene>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    next_game_state.set(GameState::Running);
}

pub fn camera_follow(
    time: Res<Time>,
    player_q: Query<&Transform, (With<Transform>, With<IsPlayer>, Without<MainCamera>)>,
    mut camera_q: Query<
        (Entity, &Transform, &mut CameraShake),
        (With<Transform>, With<MainCamera>, Without<IsPlayer>),
    >,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
) {
    if let Ok((camera_entity, camera_transform, mut shake)) = camera_q.get_single_mut() {
        if let Ok(player_transform) = player_q.get_single() {
            // Calculate the new camera position based on the player's position
            let target_position = Vec2::new(
                player_transform.translation.x + 1.0,
                player_transform.translation.y,
            );

            let current_position = camera_transform.translation.truncate();

            let smooth_move_position = current_position
                .lerp(target_position, 5.0 * time.delta_seconds())
                + shake.trauma * Math::random_2d_unit_vector();

            shake.trauma = f32::max(shake.trauma - shake.decay * time.delta_seconds(), 0.0);

            move_event_writer.send(ParallaxMoveEvent {
                translation: smooth_move_position - current_position,
                rotation: 0.0,
                camera: camera_entity,
            });
        }
    }
}
