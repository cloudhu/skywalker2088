use std::f32::consts::PI;

use bevy::prelude::*;

use super::AI;
use crate::components::common::Health;
use crate::gameplay::gamelogic::ExplodesOnDespawn;
use crate::gameplay::loot::{DropsLoot, WorthPoints};
use crate::gameplay::physics::{BaseGlyphRotation, Collider, Physics};
use crate::ship::engine::{Engine, EngineMethod};
use crate::ship::platform::{Fonts, ShipBundle};
use crate::ship::turret::{DoesDamage, FireRate, MultiShot, Range, TurretBundle, TurretClass};
use crate::util::Colour;

pub fn spawn_mothership(commands: &mut Commands, fonts: &Res<Fonts>, position: Vec3) {
    commands
        .spawn((
            ShipBundle {
                glyph: Text2dBundle {
                    text: Text::from_section(
                        "@",
                        TextStyle {
                            font: fonts.primary.clone(),
                            font_size: 60.0,
                            color: Colour::ENEMY,
                        },
                    )
                    .with_justify(JustifyText::Center),
                    transform: Transform::from_translation(position),
                    ..default()
                },
                physics: Physics::new(12.0),
                engine: Engine {
                    max_speed: 3.0,
                    power: 3.0,
                    method: EngineMethod::Keep(500.0),
                    ..Default::default()
                },
                health: Health::new(100, 80),
                collider: Collider { radius: 50.0 },
                explodes_on_despawn: ExplodesOnDespawn {
                    size_min: 55.0,
                    size_max: 65.0,
                    ..Default::default()
                },
                ..Default::default()
            },
            BaseGlyphRotation {
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
