use crate::assets::game_assets::AudioAssets;
use crate::components::events::TakeDamageEvent;
use crate::components::game::{ExplosionRender, ShouldDespawn};
use crate::components::health::*;
use crate::components::states::AppStates;
use crate::config::GameConfig;
use crate::gameplay::gamelogic::game_not_paused;
use crate::gameplay::physics::Collider;
use crate::AppSet;
use bevy::{prelude::*, utils::HashMap};
use bevy_kira_audio::prelude::Volume;
use bevy_kira_audio::{Audio, AudioControl};
use bevy_prototype_lyon::prelude::{GeometryBuilder, Path, Stroke};
use bevy_prototype_lyon::shapes;

#[derive(Component)]
pub struct Bullet {
    pub time2live: Timer,
    pub max_hits_per_entity: u8,
    pub entities_hit: HashMap<Entity, u8>,
    pub despawn_on_hit: bool,
}

impl Bullet {
    pub fn new(seconds_to_live: f32) -> Bullet {
        Bullet {
            time2live: Timer::from_seconds(seconds_to_live, TimerMode::Once),
            ..Default::default()
        }
    }
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            time2live: Timer::from_seconds(1.0, TimerMode::Once),
            max_hits_per_entity: 1,
            entities_hit: HashMap::new(),
            despawn_on_hit: true,
        }
    }
}

#[derive(Component)]
pub struct LaserRender;

#[derive(Component)]
pub struct DirectDamage(pub Damage);

#[derive(Component)]
pub struct AoeDamage {
    pub damage: Damage,
    pub range: f32,
}

#[derive(Component)]
pub struct ExpandingCollider {
    pub final_radius: f32,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            bullet_system,
            bullet_collision_system,
            laser_render_system,
            explosion_render_system,
            expanding_collider_system,
        )
            .chain()
            .in_set(AppSet::Update)
            .distributive_run_if(game_not_paused)
            .distributive_run_if(in_state(AppStates::Game)),
    );
}

pub fn bullet_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Bullet, Entity, &Transform, &Owner, Option<&AoeDamage>), With<Bullet>>,
    potential_query: Query<
        (&Collider, &Transform, Entity),
        (Without<Bullet>, With<Collider>, With<HealthComponent>),
    >,
    mut take_damage_event: EventWriter<TakeDamageEvent>,
) {
    for (mut bullet, entity, transform, owner, aoe_damage) in &mut query {
        bullet.time2live.tick(time.delta());
        if bullet.time2live.just_finished() {
            // If timed out Aoe damage should still occur
            if let Some(aoe_damage) = aoe_damage {
                let potentials = potential_query
                    .iter()
                    .filter(|&potential| potential.2 != owner.0)
                    .collect::<Vec<_>>();
                do_aoe_damage(
                    potentials,
                    (&mut bullet, transform, aoe_damage),
                    &mut take_damage_event,
                );
            }

            commands.entity(entity).insert(ShouldDespawn);
        }
    }
}

pub fn bullet_collision_system(
    mut commands: Commands,
    mut query: Query<
        (
            &Collider,
            &Transform,
            Entity,
            &Owner,
            Option<&DirectDamage>,
            Option<&AoeDamage>,
            &mut Bullet,
        ),
        (
            With<Bullet>,
            With<Collider>,
            With<Owner>,
            Without<ShouldDespawn>,
        ),
    >,
    potential_query: Query<
        (&Collider, &Transform, Entity),
        (Without<Bullet>, With<Collider>, With<HealthComponent>),
    >,
    mut take_damage_event: EventWriter<TakeDamageEvent>,
) {
    for (collider, transform, entity, owner, direct_damage, aoe_damage, mut bullet) in &mut query {
        // Get all potentials
        let potentials = potential_query
            .iter()
            .filter(|&potential| potential.2 != owner.0)
            .collect::<Vec<_>>();

        // Sort by distance to bullet
        let hit = potentials.iter().find(|potential| {
            transform
                .translation
                .truncate()
                .distance(potential.1.translation.truncate())
                <= collider.radius + potential.0.radius
                && bullet.entities_hit.get(&potential.2).unwrap_or(&0) < &bullet.max_hits_per_entity
        });

        if let Some((_collider, _transform, potential_entity)) = hit {
            if let Some(direct_damage) = direct_damage {
                let number_of_times_hit = bullet.entities_hit.entry(*potential_entity).or_insert(0);
                *number_of_times_hit += 1;

                take_damage_event.send(TakeDamageEvent {
                    target: *potential_entity,
                    damage: direct_damage.0,
                });
            }

            if let Some(aoe_damage) = aoe_damage {
                do_aoe_damage(
                    potentials,
                    (&mut bullet, transform, aoe_damage),
                    &mut take_damage_event,
                );
            }

            if bullet.despawn_on_hit {
                commands.entity(entity).insert(ShouldDespawn);
            }
            break;
        }
    }
}

fn do_aoe_damage(
    potentials: Vec<(&Collider, &Transform, Entity)>,
    bullet: (&mut Bullet, &Transform, &AoeDamage),
    take_damage_event: &mut EventWriter<TakeDamageEvent>,
) {
    let (bullet, transform, aoe_damage) = bullet;
    let all_hits: Vec<_> = potentials
        .iter()
        .filter(|potential| {
            transform
                .translation
                .truncate()
                .distance(potential.1.translation.truncate())
                <= aoe_damage.range + potential.0.radius
                && bullet.entities_hit.get(&potential.2).unwrap_or(&0) < &bullet.max_hits_per_entity
        })
        .collect();
    for h in all_hits.iter() {
        let number_of_times_hit = bullet.entities_hit.entry(h.2).or_insert(0);
        *number_of_times_hit += 1;

        take_damage_event.send(TakeDamageEvent {
            target: h.2,
            damage: aoe_damage.damage,
        });
    }
}

pub fn laser_render_system(
    mut query: Query<(&Bullet, &mut Stroke), (With<LaserRender>, With<Bullet>, With<Stroke>)>,
    sound_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    config: Res<GameConfig>,
) {
    for (bullet, mut stroke) in &mut query {
        stroke
            .color
            .set_alpha(bullet.time2live.fraction_remaining());
        //播放laser音效
        audio
            .play(sound_assets.laser1.clone())
            .with_volume(Volume::Amplitude(config.sfx_volume as f64));
    }
}

pub fn explosion_render_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (&mut ExplosionRender, &mut Path, Entity, &mut Stroke),
        Without<ShouldDespawn>,
    >,
    sound_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    config: Res<GameConfig>,
) {
    for (mut explosion, mut path, entity, mut stroke) in &mut query {
        explosion.ttl.tick(time.delta());

        let shape = shapes::Circle {
            center: explosion.origin,
            radius: explosion.radius * explosion.ttl.fraction(),
        };
        *path = GeometryBuilder::build_as(&shape);

        if explosion.fade_out {
            stroke.color.set_alpha(explosion.ttl.fraction_remaining());
        }

        if explosion.ttl.finished() {
            //播放爆炸音效
            audio
                .play(sound_assets.big_explosion.clone())
                .with_volume(Volume::Amplitude(config.sfx_volume as f64))
                .handle();
            commands.entity(entity).insert(ShouldDespawn);
        }
    }
}

pub fn expanding_collider_system(mut query: Query<(&mut Collider, &ExpandingCollider, &Bullet)>) {
    for (mut collider, expanding, bullet) in &mut query {
        collider.radius = bullet.time2live.fraction() * expanding.final_radius;
    }
}
