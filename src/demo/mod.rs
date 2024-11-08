//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use crate::gameplay::level;
use crate::player::animation;
use bevy::prelude::*;

pub mod enemy;
mod enemy_animation;
pub(crate) mod movement;
pub mod player;
// mod bullet;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        enemy_animation::plugin,
        movement::plugin,
        player::plugin,
        level::plugin,
        enemy::plugin,
        // bullet::plugin,
    ));
}
