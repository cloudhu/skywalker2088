use crate::assets::game_assets::Fonts;
use crate::components::events::TakeDamageEvent;
use crate::components::game::{ExplosionRender, ShouldDespawn};
use crate::components::health::HealthComponent;
use crate::components::player::{PlayerComponent, PlayersResource};
use crate::components::spawnable::{EffectType, TextEffectType};
use crate::components::states::*;
use crate::gameplay::effects::{FloatingText, HitFlash};
use crate::gameplay::loot::{DropsLoot, IsLoot, Points, WorthPoints};
use crate::gameplay::physics::{Collider, Physics};
use crate::spawnable::SpawnEffectEvent;
use crate::util::{Colour, Math, RenderLayer};
use crate::AppSet;
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
    pub fn required_cargo_to_level(&self) -> usize {
        (self.value * 4) as usize // TODO make exponential?
    }
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
        .add_systems(OnEnter(AppStates::Game), setup_new_game);
    app.add_systems(OnExit(AppStates::Game), reset_game);
    app.add_systems(
        Update,
        (
            // game_time_system,
            camera_follow.before(ParallaxSystems),
            shield_recharge_system,
            take_damage_events,
            drop_loot_system,
        )
            .chain()
            .in_set(AppSet::TickTimers)
            .distributive_run_if(game_not_paused)
            .distributive_run_if(in_state(AppStates::Game)),
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

pub fn game_not_paused(game_state: Res<State<GameStates>>) -> bool {
    *game_state.get() != GameStates::Paused && *game_state.get() != GameStates::Selection
}

fn game_time_system(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    game_time.0.tick(time.delta());
}

fn reset_game(
    mut commands: Commands,
    query: Query<Entity, With<DespawnWithScene>>,
    mut next_game_state: ResMut<NextState<GameStates>>,
    mut players_resource: ResMut<PlayersResource>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    *players_resource = PlayersResource::default();
    next_game_state.set(GameStates::Playing);
}

pub fn camera_follow(
    time: Res<Time>,
    player_q: Query<&Transform, (With<Transform>, With<PlayerComponent>)>,
    mut camera_q: Query<(Entity, &Transform), With<Camera2d>>,
    mut move_event_writer: EventWriter<ParallaxMoveEvent>,
) {
    if let Ok((camera_entity, camera_transform)) = camera_q.get_single_mut() {
        if let Ok(player_transform) = player_q.get_single() {
            // Calculate the new camera position based on the player's position
            let target_position = Vec2::new(
                player_transform.translation.x + 1.0,
                player_transform.translation.y,
            );

            let current_position = camera_transform.translation.truncate();

            let smooth_move_position = current_position
                .lerp(target_position, 5.0 * time.delta_seconds())
                + Math::random_2d_unit_vector();

            move_event_writer.send(ParallaxMoveEvent {
                translation: smooth_move_position - current_position,
                rotation: 0.0,
                camera: camera_entity,
            });
        }
    }
}

pub fn shield_recharge_system(
    time: Res<Time>,
    mut query: Query<(&mut HealthComponent, Entity), With<PlayerComponent>>,
) {
    for (mut health, entity) in &mut query {
        if health.get_health() <= 0 {
            continue;
        }

        // Recharge shield
        health.shield_recharge_cooldown.tick(time.delta());
        if health.shield_recharge_cooldown.finished() {
            health.regenerate_shields(time.delta());
        }
    }
}

pub fn take_damage_events(
    mut commands: Commands,
    fonts: Res<Fonts>,
    mut take_damage_events: EventReader<TakeDamageEvent>,
    mut query: Query<(&Transform, &mut HealthComponent, Option<&mut HitFlash>)>,
    mut spawn_effect_event_writer: EventWriter<SpawnEffectEvent>,
) {
    for ev in take_damage_events.read() {
        if let Ok((transform, mut health, hit_flash)) = query.get_mut(ev.target) {
            // take damage from health
            health.take_damage(ev.damage.amount);
            // spawn damage dealt text effect
            spawn_effect_event_writer.send(SpawnEffectEvent {
                effect_type: EffectType::Text(TextEffectType::DamageDealt),
                transform: Transform {
                    translation: transform.translation,
                    scale: transform.scale,
                    ..Default::default()
                },
                text: Some(ev.damage.amount.to_string()),
                ..Default::default()
            });

            if let Some(mut hit_flash) = hit_flash {
                hit_flash.hit();
            }
            //TODO:to value which floating text is better,use the better one and delete the other one
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
    }
}

pub fn drop_loot_system(
    mut commands: Commands,
    fonts: Res<Fonts>,
    mut query: Query<
        (
            Option<&Transform>,
            Option<&ExplodesOnDespawn>,
            Option<&WorthPoints>,
            Option<&DropsLoot>
        ),
        With<HealthComponent>,
    >,
    mut points: ResMut<Points>,
) {
    for (transform, explodes, worth_points,drops_loot) in &mut query {
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
