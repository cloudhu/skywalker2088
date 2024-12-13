use super::{FinalBoss, AI};
use crate::assets::enemy_assets::MobAssets;
use crate::components::health::Spacecraft;
use crate::components::spawnable::{EnemyMobType, MobType};
use crate::gameplay::gamelogic::ExplodesOnDespawn;
use crate::gameplay::loot::{DropsLoot, WorthPoints};
use crate::ship::animation::AnimationComponent;
use crate::ship::animation::AnimationDirection::PingPong;
use crate::ship::animation::PingPongDirection::Forward;
use crate::{
    components::health::Health,
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
    let mut entity = commands.spawn_empty();
    entity.insert((
        Spacecraft,
        Sprite::from_atlas_image(
            mob_assets.get_mob_image(&mob_type),
            TextureAtlas::from(mob_assets.get_mob_texture_atlas_layout(&mob_type)),
        ),
        Transform::from_translation(position),
        Physics::new(8.0),
        Engine {
            power: 40.0,
            max_speed: 80.0,
            method: EngineMethod::Keep(200.0),
            ..Default::default()
        },
        Health::new(1000, 4000, 2.0),
        Collider { radius: 50.0 },
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
    entity.insert(FinalBoss);
    entity.insert(DropsLoot);
    entity.insert(WorthPoints { value: 50 });
    entity.with_children(|parent| {
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
