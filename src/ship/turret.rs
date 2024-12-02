use crate::assets::game_assets::{AudioAssets, Fonts};
use crate::components::events::TakeDamageEvent;
use crate::components::game::ExplosionRender;
use crate::components::health::{Damage, HealthComponent, Owner, Seeker};
use crate::components::states::AppStates;
use crate::config::GameConfig;
use crate::gameplay::gamelogic::{
    game_not_paused, DespawnWithScene, ExplodesOnDespawn, Targettable, WillTarget,
};
use crate::gameplay::physics::BaseRotation;
use crate::ship::bullet::{AoeDamage, Bullet, DirectDamage, ExpandingCollider, LaserRender};
use crate::ship::engine::Engine;
use crate::util::{Colour, Math, RenderLayer};
use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::distr::Standard;
use rand::prelude::*;
use std::f32::consts::PI;
use std::time::Duration;

#[derive(Component)]
pub struct Range {
    pub max: f32,
}

impl Default for Range {
    fn default() -> Self {
        Range { max: 1000.0 } // Player shouldn't think about range but useful for enemies to have it set
    }
}

#[derive(Component, Default)]
pub struct FireRate {
    pub rate: f32,
    pub timer: Timer,
}

impl FireRate {
    pub fn from_rate_in_seconds(rate: f32) -> FireRate {
        FireRate {
            rate,
            timer: Timer::from_seconds(1.0 / rate, TimerMode::Repeating),
        }
    }

    pub fn set_rate_in_seconds(&mut self, rate: f32) {
        self.rate = rate;
        self.timer.set_duration(Duration::from_secs_f32(1.0 / rate));
    }
}

#[derive(Component, Default)]
pub struct Targets {
    pub target: Option<Entity>,
}

#[derive(Component, Copy, Clone, Eq, Hash, PartialEq, Default)]
pub enum TurretClass {
    #[default]
    AutoCannon,
    BlastLaser,
    RocketLauncher,
    MineLauncher,
    ShrapnelCannon,
    ChainLaser,
    PierceLaser,
    Emp,
}

impl std::fmt::Display for TurretClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TurretClass::AutoCannon => write!(f, "Auto Cannon"),
            TurretClass::BlastLaser => write!(f, "Blast Laser"),
            TurretClass::RocketLauncher => write!(f, "Rocket Launcher"),
            TurretClass::MineLauncher => write!(f, "Mine Launcher"),
            TurretClass::ShrapnelCannon => write!(f, "Shrapnel Cannon"),
            TurretClass::ChainLaser => write!(f, "Chain Laser"),
            TurretClass::PierceLaser => write!(f, "Pierce Laser"),
            TurretClass::Emp => write!(f, "EM Pulsar"),
        }
    }
}

impl Distribution<TurretClass> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TurretClass {
        match rng.gen_range(0..8) {
            0 => TurretClass::BlastLaser,
            1 => TurretClass::RocketLauncher,
            2 => TurretClass::MineLauncher,
            3 => TurretClass::ShrapnelCannon,
            4 => TurretClass::ChainLaser,
            5 => TurretClass::PierceLaser,
            6 => TurretClass::Emp,
            _ => TurretClass::AutoCannon,
        }
    }
}

#[derive(Component, Default)]
pub struct DoesDamage {
    pub amount: usize,
    pub crit_chance: f32,
}

impl DoesDamage {
    pub fn from_amount(amount: usize) -> Self {
        Self {
            amount,
            ..Default::default()
        }
    }

    pub fn roll(&self) -> Damage {
        if thread_rng().gen_range(0.0..1.0) < self.crit_chance {
            return Damage {
                amount: self.amount * 2,
                is_crit: true,
            };
        }
        Damage {
            amount: self.amount,
            is_crit: false,
        }
    }
}

#[derive(Component)]
pub struct MultiShot {
    pub amount: u8,
}

#[derive(Component, Default)]
pub struct EffectSize(pub f32);

