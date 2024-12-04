//! The title screen that appears when the game starts.

use crate::assets::audio_assets::{Fonts, Music};
use crate::audio::NextBgm;
use crate::components::character::CharacterType;
use crate::components::events::{ButtonActionEvent, PlayerJoinEvent};
use crate::components::input::{
    ButtonActionComponent, ButtonActionType, MainMenuExplorer, MenuAction,
};
use crate::components::player::{PlayerData, PlayerInput, PlayersResource};
use crate::config::GameConfig;
use crate::{screens::AppStates, theme::prelude::*};
use bevy::input::gamepad::GamepadButtonChangedEvent;
use bevy::prelude::*;
use bevy::window::WindowMode;
use leafwing_input_manager::action_state::ActionState;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<PlayerJoinEvent>();
    app.add_event::<ButtonActionEvent>();
    app.add_systems(OnEnter(AppStates::MainMenu), spawn_title_screen);
    app.add_systems(OnEnter(AppStates::MainMenu), play_title_music);
    app.add_systems(OnExit(AppStates::MainMenu), stop_title_music);
    app.add_systems(
        Update,
        (player_join_system,).run_if(in_state(AppStates::MainMenu)),
    );
}

fn spawn_title_screen(
    mut commands: Commands,
    mut localize: ResMut<Localize>,
    config: Res<GameConfig>,
    fonts: Res<Fonts>,
) {
    localize.set_language(config.language.clone());
    commands
        .ui_root()
        .insert(StateScoped(AppStates::MainMenu))
        .with_children(|children| {
            children
                .button("Play", fonts.primary.clone())
                .observe(enter_gameplay_screen);
            children
                .button("Credits", fonts.primary.clone())
                .observe(enter_credits_screen);
            children
                .button("Duolingo", fonts.primary.clone())
                .observe(set_lang);
            children
                .button("Toggle Fullscreen", fonts.primary.clone())
                .observe(toggle_fullscreen);
            #[cfg(not(target_family = "wasm"))]
            children
                .button("Exit", fonts.primary.clone())
                .observe(exit_app);
        });
}

fn enter_gameplay_screen(
    _trigger: Trigger<OnPress>,
    mut next_screen: ResMut<NextState<AppStates>>,
) {
    next_screen.set(AppStates::Game);
}

fn enter_credits_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<AppStates>>) {
    next_screen.set(AppStates::Credits);
}

fn set_lang(
    _trigger: Trigger<OnPress>,
    mut config: ResMut<GameConfig>,
    mut localize: ResMut<Localize>,
) {
    match config.language.as_str() {
        "English" => {
            config.set_lang("Chinese");
        }
        "Chinese" => {
            config.set_lang("English");
        }
        &_ => {}
    }
    localize.set_language(config.language.clone());
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_trigger: Trigger<OnPress>, mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit::Success);
}

fn play_title_music(mut next_bgm: ResMut<NextBgm>, music: Res<Music>) {
    *next_bgm = NextBgm(Some(music.title.clone()));
}

fn stop_title_music(mut next_bgm: ResMut<NextBgm>) {
    *next_bgm = NextBgm(None);
}

fn toggle_fullscreen(_trigger: Trigger<OnPress>, mut window_query: Query<&mut Window>) {
    let mut window = window_query.single_mut();
    window.mode = match window.mode {
        WindowMode::Windowed => WindowMode::SizedFullscreen,
        WindowMode::BorderlessFullscreen => WindowMode::Windowed,
        WindowMode::SizedFullscreen => WindowMode::Windowed,
        WindowMode::Fullscreen => WindowMode::Windowed,
    };
}

