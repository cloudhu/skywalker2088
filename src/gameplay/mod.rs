//! The game's main gameplay states and transitions between them.

pub mod animation;
pub mod gamelogic;
pub mod level;
pub mod physics;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<GameState>();
    app.enable_state_scoped_entities::<GameState>();

    app.add_plugins((gamelogic::plugin, physics::plugin, animation::plugin));
}

/// The game's gameplay states.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Running,
    Selection,
    Paused,
    GameOver,
}
