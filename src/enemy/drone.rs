use std::f32::consts::PI;

use super::AI;
use crate::components::health::{HealthComponent, ShipBundle};
use crate::gameplay::gamelogic::ExplodesOnDespawn;
use crate::gameplay::loot::{DropsLoot, WorthPoints};
use crate::gameplay::physics::BaseRotation;
use crate::ship::engine::Engine;
use crate::ship::turret::{DoesDamage, FireRate, Range, TurretBundle, TurretClass};
use bevy::prelude::*;

pub fn spawn_drone(commands: &mut Commands) {
    commands
        .spawn((
            ShipBundle {
                engine: Engine::new(10.0, 10.0),
                health: HealthComponent::new(1, 4, 3.0),
                explodes_on_despawn: ExplodesOnDespawn {
                    size_min: 15.0,
                    size_max: 20.0,
                    ..Default::default()
                },
                ..Default::default()
            },
            BaseRotation {
                rotation: Quat::from_rotation_z(-PI),
            },
            AI,
            DropsLoot,
            WorthPoints { value: 10 },
        ))
        .with_children(|parent| {
            // Custom short range blast laser
            parent.spawn(TurretBundle {
                class: TurretClass::BlastLaser,
                range: Range { max: 100.0 },
                fire_rate: FireRate::from_rate_in_seconds(2.0),
                damage: DoesDamage::from_amount(1),
                ..Default::default()
            });
        });
}
