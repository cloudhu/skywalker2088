use super::AI;
use crate::assets::enemy_assets::MobAssets;
use crate::components::health::FighterBundle;
use crate::components::spawnable::{EnemyMobType, MobType};
use crate::gameplay::gamelogic::ExplodesOnDespawn;
use crate::gameplay::loot::{DropsLoot, WorthPoints};
use crate::ship::animation::AnimationComponent;
use crate::ship::animation::AnimationDirection::PingPong;
use crate::ship::animation::PingPongDirection::Forward;
use crate::ship::turret::MultiShot;
use crate::{
    components::health::HealthComponent,
    gameplay::physics::{BaseRotation, Collider, Physics},
    ship::{
        engine::{Engine, EngineMethod},
        turret::{DoesDamage, FireRate, Range, TurretBundle, TurretClass},
    },
};
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn spawn_mothership(commands: &mut Commands, mob_assets: &MobAssets, position: Vec3) {
    let mob_type = MobType::Enemy(EnemyMobType::Ferritharax);
    commands
        .spawn((
            FighterBundle {
                sprite: SpriteBundle {
                    texture: mob_assets.get_mob_image(&mob_type),
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
                health: HealthComponent::new(100, 80, 2.0),
                collider: Collider { radius: 50.0 },
                explodes_on_despawn: ExplodesOnDespawn {
                    size_min: 55.0,
                    size_max: 65.0,
                    ..Default::default()
                },
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
