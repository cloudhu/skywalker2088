//! The game's ship and components.
pub mod animation;
pub mod bullet;
pub mod engine;
pub mod turret;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        engine::plugin,
        bullet::plugin,
        turret::plugin,
        animation::plugin,
    ));
}
