use std::f32::consts::PI;

use bevy::prelude::*;

use super::{FinalBoss, AI};
use crate::components::common::Health;
use crate::gameplay::physics::{BaseGlyphRotation, Collider, Physics};
use crate::ship::engine::{Engine, EngineMethod};
use crate::ship::platform::{Fonts, ShipBundle};
use crate::ship::turret::{DoesDamage, EffectSize, FireRate, Range, TurretBundle, TurretClass};
use crate::util::Colour;

pub fn spawn_final_boss(commands: &mut Commands, fonts: &Res<Fonts>, position: Vec3) {
    commands
        .spawn((
            ShipBundle {
                glyph: Text2dBundle {
                    text: Text::from_section(
                        "Œ",
                        TextStyle {
                            font: fonts.primary.clone(),
                            font_size: 50.0,
                            color: Colour::ENEMY,
                        },
                    )
                    .with_justify(JustifyText::Center),
                    transform: Transform::from_translation(position),
                    ..default()
                },
                physics: Physics::new(8.0),
                engine: Engine {
                    power: 40.0,
                    max_speed: 80.0,
                    method: EngineMethod::Keep(200.0),
                    ..Default::default()
                },
                health: Health::new(1000, 4000),
                collider: Collider { radius: 50.0 },
                ..Default::default()
            },
            BaseGlyphRotation {
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
