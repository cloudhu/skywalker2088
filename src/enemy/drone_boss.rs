use std::f32::consts::PI;

use bevy::prelude::*;

use super::AI;
use crate::components::common::Health;
use crate::gameplay::loot::{DropsLoot, WorthPoints};
use crate::gameplay::physics::{BaseGlyphRotation, Collider, Physics};
use crate::ship::engine::Engine;
use crate::ship::platform::{Fonts, ShipBundle};
use crate::ship::turret::{DoesDamage, FireRate, Range, TurretBundle, TurretClass};
use crate::util::Colour;

pub fn spawn_drone_boss(commands: &mut Commands, fonts: &Res<Fonts>, position: Vec3) {
    commands
        .spawn((
            ShipBundle {
                glyph: Text2dBundle {
                    text: Text::from_section(
                        "冞",
                        TextStyle {
                            font: fonts.primary.clone(),
                            font_size: 32.0,
                            color: Colour::ENEMY,
                        },
                    )
                    .with_justify(JustifyText::Center),
                    transform: Transform::from_translation(position),
                    ..default()
                },
                physics: Physics::new(8.0),
                engine: Engine::new(8.0, 8.0),
                health: Health::new(10, 40),
                collider: Collider { radius: 30.0 },
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
