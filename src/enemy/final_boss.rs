use super::{FinalBoss, AI};
use crate::assets::enemy_assets::MobAssets;
use crate::components::health::FighterBundle;
use crate::components::spawnable::{EnemyMobType, MobType};
use crate::ship::animation::AnimationComponent;
use crate::ship::animation::AnimationDirection::PingPong;
use crate::ship::animation::PingPongDirection::Forward;
use crate::{
    components::health::HealthComponent,
    gameplay::physics::{BaseRotation, Collider, Physics},
    ship::{
        engine::{Engine, EngineMethod},
        turret::{DoesDamage, EffectSize, FireRate, Range, TurretBundle, TurretClass},
    },
};
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn spawn_final_boss(commands: &mut Commands, mob_assets: &MobAssets, position: Vec3) {
    let mob_type = MobType::Enemy(EnemyMobType::Shelly);
    commands
        .spawn((
            FighterBundle {
                sprite: SpriteBundle {
                    texture: mob_assets.get_mob_image(&mob_type),
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
                health: HealthComponent::new(1000, 4000, 2.0),
                collider: Collider { radius: 50.0 },
                ..Default::default()
            },
            BaseRotation {
                rotation: Quat::from_rotation_z(-PI),
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
