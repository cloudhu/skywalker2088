use crate::gameplay::effects::HitFlash;
use crate::gameplay::gamelogic::{DespawnWithScene, ExplodesOnDespawn, Targettable, WillTarget};
use crate::ship::engine::Engine;
use bevy::prelude::*;
use std::time::Duration;

// Bundles
#[derive(Bundle, Default)]
pub struct ShipBundle {
    pub engine: Engine,
    pub health: HealthComponent,
    pub targettable: Targettable,
    pub will_target: WillTarget,
    pub despawn_with_scene: DespawnWithScene,
    pub explodes_on_despawn: ExplodesOnDespawn,
    pub hit_flash: HitFlash,
}

#[derive(Component)]
pub struct Seeker(pub Entity);

#[derive(Reflect, Component)]
pub struct Owner(pub Entity);

#[derive(Component)]
pub struct HealthComponent {
    /// Current health value
    health: usize,
    /// Maximum health value
    max_health: usize,
    /// Amount of armor, which absorbs full damage from a single hit
    armor: usize,
    /// Current shields value
    shields: usize,
    /// Maximum shields value
    max_shields: usize,
    /// Time it takes to regenerate one unit of shields
    pub shields_recharge_timer: Timer,
    pub shield_recharge_cooldown: Timer,
}

impl Default for HealthComponent {
    fn default() -> Self {
        HealthComponent::new(100, 100, 3.0)
    }
}

impl HealthComponent {
    /// Create a new health struct from a maximum health and shields value
    pub fn new(
        max_health: usize,
        max_shields: usize,
        shields_recharge_rate: f32,
    ) -> HealthComponent {
        HealthComponent {
            health: max_health,
            max_health,
            armor: 0,
            shields: max_shields,
            max_shields,
            shield_recharge_cooldown: Timer::from_seconds(3.0, TimerMode::Once),
            shields_recharge_timer: Timer::from_seconds(
                shields_recharge_rate,
                TimerMode::Repeating,
            ),
        }
    }

    pub fn regenerate_shields(&mut self, delta_time: Duration) {
        self.shields_recharge_timer.tick(delta_time);
        if self.shields_recharge_timer.just_finished() && self.shields < self.max_shields {
            self.shields += 1
        }
    }

    /// Check if health is below zero
    pub fn is_dead(&self) -> bool {
        self.health == 0
    }

    /// Take damage (deplete armor, then shields, then health  in that order)
    pub fn take_damage(&mut self, amount: usize) {
        self.shield_recharge_cooldown.reset();
        self.shields_recharge_timer.reset();
        if amount > self.shields {
            let damage_piercing_shields = amount.saturating_sub(self.shields);
            self.health = self.health.saturating_sub(damage_piercing_shields);
            self.shields = 0;
        } else {
            self.shields -= amount;
        }
    }

    #[allow(dead_code)]
    pub fn get_max_health(&self) -> usize {
        self.max_health
    }

    /// Get current health
    pub fn get_health(&self) -> usize {
        self.health
    }

    #[allow(dead_code)]
    pub fn get_max_shields(&self) -> usize {
        self.max_shields
    }

    /// Get current health
    #[allow(dead_code)]
    pub fn get_shields(&self) -> usize {
        self.shields
    }

    /// Get available armor count
    pub fn get_armor(&self) -> usize {
        self.armor
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

    /// Add to armor
    pub fn gain_armor(&mut self, armor: usize) {
        self.armor += armor;
    }

    /// Percentage of defense left
    pub fn get_health_percentage(&self) -> f32 {
        if self.max_health > 0 {
            self.health as f32 / self.max_health as f32
        } else {
            0.0
        }
    }

    /// Percentage of defense left
    pub fn get_shields_percentage(&self) -> f32 {
        if self.max_shields > 0 {
            self.shields as f32 / self.max_shields as f32
        } else {
            0.0
        }
    }

    pub fn increase_max_health(&mut self, value: usize) {
        self.max_health += value;
    }
    pub fn full_heal(&mut self) {
        self.health = self.max_health;
    }
}

#[derive(Component, Copy, Clone)]
pub struct Damage {
    pub amount: usize,
    pub is_crit: bool,
}

impl Damage {
    pub fn from_amount(amount: usize) -> Self {
        Self {
            amount,
            is_crit: false,
        }
    }
}
