use crate::components::health::Seeker;
use crate::gameplay::gamelogic::game_not_paused;
use crate::gameplay::physics::Physics;
use crate::screens::AppStates;
use crate::AppSet;
use bevy::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
pub struct Engine {
    pub target: Option<Vec2>,
    pub method: EngineMethod,
    pub power: f32,
    pub speed: f32,
    pub max_speed: f32,
    pub depower_factor: f32,
    pub steering_factor: f32,
}

impl Default for Engine {
    fn default() -> Self {
        Engine {
            target: None,
            method: EngineMethod::Approach,
            power: 10.0,
            speed: 0.0,
            max_speed: 10.0,
            depower_factor: 5.0,
            steering_factor: 20.0,
        }
    }
}

impl Engine {
    pub fn new(power: f32, max_speed: f32) -> Engine {
        Engine {
            target: None,
            method: EngineMethod::Approach,
            power,
            speed: 0.0,
            max_speed,
            depower_factor: 5.0,
            steering_factor: 20.0,
        }
    }

    pub fn new_with_steering(power: f32, max_speed: f32, steering_factor: f32) -> Engine {
        Engine {
            target: None,
            method: EngineMethod::Approach,
            power,
            speed: 0.0,
            max_speed,
            depower_factor: 5.0,
            steering_factor,
        }
    }
}

pub enum EngineMethod {
    Approach,
    Keep(f32),
    #[allow(dead_code)]
    Orbit(f32),
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (engine_system, seeker_system)
            .chain()
            .in_set(AppSet::Update)
            .distributive_run_if(game_not_paused)
            .distributive_run_if(in_state(AppStates::Game)),
    );
}

pub fn engine_system(
    time: Res<Time>,
    mut query: Query<
        (&Transform, &mut Physics, &mut Engine),
        (With<Transform>, With<Physics>, With<Engine>),
    >,
) {
    for (transform, mut physics, mut engine) in &mut query {
        let current = transform.translation.truncate();
        if let Some(target) = engine.target {
            engine.speed += engine.power * time.delta_seconds();
            if engine.speed > engine.max_speed {
                engine.speed = engine.max_speed;
            }
            let to_target = match engine.method {
                EngineMethod::Approach => approach(current, target),
                EngineMethod::Keep(distance) => keep_at_distance(current, target, distance),
                EngineMethod::Orbit(distance) => orbit(current, target, distance),
            };
            // Can only steer so many degrees per second
            let max_steer_this_step = time.delta_seconds() * PI * engine.steering_factor;
            let mut desired_steer = to_target.angle_between(physics.velocity);
            if desired_steer.is_nan() {
                // When 0 velocity
                desired_steer = 0.0;
            }
            let clamped_steer = desired_steer.clamp(-max_steer_this_step, max_steer_this_step);
            let to_target = Vec2::from_angle(clamped_steer).rotate(to_target);
            // println!("to target: {:?}", to_target);
            physics.add_force(to_target.normalize() * engine.speed);
        } else {
            engine.speed -= engine.power * time.delta_seconds() * engine.depower_factor;
            if engine.speed < 0.0 {
                engine.speed = 0.0
            }
        }
    }
}

fn approach(current: Vec2, target: Vec2) -> Vec2 {
    target - current
}

fn keep_at_distance(current: Vec2, target: Vec2, distance: f32) -> Vec2 {
    let new_target = target + (current - target).normalize() * distance;
    approach(current, new_target)
}

fn orbit(current: Vec2, target: Vec2, distance: f32) -> Vec2 {
    const ORBIT_TOLERANCE: f32 = 20.0;
    let distance_and_tolerance = distance + ORBIT_TOLERANCE;
    let distance_from_centre = current.distance(target);
    let towards_target = approach(current, target);
    if (distance_from_centre - distance_and_tolerance).abs() > ORBIT_TOLERANCE {
        keep_at_distance(current, target, distance)
    } else {
        // Circle around
        let tangential = Quat::from_rotation_z(PI / 2.0).mul_vec3(towards_target.extend(0.0));
        let new_target = current + tangential.truncate();
        approach(current, new_target)
    }
}

pub fn seeker_system(
    mut commands: Commands,
    mut query: Query<(&Seeker, &mut Engine), (With<Seeker>, With<Engine>)>,
    target_query: Query<&Transform, With<Transform>>,
) {
    for (seeker, mut engine) in &mut query {
        if commands.get_entity(seeker.0).is_some() {
            if let Ok(target) = target_query.get(seeker.0) {
                engine.target = Some(target.translation.truncate());
            }
        }
    }
}
