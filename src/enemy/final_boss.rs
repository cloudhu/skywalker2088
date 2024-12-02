use super::{FinalBoss, AI};
use crate::{
    components::health::{HealthComponent, ShipBundle},
    gameplay::physics::BaseRotation,
    ship::{
        engine::{Engine, EngineMethod},
        turret::{DoesDamage, EffectSize, FireRate, Range, TurretBundle, TurretClass},
    },
};
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn spawn_final_boss(commands: &mut Commands) {
    commands
        .spawn((
            ShipBundle {
                engine: Engine {
                    power: 40.0,
                    max_speed: 80.0,
                    method: EngineMethod::Keep(200.0),
                    ..Default::default()
                },
                health: HealthComponent::new(1000, 4000, 3.0),
                ..Default::default()
            },
            BaseRotation {
                rotation: Quat::from_rotation_z(-PI),
            },
            AI,
            FinalBoss,
        ))
        .with_children(|parent| {
            // Custom OP weapon
            parent.spawn(TurretBundle {
                class: TurretClass::PierceLaser,
                range: Range { max: 300.0 },
                fire_rate: FireRate::from_rate_in_seconds(3.0),
                damage: DoesDamage::from_amount(5),
                size: EffectSize(3.0),
                ..Default::default()
            });
        });
}
