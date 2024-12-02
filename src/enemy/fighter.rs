use super::AI;
use crate::gameplay::gamelogic::ExplodesOnDespawn;
use crate::gameplay::loot::{DropsLoot, WorthPoints};
use crate::{
    components::health::{HealthComponent, ShipBundle},
    gameplay::physics::BaseRotation,
    ship::{
        engine::Engine,
        turret::{DoesDamage, FireRate, TurretBundle, TurretClass},
    },
};
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn spawn_fighter(commands: &mut Commands) {
    commands
        .spawn((
            ShipBundle {
                engine: Engine::new(14.0, 14.0),
                health: HealthComponent::new(10, 0, 3.0),
                explodes_on_despawn: ExplodesOnDespawn {
                    size_min: 20.0,
                    size_max: 25.0,
                    ..Default::default()
                },
                ..Default::default()
            },
            BaseRotation {
                rotation: Quat::from_rotation_z(PI / 2.0),
            },
            AI,
            DropsLoot,
            WorthPoints { value: 10 },
        ))
        .with_children(|parent| {
            parent.spawn(TurretBundle {
                class: TurretClass::AutoCannon,
                fire_rate: FireRate::from_rate_in_seconds(1.0),
                damage: DoesDamage::from_amount(2),
                ..Default::default()
            });
        });
}
