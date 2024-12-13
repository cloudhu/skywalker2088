use std::f32::consts::PI;

use super::AI;
use crate::assets::enemy_assets::MobAssets;
use crate::components::health::{Health, Spacecraft};
use crate::components::spawnable::{EnemyMobType, MobType};
use crate::gameplay::gamelogic::ExplodesOnDespawn;
use crate::gameplay::loot::{DropsLoot, WorthPoints};
use crate::gameplay::physics::{BaseRotation, Collider, Physics};
use crate::ship::animation::AnimationComponent;
use crate::ship::animation::AnimationDirection::PingPong;
use crate::ship::animation::PingPongDirection::Forward;
use crate::ship::engine::Engine;
use crate::ship::turret::{DoesDamage, FireRate, Range, TurretBundle, TurretClass};
use bevy::prelude::*;

pub fn spawn_drone(commands: &mut Commands, mob_assets: &MobAssets, position: Vec3) {
    let mob_type = MobType::Enemy(EnemyMobType::Drone);
    let mut entity = commands.spawn_empty();
    entity.insert((
        Spacecraft,
        Sprite::from_atlas_image(
            mob_assets.get_mob_image(&mob_type),
            TextureAtlas::from(mob_assets.get_mob_texture_atlas_layout(&mob_type)),
        ),
        Transform::from_translation(position),
        Physics::new(5.0),
        Engine::new(10.0, 10.0),
        Health::new(1, 4, 2.0),
        Collider { radius: 10.0 },
        ExplodesOnDespawn {
            size_min: 15.0,
            size_max: 20.0,
            ..Default::default()
        },
    ));
    entity.insert(BaseRotation {
        rotation: Quat::from_rotation_z(-PI / 2.0),
    });
    entity.insert(AnimationComponent {
        timer: Timer::from_seconds(0.25, TimerMode::Repeating),
        direction: PingPong(Forward),
    });
    entity.insert(AI);
    entity.insert(DropsLoot);
    entity.insert(WorthPoints { value: 10 });
    entity.with_children(|parent| {
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
