use crate::components::common::Health;
use crate::gameplay::effects::{FloatingText, HitFlash};
use crate::gameplay::loot::{DropsLoot, IsLoot, Points, WorthPoints};
use crate::gameplay::physics::{Collider, Physics};
use crate::gameplay::player::IsPlayer;
use crate::gameplay::GameState;
use crate::screens::AppState;
use crate::ship::bullet::{ExplosionRender, ShouldDespawn};
use crate::ship::platform::{play_sound_effects, Fonts, SoundAssets};
use crate::util::{Colour, Math, RenderLayer};
use crate::{AppSet, CameraShake, MainCamera};
use bevy::app::App;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy_parallax::{ParallaxMoveEvent, ParallaxSystems};
use bevy_prototype_lyon::prelude::{GeometryBuilder, ShapeBundle, Stroke};
use bevy_prototype_lyon::shapes;
use rand::Rng;

#[derive(Component, Default)]
pub struct DespawnWithScene;

#[derive(Component)]
pub struct ExplodesOnDespawn {
    pub amount_min: u32,
    pub amount_max: u32,
    pub spread: f32,
    pub colour: Color,
    pub duration_min: f32,
    pub duration_max: f32,
    pub size_min: f32,
    pub size_max: f32,
}

impl Default for ExplodesOnDespawn {
    fn default() -> Self {
        ExplodesOnDespawn {
            amount_min: 1,
            amount_max: 1,
            colour: Colour::RED,
            duration_min: 0.3,
            duration_max: 0.4,
            size_min: 40.0,
            size_max: 40.0,
            spread: 10.0,
        }
    }
}

#[derive(Resource, Default)]
pub struct GameTime(pub Stopwatch);

#[derive(Resource)]
pub struct PlayerLevel {
    pub value: u32,
}

impl PlayerLevel {
    pub fn required_cargo_to_level(&self) -> u32 {
        self.value * 4 // TODO make exponential?
    }
}

#[derive(Component, Copy, Clone)]
pub struct Damage {
    pub amount: i32,
    pub is_crit: bool,
}

#[derive(Event)]
pub struct TakeDamageEvent {
    pub entity: Entity,
    pub damage: Damage,
}

#[derive(PartialEq)]
pub enum Allegiance {
    Friend,
    Enemy,
}

#[derive(Component)]
pub struct Targettable(pub Allegiance);

impl Default for Targettable {
    fn default() -> Self {
        Targettable(Allegiance::Enemy)
    }
}

#[derive(Component)]
pub struct WillTarget(pub Vec<Allegiance>);

impl Default for WillTarget {
    fn default() -> Self {
        WillTarget(vec![Allegiance::Friend])
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_event::<TakeDamageEvent>()
        .add_systems(OnEnter(AppState::InGame), setup_new_game);
    app.add_systems(OnExit(AppState::InGame), reset_game);
    app.add_systems(
        Update,
        (
            game_time_system,
            camera_follow.before(ParallaxSystems),
            combat_system,
            take_damage_events,
            death_system,
        )
            .chain()
            .in_set(AppSet::TickTimers)
            .distributive_run_if(game_not_paused)
            .distributive_run_if(in_state(AppState::InGame)),
    );
}
fn setup_new_game(mut commands: Commands) {
    // Set the start time
    commands.insert_resource(GameTime::default());

    // Create point count
    commands.insert_resource(Points { value: 0 });

    // Start player at level 0 so they get immediate selection
    commands.insert_resource(PlayerLevel { value: 0 });
}

pub fn game_not_paused(game_state: Res<State<GameState>>) -> bool {
    *game_state.get() != GameState::Paused && *game_state.get() != GameState::Selection
}

fn game_time_system(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    game_time.0.tick(time.delta());
}

fn reset_game(
    mut commands: Commands,
    query: Query<Entity, With<DespawnWithScene>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    next_game_state.set(GameState::Running);
}

pub fn camera_follow(
    time: Res<Time>,
    player_q: Query<&Transform, (With<Transform>, With<IsPlayer>, Without<MainCamera>)>,
    mut camera_q: Query<
        (Entity, &Transform, &mut CameraShake),
        (With<Transform>, With<MainCamera>, Without<IsPlayer>),
    >,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
) {
    if let Ok((camera_entity, camera_transform, mut shake)) = camera_q.get_single_mut() {
        if let Ok(player_transform) = player_q.get_single() {
            // Calculate the new camera position based on the player's position
            let target_position = Vec2::new(
                player_transform.translation.x + 1.0,
                player_transform.translation.y,
            );

            let current_position = camera_transform.translation.truncate();

            let smooth_move_position = current_position
                .lerp(target_position, 5.0 * time.delta_seconds())
                + shake.trauma * Math::random_2d_unit_vector();

            shake.trauma = f32::max(shake.trauma - shake.decay * time.delta_seconds(), 0.0);

            move_event_writer.send(ParallaxMoveEvent {
                translation: smooth_move_position - current_position,
                rotation: 0.0,
                camera: camera_entity,
            });
        }
    }
}

pub fn combat_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Health, Entity), Without<ShouldDespawn>>,
    sound_assets: Res<SoundAssets>,
) {
    for (mut health, entity) in &mut query {
        if health.health <= 0 {
            commands.entity(entity).insert(ShouldDespawn);
            continue;
        }

        // Recharge shield
        health.shield_recharge_cooldown.tick(time.delta());
        if health.shield_recharge_cooldown.finished() {
            health.shield_recharge_timer.tick(time.delta());
            if health.shield_recharge_timer.just_finished() {
                if health.shield == health.max_shield {
                    return;
                }
                health.shield += 1;
                //播放增加护盾的音效
                play_sound_effects(&mut commands, sound_assets.shield_up.clone());
            }
        }
    }
}

