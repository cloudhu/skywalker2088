use bevy::math::Vec2;
use bevy::prelude::{Component, Timer};

#[derive(Component)]
pub struct ShouldDespawn;

#[derive(Component)]
pub struct ExplosionRender {
    pub origin: Vec2,
    pub radius: f32,
    pub ttl: Timer,
    pub fade_out: bool,
}
