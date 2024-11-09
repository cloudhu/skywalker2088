//! The game's ship and components.

pub mod bullet;
pub mod engine;
pub mod platform;
pub mod turret;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<ShipState>();
    app.enable_state_scoped_entities::<ShipState>();

    app.add_plugins((
        platform::plugin,
        engine::plugin,
        bullet::plugin,
        turret::plugin,
    ));
}

/// The game's main ship states.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ShipState {
    #[default]
    Running,
    Selection,
    Paused,
    GameOver,
}