impl Default for MultiShot {
    fn default() -> Self {
        MultiShot { amount: 1 }
    }
}

#[derive(Component)]
pub struct EffectColour(pub Color);

impl Default for EffectColour {
    fn default() -> Self {
        EffectColour(Colour::RED)
    }
}

#[derive(Bundle, Default)]
pub struct TurretBundle {
    pub range: Range,
    pub fire_rate: FireRate,
    pub target: Targets,
    pub class: TurretClass,
    pub damage: DoesDamage,
    pub shots: MultiShot,
    pub size: EffectSize,
    pub colour: EffectColour,
}

impl TurretBundle {
    pub fn auto_cannon() -> TurretBundle {
        TurretBundle {
            class: TurretClass::AutoCannon,
            fire_rate: FireRate::from_rate_in_seconds(2.0),
            damage: DoesDamage::from_amount(2),
            colour: EffectColour(Colour::PLAYER),
            ..Default::default()
        }
    }

    pub fn blast_laser() -> TurretBundle {
        TurretBundle {
            class: TurretClass::BlastLaser,
            fire_rate: FireRate::from_rate_in_seconds(1.5),
            damage: DoesDamage::from_amount(1),
            colour: EffectColour(Colour::PINK),
            ..Default::default()
        }
    }

    pub fn rocket_launcher() -> TurretBundle {
        TurretBundle {
            class: TurretClass::RocketLauncher,
            fire_rate: FireRate::from_rate_in_seconds(0.5),
            damage: DoesDamage::from_amount(5),
            colour: EffectColour(Colour::YELLOW),
            ..Default::default()
        }
    }

    pub fn mine_launcher() -> TurretBundle {
        TurretBundle {
            class: TurretClass::MineLauncher,
            fire_rate: FireRate::from_rate_in_seconds(0.9),
            damage: DoesDamage::from_amount(6),
            size: EffectSize(40.0),
            shots: MultiShot { amount: 3 },
            colour: EffectColour(Colour::PLAYER),
            ..Default::default()
        }
    }

    pub fn shrapnel_cannon() -> TurretBundle {
        TurretBundle {
            class: TurretClass::ShrapnelCannon,
            fire_rate: FireRate::from_rate_in_seconds(0.25),
            damage: DoesDamage::from_amount(2),
            shots: MultiShot { amount: 16 },
            colour: EffectColour(Colour::PLAYER),
            ..Default::default()
        }
    }

    pub fn chain_laser() -> TurretBundle {
        TurretBundle {
            class: TurretClass::ChainLaser,
            fire_rate: FireRate::from_rate_in_seconds(0.4),
            damage: DoesDamage::from_amount(1),
            shots: MultiShot { amount: 3 },
            colour: EffectColour(Colour::GREEN),
            ..Default::default()
        }
    }

    pub fn pierce_laser() -> TurretBundle {
        TurretBundle {
            class: TurretClass::PierceLaser,
            fire_rate: FireRate::from_rate_in_seconds(0.15),
            damage: DoesDamage::from_amount(8),
            size: EffectSize(1.0),
            colour: EffectColour(Colour::YELLOW),
            ..Default::default()
        }
    }

    pub fn emp() -> TurretBundle {
        TurretBundle {
            class: TurretClass::Emp,
            fire_rate: FireRate::from_rate_in_seconds(0.7),
            damage: DoesDamage::from_amount(3),
            size: EffectSize(80.0),
            colour: EffectColour(Colour::SHIELD),
            ..Default::default()
        }
    }

