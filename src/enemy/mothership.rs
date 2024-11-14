use super::AI;
use crate::gameplay::gamelogic::ExplodesOnDespawn;
use crate::gameplay::loot::{DropsLoot, WorthPoints};
use crate::ship::turret::MultiShot;
use crate::{
    assets::Fonts,
    components::common::{Health, ShipBundle},
    gameplay::physics::{BaseGlyphRotation, Collider, Physics},
    ship::{
        engine::{Engine, EngineMethod},
        turret::{DoesDamage, FireRate, Range, TurretBundle, TurretClass},
    },
    util::Colour,
};
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn spawn_mothership(commands: &mut Commands, fonts: &Res<Fonts>, position: Vec3) {
    commands
        .spawn((
            ShipBundle {
                glyph: Text2dBundle {
                    text: Text::from_section(
                        "王",
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
