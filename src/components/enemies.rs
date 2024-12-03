use crate::components::audio::CollisionSoundType;
use crate::components::spawnable::MobType;
use crate::components::weapon::WeaponData;
use bevy::math::Vec2;
use bevy::prelude::Resource;
use serde::Deserialize;
use std::collections::HashMap;

/// Stores data about mob entities
#[derive(Resource)]
pub struct MobsResource {
    /// Mob types mapped to mob data
    pub mobs: HashMap<MobType, MobData>,
}

/// Data about mob entities that can be stored in data ron file
#[derive(Deserialize)]
pub struct MobData {
    /// Type of mob
    pub mob_type: MobType,
    /// List of spawnable behaviors that are performed
    // #[serde(default)]
    // pub spawnable_behaviors: Vec<SpawnableBehavior>,
    // /// Behavior sequence type
    // pub behavior_sequence_type: Option<MobBehaviorSequenceType>,
    // /// List of mob behaviors that are performed
    // #[serde(default)]
    // pub mob_behaviors: Vec<MobBehavior>,
    // /// behaviors used to control attached mob segments
    // #[serde(default)]
    // pub control_behaviors: Vec<MobSegmentControlBehavior>,
    /// behaviors that mob segments attached to the mob will perform, given the mobs current behavior
    // pub mob_segment_behaviors: Option<
    //     HashMap<MobSegmentControlBehavior, HashMap<MobSegmentType, Vec<MobSegmentBehavior>>>,
    // >,
    /// Whether the mob can rotate on its z axis
    #[serde(default)]
    pub can_rotate: bool,
    /// Acceleration stat
    #[serde(default)]
    pub acceleration: Vec2,
    /// Deceleration stat
    #[serde(default)]
    pub deceleration: Vec2,
    /// Maximum speed that can be accelerated to
    #[serde(default)]
    pub speed: Vec2,
    /// Angular acceleration stat
    #[serde(default)]
    pub angular_acceleration: f32,
    /// Angular deceleration stat
    #[serde(default)]
    pub angular_deceleration: f32,
    /// Maximum angular speed that can be accelerated to
    #[serde(default)]
    pub angular_speed: f32,
    /// Motion that the mob initializes with
    // #[serde(default)]
    // pub initial_motion: InitialMotion,
    // /// Dimensions of the mob's hitbox
    // pub colliders: Vec<ColliderData>,
    // /// Texture
    // pub animation: AnimationData,
    // /// Optional data describing the thruster
    // pub thruster: Option<ThrusterData>,
    /// Damage dealt to other factions through attacks
    #[serde(default)]
    pub collision_damage: usize,
    /// Damage dealt to defense objective, after reaching bottom of arena
    #[serde(default)]
    pub collision_sound: CollisionSoundType,
    // pub defense_interaction: Option<DefenseInteraction>,
    /// Health of the mob
    pub health: usize,
    /// List of consumable drops
    // #[serde(default)]
    // pub consumable_drops: DropListType,
    /// Z level of the mobs transform
    pub z_level: f32,
    /// anchor points for other mob segments
    // #[serde(default)]
    // pub mob_segment_anchor_points: Vec<MobSegmentAnchorPointData>,
    // /// mob spawners that the mob can use
    // #[serde(default)]
    // pub mob_spawners: HashMap<String, Vec<MobSpawnerData>>,
    /// projectile spawners that the mob can use
    #[serde(default)]
    pub weapons: Option<Vec<WeaponData>>,
    // #[serde(default = "default_mob_density")]
    // pub density: f32,
}