/// Handles player joining events in the game.
///
/// This function detects player join actions through keyboard, gamepad, or mouse inputs,
/// updates the players resource, and sends appropriate events.
fn player_join_system(
    button_mouse_movements: Query<(&ButtonActionComponent, &Interaction, Entity), With<Button>>,
    menu_explorer_query: Query<&ActionState<MenuAction>, With<MainMenuExplorer>>,
    mut button_event_writer: EventWriter<ButtonActionEvent>,
    mut mouse_interaction: Local<Interaction>,
    mut players_resource: ResMut<PlayersResource>,
    mut gamepad_events: EventReader<GamepadButtonChangedEvent>,
    mut player_join_event: EventWriter<PlayerJoinEvent>,
) {
    // Check if the join button was pressed
    if let Some(button) = button_mouse_movements.iter().find(|(button_action, _, _)| {
        matches!(button_action.action, ButtonActionType::CharacterSelectJoin)
    }) {
        // Get the list of currently used inputs
        let used_inputs = players_resource.get_used_inputs();

        // Check if the maximum number of players have already joined
        if used_inputs.len() < 2 {
            // Detect if a player is joining through a keyboard button press
            if let Some(player_input) = match menu_explorer_query.get_single() {
                Err(_) => None,
                Ok(action) => {
                    if action
                        .get_just_released()
                        .iter()
                        .any(|action_| match action_ {
                            MenuAction::JoinKeyboard => {
                                !used_inputs.contains(&PlayerInput::Keyboard)
                            }
                            _ => false,
                        })
                    {
                        Some(PlayerInput::Keyboard)
                    } else {
                        None
                    }
                }
            } {
                // Push the new player to the players resource
                players_resource.player_data.push(Some(PlayerData {
                    character: CharacterType::default(),
                    input: player_input,
                }));

                // Send player join event and button action event
                button_event_writer.send(ButtonActionEvent::from(
                    ButtonActionType::CharacterSelectJoin,
                ));
                player_join_event.send(PlayerJoinEvent {
                    player_idx: players_resource.player_data.len() as u8 - 1,
                    input: player_input,
                });
            }

            // Detect if a player is joining through a gamepad button press
            if let Some(player_input) = match menu_explorer_query.get_single() {
                Err(_) => None,
                Ok(action) => {
                    if let Some(gamepad_event) = gamepad_events.read().next() {
                        if action
                            .get_just_released()
                            .iter()
                            .any(|action| match action {
                                MenuAction::JoinGamepad => !used_inputs
                                    .contains(&PlayerInput::Gamepad(gamepad_event.gamepad.id)),
                                _ => false,
                            })
                        {
                            Some(PlayerInput::Gamepad(gamepad_event.gamepad.id))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            } {
                // Push the new player to the players resource
                players_resource.player_data.push(Some(PlayerData {
                    character: CharacterType::default(),
                    input: player_input,
                }));

                // Send player join event and button action event
                button_event_writer.send(ButtonActionEvent::from(
                    ButtonActionType::CharacterSelectJoin,
                ));
                player_join_event.send(PlayerJoinEvent {
                    player_idx: players_resource.player_data.len() as u8 - 1,
                    input: player_input,
                });
            }

            // Detect if a player is joining through a mouse button release
            if let Some(player_input) = match button.1 {
                // Check if mouse interaction changed from Pressed to Hovered
                // which means the player just released the mouse button over the UI button
                Interaction::Hovered => match *mouse_interaction {
                    Interaction::Pressed => {
                        if !used_inputs.contains(&PlayerInput::Keyboard) {
                            Some(PlayerInput::Keyboard)
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
                _ => None,
            } {
                // Push the new player to the players resource
                players_resource.player_data.push(Some(PlayerData {
                    character: CharacterType::default(),
                    input: player_input,
                }));

                // Send player join event and button action event
                button_event_writer.send(ButtonActionEvent::from(
                    ButtonActionType::CharacterSelectJoin,
                ));
                player_join_event.send(PlayerJoinEvent {
                    player_idx: players_resource.player_data.len() as u8 - 1,
                    input: player_input,
                });
            }

            // Track the current mouse interaction in a local variable
            *mouse_interaction = *button.1;
        }
    }
}
