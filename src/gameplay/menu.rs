use crate::gameplay::loot::Points;
use crate::gameplay::GameState;
use crate::ship::platform::Fonts;
use bevy::prelude::*;

#[derive(Resource, Default)]
struct MenuData {
    main: Option<Entity>,
    pause: Option<Entity>,
    game_over: Option<Entity>,
}

#[derive(Component)]
struct MenuButton(pub ButtonAction);

enum ButtonAction {
    Play,
    Exit,
    ToTitle,
}

const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(MenuData::default())
        .add_systems(OnEnter(GameState::Paused), setup_paused)
        .add_systems(OnExit(GameState::Paused), cleanup_pause)
        .add_systems(OnEnter(GameState::GameOver), setup_game_over)
        .add_systems(OnExit(GameState::GameOver), cleanup_game_over);
}

fn cleanup(mut commands: Commands, root_entity: &mut Option<Entity>) {
    if let Some(entity) = *root_entity {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
        *root_entity = None;
    }
}

fn cleanup_menu(commands: Commands, mut menu_data: ResMut<MenuData>) {
    cleanup(commands, &mut menu_data.main);
}

fn cleanup_pause(commands: Commands, mut menu_data: ResMut<MenuData>) {
    cleanup(commands, &mut menu_data.pause);
}

fn cleanup_game_over(commands: Commands, mut menu_data: ResMut<MenuData>) {
    cleanup(commands, &mut menu_data.game_over);
}

fn button(parent: &mut ChildBuilder, fonts: &Res<Fonts>, text: &str, action: ButtonAction) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    min_width: Val::Px(200.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            MenuButton(action),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 40.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

fn setup_paused(mut commands: Commands, fonts: Res<Fonts>, mut menu_data: ResMut<MenuData>) {
    let root_entity = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                padding: UiRect::top(Val::Px(10.0)),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Paused".to_owned(),
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 30.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ));
            parent.spawn(TextBundle::from_section(
                "Click <Escape> To Resume".to_owned(),
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 16.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .id();
    menu_data.pause = Some(root_entity);
}

fn setup_game_over(
    mut commands: Commands,
    fonts: Res<Fonts>,
    mut menu_data: ResMut<MenuData>,
    points: Res<Points>,
) {
    let root_entity = commands
        .spawn(NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("{} points!", points.into_inner()),
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 30.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ));
            button(parent, &fonts, "Return To Title", ButtonAction::ToTitle);
        })
        .id();
    menu_data.game_over = Some(root_entity);
}