    pub fn from_class(class: &TurretClass) -> TurretBundle {
        match class {
            TurretClass::AutoCannon => TurretBundle::auto_cannon(),
            TurretClass::BlastLaser => TurretBundle::blast_laser(),
            TurretClass::RocketLauncher => TurretBundle::rocket_launcher(),
            TurretClass::MineLauncher => TurretBundle::mine_launcher(),
            TurretClass::ShrapnelCannon => TurretBundle::shrapnel_cannon(),
            TurretClass::ChainLaser => TurretBundle::chain_laser(),
            TurretClass::PierceLaser => TurretBundle::pierce_laser(),
            TurretClass::Emp => TurretBundle::emp(),
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_event::<TurretFireEvent>()
        .add_systems(
            Update,
            (turret_targetting_system, turret_fire_system)
                .distributive_run_if(game_not_paused)
                .distributive_run_if(in_state(AppStates::Game)),
        )
        .add_systems(
            Update,
            (
                fire_blast_laser,
                fire_rocket_launcher,
                fire_mine_launcher,
                fire_auto_cannon,
                fire_shrapnel_cannon,
                fire_chain_laser,
                fire_pierce_laser,
                fire_emp,
            )
                .distributive_run_if(in_state(AppStates::Game)),
        );
}

#[derive(Event)]
pub struct TurretFireEvent {
    pub class: TurretClass,
    pub turret: Entity,
}

pub fn get_closest_target(
    potentials: &mut [(Entity, &Transform, &Targettable)],
    point: Vec2,
) -> Option<Entity> {
    potentials.sort_by(|a, b| {
        a.1.translation
            .truncate()
            .distance(point)
            .partial_cmp(&b.1.translation.truncate().distance(point))
            .unwrap()
    });
    potentials.first().map(|potential| potential.0)
}

fn turret_targetting_system(
    mut query: Query<(&mut Targets, &Parent, &Range)>,
    target_query: Query<(Entity, &Transform, &Targettable)>,
    parent_query: Query<(&Transform, Entity, &WillTarget)>,
) {
    for (mut targets, parent, range) in &mut query {
        // Get parent (ship)
        if let Ok((parent_transform, parent_entity, parent_will_target)) =
            parent_query.get(parent.get())
        {
            if let Some(target) = targets.target {
                // Check still in range
                if let Ok(current_target) = target_query.get(target) {
                    if current_target
                        .1
                        .translation
                        .truncate()
                        .distance(parent_transform.translation.truncate())
                        > range.max
                    {
                        targets.target = None;
                    }
                }
            } else {
                // Look for a target
                let mut potentials_without_parent: Vec<(Entity, &Transform, &Targettable)> =
                    target_query
                        .iter()
                        .filter(|a| a.0 != parent_entity && parent_will_target.0.contains(&a.2 .0))
                        .filter(|a| {
                            a.1.translation
                                .truncate()
                                .distance(parent_transform.translation.truncate())
                                <= range.max
                        })
                        .collect();
                targets.target = get_closest_target(
                    &mut potentials_without_parent,
                    parent_transform.translation.truncate(),
                );
            }
        }
    }
}

fn turret_fire_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut FireRate, &TurretClass, &mut Targets, Entity)>,
    mut fire_event: EventWriter<TurretFireEvent>,
) {
    for (mut fire_rate, class, mut targets, entity) in &mut query {
        if let Some(target) = targets.target {
            // Check target still exists and if not clear it

            if commands.get_entity(target).is_none() {
                targets.target = None;
                break;
            }

            fire_rate.timer.tick(time.delta());
            if fire_rate.timer.just_finished() {
                // Fire!
                fire_event.send(TurretFireEvent {
                    class: *class,
                    turret: entity,
                });
            }
        } else {
            fire_rate.timer.reset();
        }
    }
}

