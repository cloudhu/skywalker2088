use crate::components::player::PlayerInput;
use bevy::prelude::{Entity, Event};

/// This event is used for notifying systems when an animation for an entity has been completed
/// Can be used for despawning entities after animations finish
#[derive(Event)]
pub struct AnimationCompletedEvent(pub Entity);

/// Stores the index (likely 0 or 1) of the player that joined an n-player game.
#[derive(Event)]
pub struct PlayerJoinEvent {
    pub player_idx: u8,
    pub input: PlayerInput,
}
