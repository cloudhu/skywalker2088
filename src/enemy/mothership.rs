use super::AI;
use crate::assets::enemy_assets::MobAssets;
use crate::components::health::Spacecraft;
use crate::components::spawnable::{EnemyMobType, MobType};
use crate::gameplay::gamelogic::ExplodesOnDespawn;
use crate::gameplay::loot::{DropsLoot, WorthPoints};
use crate::ship::animation::AnimationComponent;
use crate::ship::animation::AnimationDirection::PingPong;
use crate::ship::animation::PingPongDirection::Forward;
use crate::ship::turret::MultiShot;
use crate::{
    components::health::Health,
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
    let mut entity = commands.spawn_empty();
    entity.insert((
        Spacecraft,
        Sprite::from_atlas_image(
            mob_assets.get_mob_image(&mob_type),
            TextureAtlas::from(mob_assets.get_mob_texture_atlas_layout(&mob_type)),
        ),
        Transform::from_translation(position),
        Physics::new(12.0),
        Engine {
            max_speed: 3.0,
            power: 3.0,
            method: EngineMethod::Keep(500.0),
            ..Default::default()
        },
        Health::new(100, 80, 2.0),
        Collider { radius: 50.0 },
        ExplodesOnDespawn {
            size_min: 55.0,
            size_max: 65.0,
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
    entity.insert(WorthPoints { value: 50 });
    entity.with_children(|parent| {
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