pub fn fire_blast_laser(
    mut commands: Commands,
    mut fire_event: EventReader<TurretFireEvent>,
    turret_query: Query<(&Parent, &Targets, &DoesDamage, &EffectColour)>,
    parent_query: Query<&Transform>,
    target_query: Query<&Transform>,
    mut take_damage_event: EventWriter<TakeDamageEvent>,
) {
    for ev in fire_event.read() {
        if ev.class == TurretClass::BlastLaser {
            // Get Turret Info
            let Ok((parent, targets, damage, colour)) = turret_query.get(ev.turret) else {
                continue;
            };

            // Get Target
            let Some(target) = targets.target else {
                continue;
            };

            // Get Target Info
            let Ok(target_transform) = target_query.get(target) else {
                continue;
            };

            // Get Parent Info
            let Ok(parent_transform) = parent_query.get(parent.get()) else {
                continue;
            };

            // Spawn graphic
            let origin = parent_transform.translation.truncate();
            let target_pos = target_transform.translation.truncate();
            commands.spawn((
                Bullet::new(0.1),
                LaserRender,
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Line(origin, target_pos)),
                    spatial: SpatialBundle::from_transform(Transform::from_xyz(
                        0.,
                        0.,
                        RenderLayer::Bullet.as_z(),
                    )),
                    ..default()
                },
                Stroke::new(colour.0, 1.0),
                Owner(parent.get()),
                DespawnWithScene,
            ));

            // Immediate hit
            take_damage_event.send(TakeDamageEvent {
                target,
                damage: damage.roll(),
            });
        }
    }
}

pub fn fire_auto_cannon(
    mut commands: Commands,
    mut fire_event: EventReader<TurretFireEvent>,
    turret_query: Query<(&Parent, &Targets, &DoesDamage, &EffectColour)>,
    parent_query: Query<&Transform>,
    target_query: Query<&Transform>,
    fonts: Res<Fonts>,
) {
    for ev in fire_event.read() {
        if ev.class == TurretClass::AutoCannon {
            // Get Turret Info
            let Ok((parent, targets, damage, colour)) = turret_query.get(ev.turret) else {
                continue;
            };

            // Get Target
            let Some(target) = targets.target else {
                continue;
            };

            // Get Target Info
            let Ok(target_transform) = target_query.get(target) else {
                continue;
            };

            // Get Parent Info
            let Ok(parent_transform) = parent_query.get(parent.get()) else {
                continue;
            };

            // Spawn bullet
            let bullet_speed = 1000.0;
            let origin = parent_transform.translation.truncate();
            let destination = target_transform.translation.truncate();
            let direction = (destination - origin).normalize();
            spawn_bullet(
                &mut commands,
                fonts.primary.clone(),
                colour,
                origin.extend(RenderLayer::Bullet.as_z()),
                parent,
                damage,
                &".".to_string(),
                1.2,
                16.0,
            );
        }
    }
}

fn spawn_link(
    commands: &mut Commands,
    take_damage_event: &mut EventWriter<TakeDamageEvent>,
    target_query: &Query<&Transform>,
    origin: Vec2,
    target: Entity,
    damage: &DoesDamage,
    jump: u8,
    colour: &EffectColour,
    owner: Entity,
) -> Result<Vec2, QueryEntityError> {
    // Get Target Info
    let target_transform = target_query.get(target)?;
    let target_position = target_transform.translation.truncate();
    // Spawn graphic
    commands.spawn((
        Bullet::new(0.2 + (jump as f32) * 0.1),
        LaserRender,
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Line(origin, target_position)),
            spatial: SpatialBundle::from_transform(Transform::from_xyz(
                0.,
                0.,
                RenderLayer::Bullet.as_z(),
            )),
            ..default()
        },
        Stroke::new(colour.0, 2.0),
        Owner(owner),
        DespawnWithScene,
    ));
    // Immediate hit
    take_damage_event.send(TakeDamageEvent {
        target,
        damage: damage.roll(),
    });
    Ok(target_position)
}

