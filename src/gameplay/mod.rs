//! The game's main gameplay states and transitions between them.

pub mod control;
pub mod effects;
pub mod gamelogic;
mod hud;
pub mod level;
pub mod loot;
mod object;
pub mod physics;
mod selection;
mod upgrade;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        gamelogic::plugin,
        physics::plugin,
        effects::plugin,
        loot::plugin,
        // selection::plugin,
        // upgrade::plugin,
        // object::plugin,
        // hud::plugin,
        // control::plugin,
    ));
}
