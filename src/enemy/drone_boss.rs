use super::AI;
use crate::assets::enemy_assets::MobAssets;
use crate::components::health::{FighterBundle, HealthComponent};
use crate::components::spawnable::{EnemyMobType, MobType};
use crate::gameplay::loot::{DropsLoot, WorthPoints};
use crate::gameplay::physics::{BaseRotation, Collider, Physics};
use crate::ship::animation::AnimationComponent;
use crate::ship::animation::AnimationDirection::PingPong;
use crate::ship::animation::PingPongDirection::Forward;
use crate::ship::engine::Engine;
use crate::ship::turret::{DoesDamage, FireRate, Range, TurretBundle, TurretClass};
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn spawn_drone_boss(commands: &mut Commands, mob_assets: &MobAssets, position: Vec3) {
    let mob_type = MobType::Enemy(EnemyMobType::MechaSaucetron);
    commands
        .spawn((
            FighterBundle {
                sprite: SpriteBundle {
                    texture: mob_assets.get_mob_image(&mob_type),
                    transform: Transform::from_translation(position),
                    ..default()
                },
                physics: Physics::new(8.0),
                engine: Engine::new(8.0, 8.0),
                health: HealthComponent::new(10, 40, 2.0),
                collider: Collider { radius: 30.0 },
                ..Default::default()
            },
            BaseRotation {
                rotation: Quat::from_rotation_z(-PI / 2.0),
            },
            TextureAtlas {
                layout: mob_assets.get_mob_texture_atlas_layout(&mob_type),
                ..default()
            },
            AnimationComponent {
                timer: Timer::from_seconds(0.25, TimerMode::Repeating),
                direction: PingPong(Forward),
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
