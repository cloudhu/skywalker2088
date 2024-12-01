use crate::components::states::AppStates;
use crate::gameplay::gamelogic::game_not_paused;
use crate::gameplay::physics::{Collider, Physics};
use crate::AppSet;
use bevy::app::{App, Update};
use bevy::prelude::*;
use rand::Rng;
use std::fmt;

#[derive(Resource)]
pub struct Points {
    pub value: u32,
}

impl fmt::Display for Points {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Component)]
pub struct WorthPoints {
    pub value: u32,
}

#[derive(Component)]
pub struct IsLoot;

#[derive(Component)]
pub struct DropsLoot;

#[derive(Component, Default)]
pub struct Cargo {
    pub amount: usize,
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
    app.add_systems(
        Update,
        (loot_magnet_system, loot_cargo_collision)
            .chain()
            .in_set(AppSet::Update)
            .distributive_run_if(game_not_paused)
            .distributive_run_if(in_state(AppStates::Game)),
    );
}

pub fn loot_magnet_system(
    query: Query<(&Magnet, &Transform), (With<Magnet>, With<Transform>)>,
    mut loot_query: Query<
        (&mut Physics, &Transform),
        (
            With<IsLoot>,
            With<Physics>,
            With<Transform>,
            Without<Magnet>,
        ),
    >,
) {
    for (magnet, transform) in &query {
        for (mut physics, loot_transform) in &mut loot_query {
            if loot_transform
                .translation
                .truncate()
                .distance(transform.translation.truncate())
                > magnet.range
            {
                continue;
            }
            let direction = (transform.translation.truncate()
                - loot_transform.translation.truncate())
            .normalize_or_zero();
            physics.add_force(direction * magnet.strength);
        }
    }
}

pub fn loot_cargo_collision(
    mut commands: Commands,
    mut query: Query<
        (&mut Cargo, &Transform, &Collider),
        (With<Cargo>, With<Transform>, With<Collider>),
    >,
    loot_query: Query<
        (&Transform, Entity, &Collider, Option<&WorthPoints>),
        (
            With<IsLoot>,
            With<Transform>,
            With<Collider>,
            Without<Cargo>,
        ),
    >,
    mut points: ResMut<Points>,
) {
    for (mut cargo, transform, collider) in &mut query {
        for (loot_transform, loot_entity, loot_collider, worth_points) in &loot_query {
            if loot_transform
                .translation
                .truncate()
                .distance(transform.translation.truncate())
                <= loot_collider.radius + collider.radius
            {
                // Increase cargo
                cargo.amount += 1;
                if rand::thread_rng().gen_range(0.0..1.0) < cargo.bonus_chance {
                    cargo.amount += 2;
                }

                // Add points
                if let Some(worth_points) = worth_points {
                    points.value += worth_points.value;
                }

                // Despawn
                if let Some(mut subcommand) = commands.get_entity(loot_entity) {
                    subcommand.despawn(); // Direct despawn because adding ShouldDespawn has issues
                }
            }
        }
    }
}