pub fn fire_chain_laser(
    mut commands: Commands,
    mut fire_event: EventReader<TurretFireEvent>,
    turret_query: Query<(&Parent, &Targets, &DoesDamage, &MultiShot, &EffectColour)>,
    parent_query: Query<(&Transform, &WillTarget)>,
    target_query: Query<&Transform>,
    potential_query: Query<
        (Entity, &Transform, &Targettable),
        (With<Targettable>, With<Transform>),
    >,
    mut take_damage_event: EventWriter<TakeDamageEvent>,
) {
    for ev in fire_event.read() {
        if ev.class == TurretClass::ChainLaser {
            // Get Turret Info
            let Ok((parent, targets, damage, shots, colour)) = turret_query.get(ev.turret) else {
                continue;
            };

            // Get Target
            let Some(target) = targets.target else {
                continue;
            };

            // Get Parent Info
            let Ok((parent_transform, parent_will_target)) = parent_query.get(parent.get()) else {
                continue;
            };

            // Get all possible targets
            let mut potential_targets: Vec<(Entity, &Transform, &Targettable)> = potential_query
                .iter()
                .filter(|a| a.0 != parent.get() && parent_will_target.0.contains(&a.2 .0))
                .collect();

            // Get other nearby targets to bounce to
            let mut num_jumps = 0;
            let mut current_target = Some(target);
            let mut previous_position = parent_transform.translation.truncate();

            while num_jumps < shots.amount && current_target.is_some() {
                num_jumps += 1;

                let Some(target) = current_target else {
                    break;
                };

                // Remove target from potentials list so no repeats
                potential_targets.retain(|potential| potential.0 != target);

                let result = spawn_link(
                    &mut commands,
                    &mut take_damage_event,
                    &target_query,
                    previous_position,
                    target,
                    damage,
                    num_jumps,
                    colour,
                    parent.get(),
                );

                match result {
                    Ok(pos) => {
                        previous_position = pos;
                    }
                    Err(_) => break,
                }

                current_target = get_closest_target(&mut potential_targets, previous_position)
            }
        }
    }
}

pub fn fire_emp(
    mut commands: Commands,
    mut fire_event: EventReader<TurretFireEvent>,
    turret_query: Query<(&Parent, &DoesDamage, &EffectSize, &EffectColour)>,
    parent_query: Query<&Transform>,
) {
    for ev in fire_event.read() {
        if ev.class == TurretClass::Emp {
            // Get Turret Info
            let Ok((parent, damage, size, colour)) = turret_query.get(ev.turret) else {
                continue;
            };

            // Get Parent Info
            let Ok(parent_transform) = parent_query.get(parent.get()) else {
                continue;
            };

            let origin = parent_transform.translation.truncate();
            let time_to_live = 1.0;

            // Spawn graphic
            commands.spawn((
                ExplosionRender {
                    origin,
                    radius: size.0,
                    ttl: Timer::from_seconds(time_to_live, TimerMode::Once),
                    fade_out: true,
                },
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Circle {
                        center: origin,
                        radius: 0.0,
                    }),
                    spatial: SpatialBundle::from_transform(Transform::from_xyz(
                        0.0,
                        0.0,
                        RenderLayer::Effects.as_z(),
                    )),
                    ..default()
                },
                Stroke::new(colour.0, 1.0),
            ));

            // Spawn bullet that damages
            commands.spawn((
                Bullet {
                    time2live: Timer::from_seconds(time_to_live, TimerMode::Once),
                    despawn_on_hit: false,
                    ..Default::default()
                },
                Transform::from_translation(parent_transform.translation),
                ExpandingCollider {
                    final_radius: size.0,
                },
                DirectDamage(damage.roll()),
                Owner(parent.get()),
            ));
        }
    }
}

