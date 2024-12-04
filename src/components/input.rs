//! The structures used for knowing which player inputs from controller/keyboard+mouse/etc map to
//! known game actions.
use bevy::prelude::{Component, Reflect, Resource};
use leafwing_input_manager::{prelude::InputMap, Actionlike};
use serde::Deserialize;

/// Used by players to access their matching menu ui
/// has an u8 index matching the player (0-3) for a 4 player game
#[derive(Component)]
pub struct MenuExplorer(pub u8);

/// Shared between all players to access shared ui such as the main and pause menus
#[derive(Component)]
pub struct MainMenuExplorer;

/// The input behaviors from the controller/gamepad available while in the menus.
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect, Deserialize)]
pub enum MenuAction {
    Confirm,
    JoinKeyboard,
    JoinGamepad,
    Back,
    Reset,
    ExitPauseMenu,
    PauseGame,
    NavigateUpKeyboard,
    NavigateDownKeyboard,
    NavigateUpGamepad,
    NavigateDownGamepad,
    NavigateLeftKeyboard,
    NavigateRightKeyboard,
    NavigateLeftGamepad,
    NavigateRightGamepad,
    PlayerReadyKeyboard,
    PlayerReadyGamepad,
}

/// Player actions during the main game/while fighting mobs. Many of these can be simultaneously
/// accepted from the gamepad/controller.
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect, Deserialize)]
pub enum PlayerAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    SlotOneAbility,
    SlotTwoAbility,
}

/// The parsed input/key bindings used for the life of the  entire game. This is read from files/
/// compiled files very early in the game startup since this must exist in the world before we can
/// accept player/user input.
#[derive(Resource, Debug)]
pub struct InputsResource {
    pub menu: InputMap<MenuAction>,
    pub player_keyboard: InputMap<PlayerAction>,
    pub player_gamepad: InputMap<PlayerAction>,
}

/// Event and Component for giving and sending menu buttons actions to move the user from
/// `AppStates::MainMenu` to `AppStates::CharacterSelection`, plus possibly a few digressions and
/// sprinkles.
#[derive(Clone, PartialEq, Eq, Copy, Debug)]
pub enum ButtonActionType {
    CharacterSelectReady(u8),
    CharacterSelectJoin,
    CharacterSelectRight(u8),
    CharacterSelectLeft(u8),
    EnterCharacterSelection,
    EnterOptions,
    EnterCompendium,
    QuitGame,
}

#[derive(Component, Clone, PartialEq, Eq, Copy, Debug)]
pub struct ButtonActionComponent {
    pub action: ButtonActionType,
}

impl From<ButtonActionType> for ButtonActionComponent {
    fn from(value: ButtonActionType) -> Self {
        ButtonActionComponent { action: value }
    }
}
