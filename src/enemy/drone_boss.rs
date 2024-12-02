use super::AI;
use crate::components::health::{HealthComponent, ShipBundle};
use crate::gameplay::loot::{DropsLoot, WorthPoints};
use crate::gameplay::physics::BaseRotation;
use crate::ship::engine::Engine;
use crate::ship::turret::{DoesDamage, FireRate, Range, TurretBundle, TurretClass};
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn spawn_drone_boss(commands: &mut Commands) {
    commands
        .spawn((
            ShipBundle {
                engine: Engine::new(8.0, 8.0),
                health: HealthComponent::new(10, 40, 3.0),
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
            // Custom short range blast laser
            parent.spawn(TurretBundle {
                class: TurretClass::BlastLaser,
                range: Range { max: 150.0 },
                fire_rate: FireRate::from_rate_in_seconds(1.0),
                damage: DoesDamage::from_amount(1),
                ..Default::default()
            });
        });
}
