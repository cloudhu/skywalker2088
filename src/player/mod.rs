//! The game's main player states and transitions between them.

pub mod batman;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<PlayerState>();
    app.enable_state_scoped_entities::<PlayerState>();

    app.add_plugins((batman::plugin,));
}

/// The game's main player states.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum PlayerState {
    #[default]
    Running,
    Selection,
    Paused,
    GameOver,
}
