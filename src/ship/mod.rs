//! The game's ship and components.
pub mod bullet;
pub mod engine;
pub mod platform;
pub mod turret;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        platform::plugin,
        engine::plugin,
        bullet::plugin,
        turret::plugin,
    ));
}