pub fn fire_mine_launcher(
    mut commands: Commands,
    mut fire_event: EventReader<TurretFireEvent>,
    turret_query: Query<(&Parent, &DoesDamage, &EffectSize, &EffectColour, &MultiShot)>,
    parent_query: Query<&Transform>,
    fonts: Res<Fonts>,
    sound_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    config: Res<GameConfig>,
) {
    for ev in fire_event.read() {
        if ev.class == TurretClass::MineLauncher {
            // Get Turret Info
            let Ok((parent, damage, size, colour, shots)) = turret_query.get(ev.turret) else {
                continue;
            };

            // Get Parent Info
            let Ok(parent_transform) = parent_query.get(parent.get()) else {
                continue;
            };

            // Spawn mine
            let origin = parent_transform.translation.truncate();
            //播放音效
            audio
                .play(sound_assets.zap.clone())
                .with_volume(Volume::Amplitude(config.sfx_volume as f64));
            commands.spawn((
                Bullet::new(30.0),
                Text2dBundle {
                    text: Text::from_section(
                        "¤",
                        TextStyle {
                            font: fonts.primary.clone(),
                            font_size: 12.0,
                            color: colour.0,
                        },
                    )
                    .with_justify(JustifyText::Center),
                    transform: Transform {
                        translation: origin.extend(RenderLayer::Bullet.as_z()),
                        ..Default::default()
                    },
                    ..default()
                },
                HealthComponent::new(1, 0, 3.0),
                Owner(parent.get()),
                ExplodesOnDespawn {
                    amount_min: shots.amount as u32,
                    amount_max: shots.amount as u32,
                    size_min: size.0 / 2.0,
                    size_max: size.0 / 2.0,
                    colour: colour.0,
                    spread: size.0,
                    ..Default::default()
                },
                AoeDamage {
                    damage: damage.roll(),
                    range: size.0,
                },
                DespawnWithScene,
            ));
        }
    }
}

pub fn fire_pierce_laser(
    mut commands: Commands,
    mut fire_event: EventReader<TurretFireEvent>,
    turret_query: Query<(&Parent, &Targets, &DoesDamage, &EffectSize, &EffectColour)>,
    parent_query: Query<(&Transform, &WillTarget)>,
    target_query: Query<&Transform>,
    potential_query: Query<(Entity, &Transform, &Targettable)>,
    mut take_damage_event: EventWriter<TakeDamageEvent>,
) {
    for ev in fire_event.read() {
        if ev.class == TurretClass::PierceLaser {
            // Get Turret Info
            let Ok((parent, targets, damage, size, colour)) = turret_query.get(ev.turret) else {
                continue;
            };

            // Get Target
            let Some(target) = targets.target else {
                continue;
            };

            // Get Target Info
            let Ok(target_transform) = target_query.get(target) else {
                continue;
            };

            // Get Parent Info
            let Ok((parent_transform, parent_will_target)) = parent_query.get(parent.get()) else {
                continue;
            };

            // Spawn graphic
            const LASER_LENGTH: f32 = 8000.0;
            let origin = parent_transform.translation.truncate();
            let target = target_transform.translation.truncate();
            let end = (target - origin).normalize() * LASER_LENGTH;
            commands.spawn((
                Bullet::new(1.0),
                LaserRender,
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Line(origin, end)),
                    spatial: SpatialBundle::from_transform(Transform::from_xyz(
                        0.,
                        0.,
                        RenderLayer::Bullet.as_z(),
                    )),
                    ..default()
                },
                Stroke::new(colour.0, size.0),
                Owner(parent.get()),
                DespawnWithScene,
            ));

            // Hit everything on the path
            let events = potential_query
                .iter()
                .filter(|a| a.0 != parent.get() && parent_will_target.0.contains(&a.2 .0))
                .filter(|a| {
                    Math::distance_from_point_to_line(a.1.translation.truncate(), origin, end)
                        <= 1.0 + size.0
                })
                .map(|hit| TakeDamageEvent {
                    target: hit.0,
                    damage: damage.roll(),
                });
            take_damage_event.send_batch(events);
        }
    }
}

