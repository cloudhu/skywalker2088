use crate::components::objectives::DefenseInteraction;
use crate::components::player::PlayerInput;
use crate::components::spawnable::{MobSegmentType, MobType};
use bevy::prelude::*;

pub enum RunOutcomeType {
    Victory,
    Defeat(RunDefeatType),
}

pub enum RunDefeatType {
    PlayersDestroyed,
    DefenseDestroyed,
}

#[derive(Event)]
pub struct RunEndEvent {
    pub outcome: RunOutcomeType,
}

#[derive(Event)]
pub struct CyclePhaseEvent;

// Event for sending damage dealt from mob reaching bottom of arena
#[derive(Event)]
pub struct MobReachedBottomGateEvent {
    pub mob_type: Option<MobType>,
    pub mob_segment_type: Option<MobSegmentType>,
    pub defense_interaction: DefenseInteraction,
}

/// An event that triggers the user's screen/camera to shake, as if the player's ship was just hit.
#[derive(Event)]
pub struct ScreenShakeEvent {
    /// This should be between 0 and 1. ( 0 is no screen shake; 1 is a very aggressive shake.)
    pub trauma: f32,
}

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
