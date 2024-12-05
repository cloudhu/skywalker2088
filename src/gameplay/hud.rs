use crate::assets::audio_assets::Fonts;
use crate::components::health::HealthComponent;
use crate::gameplay::gamelogic::{DespawnWithScene, GameTime, PlayerLevel};
use crate::gameplay::loot::Cargo;
use crate::gameplay::player::PlayerComponent;
use crate::gameplay::upgrade::PlayerUpgrades;
use crate::screens::AppStates;
use crate::ship::engine::Engine;
use crate::ship::turret::{FireRate, TurretClass};
use crate::theme::language::Localize;
use crate::util::Colour;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppStates::Game), setup_hud)
        // Always run while game is running
        .add_systems(Update, hud_system.run_if(in_state(AppStates::Game)));
}

#[derive(Component)]
pub enum UINode {
    Status,
    Equipment,
    Upgrades,
}

// Spawn the hud
fn setup_hud(mut commands: Commands, fonts: Res<Fonts>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(20.0),
                    height: Val::Percent(20.0),
                    margin: UiRect::all(Val::Px(5.0)),
                    column_gap: Val::Px(2.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            UINode::Status,
            DespawnWithScene,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 12.0,
                    color: Colour::WHITE,
                },
            ));
            parent.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 12.0,
                    color: Colour::SHIELD,
                },
            ));
            parent.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 12.0,
                    color: Colour::RED,
                },
            ));
            parent.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 12.0,
                    color: Colour::INACTIVE,
                },
            ));
            parent.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 12.0,
                    color: Colour::PLAYER,
                },
            ));
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(0.0),
                    width: Val::Percent(20.0),
                    height: Val::Percent(20.0),
                    margin: UiRect::all(Val::Px(5.0)),
                    column_gap: Val::Px(2.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::End,
                    ..default()
                },
                ..default()
            },
            UINode::Equipment,
            DespawnWithScene,
        ))
        .with_children(|parent| {
            for _ in 0..10 {
                parent.spawn(TextBundle::from_section(
                    "",
                    TextStyle {
                        font: fonts.primary.clone(),
                        font_size: 12.0,
                        color: Colour::WHITE,
                    },
                ));
            }
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    width: Val::Percent(20.0),
                    height: Val::Percent(20.0),
                    margin: UiRect::all(Val::Px(5.0)),
                    column_gap: Val::Px(2.0),
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::End,
                    ..default()
                },
                ..default()
            },
            UINode::Upgrades,
            DespawnWithScene,
        ))
        .with_children(|parent| {
            for _ in 0..10 {
                parent.spawn(TextBundle::from_section(
                    "",
                    TextStyle {
                        font: fonts.primary.clone(),
                        font_size: 12.0,
                        color: Colour::WHITE,
                    },
                ));
            }
        });
}

fn bar(current: usize, max: usize, width: usize) -> String {
    if max == 0 {
        return String::from(' ').repeat(width);
    }
    let bars: usize = (current.clamp(0, max) * width / max)
        .try_into()
        .unwrap_or(0);
    format!(
        "{}{}",
        String::from('|').repeat(bars),
        String::from('.').repeat(width - bars)
    )
}

pub fn hud_system(
    upgrades: Res<PlayerUpgrades>,
    player_query: Query<(&Engine, &HealthComponent, &Cargo, &Children), With<PlayerComponent>>,
    turret_query: Query<(&FireRate, &TurretClass)>,
    mut query: Query<(&Children, &UINode)>,
    mut q_child: Query<&mut Text>,
    level: Res<PlayerLevel>,
    game_time: Res<GameTime>,
    localize: Res<Localize>,
) {
    if let Ok((engine, health, cargo, turrets)) = player_query.get_single() {
        // Loop over children and update display values
        for (children, ui_node) in &mut query {
            let displays = match ui_node {
                UINode::Status => vec![
                    format!(
                        "{:<8} {} {}",
                        localize.get("Armor"),
                        bar(health.health, health.max_health, 10),
                        health.health
                    ),
                    format!(
                        "{:<8} {} {}",
                        localize.get("Shield"),
                        bar(health.shields, health.max_shields, 10),
                        health.shields
                    ),
                    format!(
                        "{:<8} {} {:0>2}",
                        localize.get("Level"),
                        bar(cargo.amount, level.required_cargo_to_level(), 10),
                        level.value
                    ),
                    format!("{:<8} {} m/s", localize.get("Speed"), engine.speed.round()),
                    format!(
                        "{:<8} {:0>2}:{:0>2}",
                        localize.get("Time"),
                        game_time.0.elapsed().as_secs() / 60,
                        game_time.0.elapsed().as_secs() % 60
                    ),
                ],
                UINode::Equipment => {
                    let mut display = turrets
                        .iter()
                        .map(|e| turret_query.get(*e))
                        .filter_map(|result| result.ok())
                        .map(|(fire_rate, class)| {
                            format!(
                                "{} {:>16}",
                                bar((fire_rate.timer.fraction() * 10.0).round() as usize, 10, 10),
                                localize.get(format!("{:>16}", class).as_str()),
                            )
                        })
                        .collect::<Vec<String>>();
                    display.resize_with(10, Default::default);
                    display
                }
                UINode::Upgrades => {
                    let mut display = upgrades.display_for_ui(&localize);
                    display.resize_with(10, Default::default);
                    display
                }
            };

            for (i, display) in displays.iter().enumerate() {
                if let Some(&child) = children.get(i) {
                    if let Ok(mut text) = q_child.get_mut(child) {
                        if let Some(section) = text.sections.get_mut(0) {
                            section.value = display.to_string();
                        }
                    }
                }
            }
        }
    }
}
