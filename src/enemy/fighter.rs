use super::AI;
use crate::assets::enemy_assets::MobAssets;
use crate::components::health::FighterBundle;
use crate::components::spawnable::{EnemyMobType, MobType};
use crate::gameplay::gamelogic::ExplodesOnDespawn;
use crate::gameplay::loot::{DropsLoot, WorthPoints};
use crate::ship::animation::AnimationComponent;
use crate::ship::animation::AnimationDirection::PingPong;
use crate::ship::animation::PingPongDirection::Forward;
use crate::{
    components::health::HealthComponent,
    gameplay::physics::{BaseRotation, Collider, Physics},
    ship::{
        engine::Engine,
        turret::{DoesDamage, FireRate, TurretBundle, TurretClass},
    },
};
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn spawn_fighter(commands: &mut Commands, mob_assets: &MobAssets, position: Vec3) {
    let mob_type = MobType::Enemy(EnemyMobType::MissileLauncher);
    commands
        .spawn((
            FighterBundle {
                sprite: SpriteBundle {
                    texture: mob_assets.get_mob_image(&mob_type),
                    transform: Transform::from_translation(position),
                    ..default()
                },
                physics: Physics::new(5.0),
                engine: Engine::new(14.0, 14.0),
                health: HealthComponent::new(10, 0, 2.0),
                collider: Collider { radius: 10.0 },
                explodes_on_despawn: ExplodesOnDespawn {
                    size_min: 20.0,
                    size_max: 25.0,
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
