use super::{FinalBoss, AI};
use crate::{
    assets::Fonts,
    components::common::{Health, ShipBundle},
    gameplay::physics::{BaseGlyphRotation, Collider, Physics},
    ship::{
        engine::{Engine, EngineMethod},
        turret::{DoesDamage, EffectSize, FireRate, Range, TurretBundle, TurretClass},
    },
    util::Colour,
};
use bevy::prelude::*;
use std::f32::consts::PI;

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
