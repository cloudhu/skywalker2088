use crate::gameplay::gamelogic::game_not_paused;
use crate::screens::AppStates;
use crate::util::Math;
use crate::AppSet;
use bevy::prelude::*;

#[derive(Component)]
pub struct BaseGlyphRotation {
    pub rotation: Quat,
}

#[derive(Component, Default)]
pub struct Collider {
    pub radius: f32,
}

#[derive(Component)]
pub struct Physics {
    pub acceleration: Vec2,
    pub velocity: Vec2,
    pub drag: f32,
    pub face_velocity: bool,
}

impl Physics {
    pub fn new(drag: f32) -> Physics {
        Physics {
            drag,
            ..Default::default()
        }
    }

    pub fn add_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }
}

impl Default for Physics {
    fn default() -> Self {
        Self {
            acceleration: Vec2::ZERO,
            velocity: Vec2::ZERO,
            drag: 0.0,
            face_velocity: true,
        }
    }
}

#[derive(Component)]
pub struct Rotator {
    pub speed: f32,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (physics_system, rotator_system)
            .chain()
            .in_set(AppSet::Update)
            .distributive_run_if(game_not_paused)
            .distributive_run_if(in_state(AppStates::Game)),
    );
}

pub fn physics_system(
    time: Res<Time>,
    mut query: Query<
        (&mut Transform, &mut Physics, Option<&BaseGlyphRotation>),
        (With<Transform>, With<Physics>),
    >,
) {
    for (mut transform, mut physics, base_rotation) in &mut query {
        // Not sure how to avoid cloning here
        let current_acceleration = physics.acceleration;
        let drag = physics.drag;
        physics.velocity += current_acceleration;
        transform.translation += physics.velocity.extend(0.0) * time.delta_seconds();
        // println!("Player translate at {:?}", transform.translation);
        // TODO make acceleration ramp down
        physics.acceleration = Vec2::ZERO;
        physics.velocity *= 1.0 - (drag * time.delta_seconds());

        if physics.face_velocity {
            transform.rotation = Math::quaternion_from_2d_vector(physics.velocity);
        }

        if let Some(base_rotation) = base_rotation {
            transform.rotation *= base_rotation.rotation; // Multiplication is like combining rotations together
        }
    }
}

pub fn rotator_system(time: Res<Time>, mut query: Query<(&mut Transform, &Rotator)>) {
    for (mut transform, rotator) in &mut query {
        transform.rotate(Quat::from_rotation_z(rotator.speed * time.delta_seconds()));
    }
}
