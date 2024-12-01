//! Spawn the main level.
use crate::player::spawn::SpawnPlayer;
use bevy::{ecs::world::Command, prelude::*};
// pub(crate) fn plugin(_app: &mut App) {
//     // No setup required for this plugin.
//     // It's still good to have a function here so that we can add some setup
//     // later if needed.
// }

/// A [`Command`] to spawn the level.
/// Functions that accept only `&mut World` as their parameter implement [`Command`].
/// We use this style when a command requires no configuration.
pub fn spawn_level(world: &mut World) {
    debug!("spawning level");
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    SpawnPlayer::default().apply(world);
}
