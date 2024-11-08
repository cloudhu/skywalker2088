use crate::util::Colour;
use bevy::app::App;
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{Component, Timer, TimerMode};

#[derive(Component)]
pub struct ExplosionRender {
    pub origin: Vec2,
    pub radius: f32,
    pub ttl: Timer,
    pub fade_out: bool,
}

#[derive(Component)]
pub struct ShouldDespawn;

#[derive(Component, Default)]
pub struct DespawnWithScene;

#[derive(Component)]
pub struct ExplodesOnDespawn {
    pub amount_min: u32,
    pub amount_max: u32,
    pub spread: f32,
    pub colour: Color,
    pub duration_min: f32,
    pub duration_max: f32,
    pub size_min: f32,
    pub size_max: f32,
}

impl Default for ExplodesOnDespawn {
    fn default() -> Self {
        ExplodesOnDespawn {
            amount_min: 1,
            amount_max: 1,
            colour: Colour::RED,
            duration_min: 0.3,
            duration_max: 0.4,
            size_min: 40.0,
            size_max: 40.0,
            spread: 10.0,
        }
    }
}

#[derive(Component)]
pub struct HitFlash {
    pub timer: Timer,
    pub flash_colour: Color,
    pub original_colour: Option<Color>,
}

impl HitFlash {
    pub fn hit(&mut self) {
        self.timer.reset();
        self.timer.unpause();
    }
}

impl Default for HitFlash {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(0.1, TimerMode::Once);
        timer.pause();
        Self {
            timer,
            flash_colour: Colour::RED,
            original_colour: None,
        }
    }
}

pub(super) fn plugin(_app: &mut App) {}
