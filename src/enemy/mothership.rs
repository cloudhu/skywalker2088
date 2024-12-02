use super::AI;
use crate::gameplay::gamelogic::ExplodesOnDespawn;
use crate::gameplay::loot::{DropsLoot, WorthPoints};
use crate::ship::turret::MultiShot;
use crate::{
    components::health::{HealthComponent, ShipBundle},
    gameplay::physics::BaseRotation,
    ship::{
        engine::{Engine, EngineMethod},
        turret::{DoesDamage, FireRate, Range, TurretBundle, TurretClass},
    },
};
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn spawn_mothership(commands: &mut Commands) {
    commands
        .spawn((
            ShipBundle {
                engine: Engine {
                    max_speed: 3.0,
                    power: 3.0,
                    method: EngineMethod::Keep(500.0),
                    ..Default::default()
                },
                health: HealthComponent::new(100, 80, 3.0),
                explodes_on_despawn: ExplodesOnDespawn {
                    size_min: 55.0,
                    size_max: 65.0,
                    ..Default::default()
                },
                ..Default::default()
            },
            BaseRotation {
                rotation: Quat::from_rotation_z(-PI),
            },
            AI,
            DropsLoot,
            WorthPoints { value: 50 },
        ))
        .with_children(|parent| {
            // Custom rocket launcher
            parent.spawn(TurretBundle {
                class: TurretClass::RocketLauncher,
                range: Range { max: 1000.0 },
                fire_rate: FireRate::from_rate_in_seconds(0.2),
                damage: DoesDamage::from_amount(5),
                shots: MultiShot { amount: 8 },
                ..Default::default()
            });
        });
}
