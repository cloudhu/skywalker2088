use crate::gameplay::effects::HitFlash;
use crate::gameplay::gamelogic::{ExplodesOnDespawn, Targettable, WillTarget};
use crate::gameplay::physics::{Collider, Physics};
use crate::ship::engine::Engine;
use bevy::prelude::*;

// Bundles
#[derive(Component, Default)]
#[require(
    Sprite,
    Physics,
    Engine,
    Health,
    Collider,
    Targettable,
    WillTarget,
    ExplodesOnDespawn,
    HitFlash
)]
pub struct Spacecraft;

#[derive(Component)]
pub struct Seeker(pub Entity);

#[derive(Reflect, Component)]
pub struct Owner(pub Entity);

#[derive(Component)]
pub struct Health {
    pub health: usize,
    pub shields: usize,
    pub max_health: usize,
    pub max_shields: usize,
    pub shields_recharge_rate: f32,
    pub shields_recharge_cooldown: Timer,
    pub shields_recharge_timer: Timer,
}

impl Default for Health {
    fn default() -> Self {
        Health::new(100, 100, 2.0)
    }
}

impl Health {
    pub fn new(max_health: usize, max_shield: usize, shields_recharge_rate: f32) -> Health {
        Health {
            health: max_health,
            max_health,
            shields: max_shield,
            max_shields: max_shield,
            shields_recharge_rate,
            shields_recharge_cooldown: Timer::from_seconds(3.0, TimerMode::Once),
            shields_recharge_timer: Timer::from_seconds(
                shields_recharge_rate,
                TimerMode::Repeating,
            ),
        }
    }

    pub fn take_damage(&mut self, amount: usize) {
        self.shields_recharge_cooldown.reset();
        self.shields_recharge_timer.reset();
        if amount > self.shields {
            self.health = self
                .health
                .saturating_sub(amount.saturating_sub(self.shields));
            self.shields = 0;
        } else {
            self.shields = self.shields.saturating_sub(amount);
        }
    }

    pub fn heal(&mut self, amount: usize) {
        let missing_health = self.max_health - self.health;
        if amount > missing_health {
            self.health = self.max_health;
            self.shields += amount - missing_health;
            if self.shields > self.max_shields {
                self.shields = self.max_shields;
            }
        } else {
            self.health += amount;
        }
    }
}
