use crate::audio::SoundEffect;
use crate::gameplay::gamelogic::PlayerLevel;
use crate::gameplay::upgrade::{PlayerUpgrades, UpgradeEvent};
use crate::gameplay::GameState;
use crate::ship::platform::Fonts;
use crate::ship::turret::TurretClass;
use crate::theme::interaction::InteractionAssets;
use crate::util::Colour;
use bevy::prelude::*;
use rand::Rng;

#[derive(Resource)]
struct SelectionData(pub Vec<Entity>);

#[derive(Component)]
struct SelectionButton(UpgradeEvent);

const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(SelectionData(vec![]))
        .add_systems(OnEnter(GameState::Selection), setup_selection)
        .add_systems(Update, menu.run_if(in_state(GameState::Selection)))
        .add_systems(OnExit(GameState::Selection), cleanup);
}

fn random_starting_weapon() -> TurretClass {
    match rand::thread_rng().gen_range(0..4) {
        0 => TurretClass::AutoCannon,
        1 => TurretClass::BlastLaser,
        2 => TurretClass::RocketLauncher,
        _ => TurretClass::MineLauncher,
    }
}

fn roll_starting() -> Vec<UpgradeEvent> {
    let mut options: Vec<UpgradeEvent> = vec![];
    while options.len() < 3 {
        let potential = UpgradeEvent::Weapon(random_starting_weapon());
        if !options.contains(&potential) {
            options.push(potential);
        }
    }
    options
}

fn roll(upgrades: Res<PlayerUpgrades>) -> Vec<UpgradeEvent> {
    let mut options: Vec<UpgradeEvent> = vec![];
    let mut iterations = 0;
    while options.len() < 3 {
        iterations += 1;

        if iterations > 100 {
            options.push(UpgradeEvent::Heal);
            continue;
        }

        let potential: UpgradeEvent = rand::random();
        // No duplicates
        if options.contains(&potential) {
            continue;
        }

        let current_level = upgrades.0.get(&potential).unwrap_or(&0);

        // Can't go above max level
        if *current_level >= PlayerUpgrades::max_allowed_level() {
            continue;
        }

        // Cannot have too many passives or weapons
        let cap_reached = match potential {
            UpgradeEvent::Weapon(_) => upgrades.reached_max_weapons(),
            UpgradeEvent::Passive(_) => upgrades.reached_max_passives(),
            UpgradeEvent::Heal => false,
        };
        // If new check we haven't had too many of given type
        if current_level == &0 && cap_reached {
            continue;
        }

        options.push(potential);
    }
    options
}

fn setup_selection(
    mut commands: Commands,
    fonts: Res<Fonts>,
    mut menu_data: ResMut<SelectionData>,
    player_level: Res<PlayerLevel>,
    upgrades: Res<PlayerUpgrades>,
) {
    // Roll for options
    let options = match player_level.value {
        1 => roll_starting(),
        _ => roll(upgrades),
    };

    let root_entity = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                ..Default::default()
            },
            ..default()
        })
        .with_children(|parent| {
            for option in options {
                button(parent, &fonts, option);
            }
        })
        .id();
    menu_data.0.push(root_entity);
}

fn menu(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &SelectionButton),
        (Changed<Interaction>, With<Button>, With<SelectionButton>),
    >,
    mut upgrade_event: EventWriter<UpgradeEvent>,
    interaction_assets: Res<InteractionAssets>,
    mut commands: Commands,
) {
    for (interaction, mut color, button) in &mut interaction_query {
        let mut source = interaction_assets.press.clone();
        match *interaction {
            Interaction::Pressed => {
                upgrade_event.send(button.0);
                next_state.set(GameState::Running);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                source = interaction_assets.hover.clone();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
        commands.spawn((
            AudioBundle {
                source,
                settings: PlaybackSettings::DESPAWN,
            },
            SoundEffect,
        ));
    }
}

fn cleanup(mut commands: Commands, mut menu_data: ResMut<SelectionData>) {
    for entity in menu_data.0.iter() {
        if let Some(entity) = commands.get_entity(*entity) {
            entity.despawn_recursive();
        }
    }
    menu_data.0.clear();
}

fn button(parent: &mut ChildBuilder, fonts: &Res<Fonts>, upgrade: UpgradeEvent) {
    let type_text = match upgrade {
        UpgradeEvent::Weapon(_) => "Weapon".to_string(),
        UpgradeEvent::Passive(_) => "Passive".to_string(),
        UpgradeEvent::Heal => "Consumable".to_string(),
    };
    let type_color = match upgrade {
        UpgradeEvent::Weapon(_) => Colour::RED,
        UpgradeEvent::Passive(_) => Colour::SHIELD,
        UpgradeEvent::Heal => Colour::GREEN,
    };
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(300.0),
                    height: Val::Px(140.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.0)),
                    column_gap: Val::Px(10.0),
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            SelectionButton(upgrade),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    type_text,
                    TextStyle {
                        font: fonts.primary.clone(),
                        font_size: 14.0,
                        color: type_color,
                    },
                ),
                style: Style {
                    top: Val::Px(10.0),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                ..Default::default()
            });
            parent.spawn(TextBundle {
                text: Text::from_section(
                    format!("{}", upgrade),
                    TextStyle {
                        font: fonts.primary.clone(),
                        font_size: 24.0,
                        color: Color::srgb(0.9, 0.9, 0.9),
                    },
                ),
                style: Style {
                    top: Val::Px(30.0),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                ..Default::default()
            });
            parent.spawn(TextBundle {
                text: Text::from_section(
                    upgrade.describe().to_string(),
                    TextStyle {
                        font: fonts.primary.clone(),
                        font_size: 14.0,
                        color: Color::srgba(0.8, 0.8, 0.8, 0.8),
                    },
                )
                .with_justify(JustifyText::Center),
                style: Style {
                    bottom: Val::Px(20.0),
                    position_type: PositionType::Absolute,
                    margin: UiRect {
                        left: Val::Px(15.0),
                        ..Default::default()
                    }, // Text wrapping kind of sucks in bevy...
                    width: Val::Px(200.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}
