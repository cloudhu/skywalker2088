use crate::gameplay::effects::HitFlash;
use crate::gameplay::gamelogic::{DespawnWithScene, ExplodesOnDespawn, Targettable, WillTarget};
use crate::gameplay::physics::{Collider, Physics};
use crate::ship::engine::Engine;
use bevy::prelude::*;

// Bundles
#[derive(Bundle, Default)]
pub struct ShipBundle {
    pub glyph: Text2dBundle,
    pub physics: Physics,
    pub engine: Engine,
    pub health: Health,
    pub collider: Collider,
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
pub struct Health {
    pub health: i32,
    pub shield: i32,
    pub max_health: i32,
    pub max_shield: i32,
    pub shield_recharge_cooldown: Timer,
    pub shield_recharge_timer: Timer,
}

impl Default for Health {
    fn default() -> Self {
        Health::new(100, 100)
    }
}

impl Health {
    pub fn new(max_health: i32, max_shield: i32) -> Health {
        Health {
            health: max_health,
            max_health,
            shield: max_shield,
            max_shield,
            shield_recharge_cooldown: Timer::from_seconds(3.0, TimerMode::Once),
            shield_recharge_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.shield_recharge_cooldown.reset();
        self.shield_recharge_timer.reset();
        if amount > self.shield {
            self.health -= amount - self.shield;
            self.shield = 0;
        } else {
            self.shield -= amount;
        }
    }

    pub fn heal(&mut self, amount: i32) {
        let missing_health = self.max_health - self.health;
        if amount > missing_health {
            self.health = self.max_health;
            self.shield += amount - missing_health;
            if self.shield > self.max_shield {
                self.shield = self.max_shield;
            }
        } else {
            self.health += amount;
        }
    }
}
