use std::f32::consts::PI;

use bevy::prelude::*;

use super::AI;
use crate::assets::Fonts;
use crate::components::common::{Health, ShipBundle};
use crate::gameplay::gamelogic::ExplodesOnDespawn;
use crate::gameplay::loot::{DropsLoot, WorthPoints};
use crate::gameplay::physics::{BaseGlyphRotation, Collider, Physics};
use crate::ship::engine::Engine;
use crate::ship::turret::{DoesDamage, FireRate, Range, TurretBundle, TurretClass};
use crate::util::Colour;

pub fn spawn_drone(commands: &mut Commands, fonts: &Res<Fonts>, position: Vec3) {
    commands
        .spawn((
            ShipBundle {
                glyph: Text2dBundle {
                    text: Text::from_section(
                        "米",
                        TextStyle {
                            font: fonts.primary.clone(),
                            font_size: 18.0,
                            color: Colour::ENEMY,
                        },
                    )
                    .with_justify(JustifyText::Center),
                    transform: Transform::from_translation(position),
                    ..default()
                },
                physics: Physics::new(5.0),
                engine: Engine::new(10.0, 10.0),
                health: Health::new(1, 4),
                collider: Collider { radius: 10.0 },
                explodes_on_despawn: ExplodesOnDespawn {
                    size_min: 15.0,
                    size_max: 20.0,
                    ..Default::default()
                },
                ..Default::default()
            },
            BaseGlyphRotation {
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