pub fn take_damage_events(
    mut commands: Commands,
    fonts: Res<Fonts>,
    mut take_damage_events: EventReader<TakeDamageEvent>,
    mut query: Query<(
        &Transform,
        &mut Health,
        Option<&IsPlayer>,
        Option<&mut HitFlash>,
    )>,
    mut camera: Query<&mut CameraShake>,
    sound_assets: Res<SoundAssets>,
) {
    for ev in take_damage_events.read() {
        if let Ok((transform, mut health, is_player, hit_flash)) = query.get_mut(ev.entity) {
            health.take_damage(ev.damage.amount);

            //玩家受击时带有相机抖动效果
            if is_player.is_some() {
                if let Ok(mut shake) = camera.get_single_mut() {
                    shake.trauma = ev.damage.amount.clamp(0, 5) as f32;
                }
                //播放玩家被击中音效
                play_sound_effects(&mut commands, sound_assets.bullet_hit_1.clone());
            } else {
                //播放敌人被击中音效
                play_sound_effects(&mut commands, sound_assets.bullet_hit_2.clone());
                // Floating Text
                commands.spawn((
                    FloatingText::default(),
                    Text2dBundle {
                        text: Text::from_section(
                            format!("{}", ev.damage.amount),
                            TextStyle {
                                font: fonts.primary.clone(),
                                font_size: if ev.damage.is_crit { 14.0 } else { 12.0 },
                                color: if ev.damage.is_crit {
                                    Colour::YELLOW
                                } else {
                                    Colour::WHITE
                                },
                            },
                        )
                        .with_justify(JustifyText::Center),
                        transform: Transform::from_xyz(
                            transform.translation.x,
                            transform.translation.y + 10.0,
                            RenderLayer::Effects.as_z(),
                        ),
                        ..default()
                    },
                ));
            }

            if let Some(mut hit_flash) = hit_flash {
                hit_flash.hit();
            }
        }
    }
}

pub fn death_system(
    mut commands: Commands,
    fonts: Res<Fonts>,
    mut query: Query<
        (
            Entity,
            Option<&DropsLoot>,
            Option<&Transform>,
            Option<&IsPlayer>,
            Option<&ExplodesOnDespawn>,
            Option<&WorthPoints>,
        ),
        With<ShouldDespawn>,
    >,
    mut game_state: ResMut<NextState<GameState>>,
    mut points: ResMut<Points>,
    sound_assets: Res<SoundAssets>,
) {
    for (entity, drops_loot, transform, is_player, explodes, worth_points) in &mut query {
        commands.entity(entity).despawn_recursive();

        if let Some(transform) = transform {
            if let Some(_drops_loot) = drops_loot {
                spawn_loot(&mut commands, &fonts, transform.translation);
            }
            if let Some(explodes) = explodes {
                explode(&mut commands, explodes, transform.translation.truncate());
            }
        }

        if let Some(worth_points) = worth_points {
            points.value += worth_points.value;
        }

        if is_player.is_some() {
            //播放失败音效
            play_sound_effects(&mut commands, sound_assets.lose.clone());
            game_state.set(GameState::GameOver);
        }
    }
}

fn spawn_loot(commands: &mut Commands, fonts: &Res<Fonts>, position: Vec3) {
    let mut rng = rand::thread_rng();
    let loots = (0..rng.gen_range(1..=3))
        .map(|_| {
            (
                IsLoot,
                Text2dBundle {
                    text: Text::from_section(
                        "*",
                        TextStyle {
                            font: fonts.primary.clone(),
                            font_size: 12.0,
                            color: Colour::PURPLE,
                        },
                    )
                    .with_justify(JustifyText::Center),
                    transform: Transform::from_translation(position),
                    ..Default::default()
                },
                Physics {
                    acceleration: Vec2 {
                        x: rng.gen_range(-1.0..1.0),
                        y: rng.gen_range(-1.0..1.0),
                    }
                    .normalize_or_zero()
                        * rng.gen_range(50.0..100.0),
                    drag: 1.0,
                    ..Default::default()
                },
                Collider { radius: 20.0 },
                DespawnWithScene,
                WorthPoints { value: 1 },
            )
        })
        .collect::<Vec<_>>();
    commands.spawn_batch(loots);
}

fn explode(commands: &mut Commands, explodes: &ExplodesOnDespawn, position: Vec2) {
    // Spawn several explosions
    let mut rng = rand::thread_rng();
    let amount = rng.gen_range(explodes.amount_min..=explodes.amount_max);
    for _ in 0..amount {
        let offset = Vec2 {
            x: rng.gen_range(-explodes.spread..=explodes.spread),
            y: rng.gen_range(-explodes.spread..=explodes.spread),
        };
        commands.spawn((
            ExplosionRender {
                origin: position + offset,
                radius: rng.gen_range(explodes.size_min..=explodes.size_max),
                ttl: Timer::from_seconds(
                    rng.gen_range(explodes.duration_min..=explodes.duration_max),
                    TimerMode::Once,
                ),
                fade_out: false,
            },
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    center: position,
                    radius: 0.0,
                }),
                spatial: SpatialBundle::from_transform(Transform::from_xyz(
                    0.,
                    0.,
                    RenderLayer::Effects.as_z(),
                )),
                ..default()
            },
            Stroke::new(explodes.colour, 1.0),
        ));
    }
}
