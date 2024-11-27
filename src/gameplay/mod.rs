//! The game's main gameplay states and transitions between them.

pub mod effects;
pub mod gamelogic;
mod hud;
pub mod level;
pub mod loot;
mod object;
pub mod physics;
pub mod player;
mod selection;
mod upgrade;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<GameState>();
    app.enable_state_scoped_entities::<GameState>();

    app.add_plugins((
        gamelogic::plugin,
        physics::plugin,
        effects::plugin,
        loot::plugin,
        selection::plugin,
        upgrade::plugin,
        object::plugin,
        hud::plugin,
        player::plugin,
    ));
}

/// The game's gameplay states.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Running,
    Selection,
    Paused,
    GameOver,
    Victory,
}
