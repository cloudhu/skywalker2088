use crate::assets::audio_assets::Fonts;
use crate::gameplay::gamelogic::PlayerLevel;
use crate::gameplay::upgrade::{PlayerUpgrades, UpgradeEvent};
use crate::gameplay::GameStates;
use crate::ship::turret::TurretClass;
use crate::theme::language::Localize;
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
        .add_systems(OnEnter(GameStates::Selection), setup_selection)
        .add_systems(Update, menu.run_if(in_state(GameStates::Selection)))
        .add_systems(OnExit(GameStates::Selection), cleanup);
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
    localize: Res<Localize>,
) {
    // Roll for options
    let options = match player_level.value {
        1 => roll_starting(),
        _ => roll(upgrades),
    };

    let root_entity = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(10.0),
            ..default()
        })
        .with_children(|parent| {
            for option in options {
                button(parent, &fonts, option, &localize);
            }
        })
        .id();
    menu_data.0.push(root_entity);
}

fn menu(
    mut next_state: ResMut<NextState<GameStates>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &SelectionButton),
        (Changed<Interaction>, With<Button>, With<SelectionButton>),
    >,
    mut upgrade_event: EventWriter<UpgradeEvent>,
) {
    for (interaction, mut color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                upgrade_event.send(button.0);
                next_state.set(GameStates::Playing);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
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

fn button(
    parent: &mut ChildBuilder,
    fonts: &Res<Fonts>,
    upgrade: UpgradeEvent,
    localize: &Res<Localize>,
) {
    let type_text = match upgrade {
        UpgradeEvent::Weapon(_) => localize.get("Weapon"),
        UpgradeEvent::Passive(_) => localize.get("Passive"),
        UpgradeEvent::Heal => localize.get("Consumable"),
    };
    let type_color = match upgrade {
        UpgradeEvent::Weapon(_) => Colour::RED,
        UpgradeEvent::Passive(_) => Colour::SHIELD,
        UpgradeEvent::Heal => Colour::GREEN,
    };
    // info!("{}",format!("{}", upgrade));
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(300.0),
                height: Val::Px(140.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                column_gap: Val::Px(10.0),
                ..default()
            },
            SelectionButton(upgrade),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(type_text),
                TextFont {
                    font: fonts.primary.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor::from(type_color),
                Node {
                    top: Val::Px(10.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
            ));
            parent.spawn((
                Text::new(localize.get(format!("{}", upgrade).as_str())),
                TextFont {
                    font: fonts.primary.clone(),
                    font_size: 24.0,
                    ..default()
                },
                TextColor::from(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    top: Val::Px(30.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
            ));
            parent.spawn((
                Text::new(localize.get(&upgrade.describe())),
                TextFont {
                    font: fonts.primary.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor::from(Color::srgba(0.8, 0.8, 0.8, 0.8)),
                TextLayout {
                    justify: JustifyText::Center,
                    ..default()
                },
                Node {
                    bottom: Val::Px(20.0),
                    position_type: PositionType::Absolute,
                    margin: UiRect {
                        left: Val::Px(15.0),
                        ..Default::default()
                    },
                    width: Val::Px(200.0),
                    ..default()
                },
            ));
        });
}