pub fn fire_rocket_launcher(
    mut commands: Commands,
    mut fire_event: EventReader<TurretFireEvent>,
    turret_query: Query<(&Parent, &Targets, &DoesDamage, &MultiShot, &EffectColour)>,
    parent_query: Query<&Transform>,
    fonts: Res<Fonts>,
    sound_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    config: Res<GameConfig>,
) {
    for ev in fire_event.read() {
        if ev.class == TurretClass::RocketLauncher {
            // Get Turret Info
            let Ok((parent, targets, damage, shots, colour)) = turret_query.get(ev.turret) else {
                continue;
            };

            // Get Target
            let Some(target) = targets.target else {
                continue;
            };

            // Get Parent Info
            let Ok(parent_transform) = parent_query.get(parent.get()) else {
                continue;
            };

            // Spawn rocket
            let origin = parent_transform.translation.truncate();
            for _ in 0..shots.amount {
                //播放音效
                audio
                    .play(sound_assets.player_fire.clone())
                    .with_volume(Volume::Amplitude(config.sfx_volume as f64));
                commands.spawn((
                    Bullet::new(3.0),
                    Text2dBundle {
                        text: Text::from_section(
                            "!",
                            TextStyle {
                                font: fonts.primary.clone(),
                                font_size: 12.0,
                                color: colour.0,
                            },
                        )
                        .with_justify(JustifyText::Center),
                        transform: Transform {
                            translation: origin.extend(RenderLayer::Bullet.as_z()),
                            ..Default::default()
                        },
                        ..default()
                    },
                    BaseRotation {
                        rotation: Quat::from_rotation_z(PI / 2.0),
                    },
                    Engine::new_with_steering(40.0, 10.0, 0.5),
                    Seeker(target),
                    Owner(parent.get()),
                    ExplodesOnDespawn {
                        colour: colour.0,
                        ..Default::default()
                    },
                    AoeDamage {
                        damage: damage.roll(),
                        range: 40.0,
                    },
                    DespawnWithScene,
                ));
            }
        }
    }
}

pub fn fire_shrapnel_cannon(
    mut commands: Commands,
    mut fire_event: EventReader<TurretFireEvent>,
    turret_query: Query<(&Parent, &Targets, &DoesDamage, &MultiShot, &EffectColour)>,
    parent_query: Query<&Transform>,
    target_query: Query<&Transform>,
    fonts: Res<Fonts>,
) {
    for ev in fire_event.read() {
        if ev.class == TurretClass::ShrapnelCannon {
            // Get Turret Info
            let Ok((parent, targets, damage, shots, colour)) = turret_query.get(ev.turret) else {
                continue;
            };

            // Get Target
            let Some(target) = targets.target else {
                continue;
            };

            // Get Target Info
            let Ok(target_transform) = target_query.get(target) else {
                continue;
            };

            // Get Parent Info
            let Ok(parent_transform) = parent_query.get(parent.get()) else {
                continue;
            };

            // Spawn bullets
            const SPREAD: f32 = PI / 4.0;
            const SPEED_VARIANCE: f32 = 400.0;

            let bullet_speed = 600.0;
            let origin = parent_transform.translation.truncate();
            let destination = target_transform.translation.truncate();
            let direction = (destination - origin).normalize();

            let mut rng: ThreadRng = thread_rng();
            for _ in 0..shots.amount {
                let random_angle = rng.gen_range(-SPREAD / 2.0..SPREAD / 2.0);
                let spread_direction = Vec2::from_angle(random_angle).rotate(direction);
                let random_speed =
                    rng.gen_range(-SPEED_VARIANCE / 2.0..SPEED_VARIANCE / 2.0) + bullet_speed;
                spawn_bullet(
                    &mut commands,
                    fonts.primary.clone(),
                    colour,
                    origin.extend(RenderLayer::Bullet.as_z()),
                    parent,
                    damage,
                    &".".to_string(),
                    1.2,
                    16.0,
                );
            }
        }
    }
}

fn spawn_bullet(
    commands: &mut Commands,
    font: Handle<Font>,
    colour: &EffectColour,
    translation: Vec3,
    parent: &Parent,
    damage: &DoesDamage,
    bullet_text: &String,
    seconds2live: f32,
    font_size: f32,
) {
    commands.spawn((
        Bullet::new(seconds2live),
        Text2dBundle {
            text: Text::from_section(
                bullet_text,
                TextStyle {
                    font,
                    font_size,
                    color: colour.0,
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform {
                translation,
                ..Default::default()
            },
            ..default()
        },
        Owner(parent.get()),
        DirectDamage(damage.roll()),
        DespawnWithScene,
    ));
}
