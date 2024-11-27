//! The game's main screen states and transitions between them.

mod credits;
mod gameplay;
mod loading;
mod splash;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<AppStates>();
    app.enable_state_scoped_entities::<AppStates>();

    app.add_plugins((
        credits::plugin,
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum AppStates {
    #[default]
    Splash,
    Loading,
    MainMenu,
    CharacterSelection,
    Credits,
    InGame,
}
