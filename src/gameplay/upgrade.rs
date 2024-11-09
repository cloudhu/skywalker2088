use crate::components::common::Health;
use crate::gameplay::loot::{Cargo, Magnet};
use crate::player::batman::IsPlayer;
use crate::screens::AppState;
use crate::ship::engine::Engine;
use crate::ship::turret::{DoesDamage, EffectSize, FireRate, MultiShot, TurretBundle, TurretClass};
use bevy::app::App;
use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::distr::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use std::fmt::Display;
use std::time::Duration;

#[derive(Resource)]
pub struct PlayerUpgrades(pub HashMap<UpgradeEvent, u8>);

impl PlayerUpgrades {
    pub fn display_for_ui(&self) -> Vec<String> {
        self.0
            .iter()
            .filter(|(_, level)| **level > 0)
            .map(|(upgrade, level)| format!("{:0>2} {:>16}", level, upgrade))
            .collect()
    }

    pub fn max_allowed_level() -> u8 {
        8
    }

    pub fn reached_max_passives(&self) -> bool {
        self.0
            .iter()
            .filter(|(upgrade, _)| match upgrade {
                UpgradeEvent::Passive(_) => true,
                _ => false,
            })
            .count()
            >= 6
    }

    pub fn reached_max_weapons(&self) -> bool {
        self.0
            .iter()
            .filter(|(upgrade, _)| match upgrade {
                UpgradeEvent::Weapon(_) => true,
                _ => false,
            })
            .count()
            >= 4
    }
}

#[derive(Event, Copy, Clone, Eq, Hash, PartialEq)]
pub enum UpgradeEvent {
    Weapon(TurretClass),
    Passive(Passive),
    Heal,
}

impl Distribution<UpgradeEvent> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UpgradeEvent {
        match rng.gen_range(0..2) {
            0 => UpgradeEvent::Weapon(rand::random()),
            _ => UpgradeEvent::Passive(rand::random()),
        }
    }
}

impl Display for UpgradeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpgradeEvent::Weapon(weapon) => write!(f, "{}", weapon),
            UpgradeEvent::Passive(passive) => write!(f, "{}", passive),
            UpgradeEvent::Heal => write!(f, "Heal"),
        }
    }
}

impl UpgradeEvent {
    pub fn describe(&self) -> String {
        match self {
            UpgradeEvent::Weapon(TurretClass::AutoCannon) => {
                "Rapidly fires bullets towards the target"
            }
            UpgradeEvent::Weapon(TurretClass::BlastLaser) => "Always hits. Deals low damage",
            UpgradeEvent::Weapon(TurretClass::ChainLaser) => {
                "Shoots a laser that jumps to nearby enemies"
            }
            UpgradeEvent::Weapon(TurretClass::Emp) => {
                "Creates a shockwave around you that deals damage to enemies"
            }
            UpgradeEvent::Weapon(TurretClass::MineLauncher) => {
                "Drops mines that explode when enemies are in close proximity"
            }
            UpgradeEvent::Weapon(TurretClass::PierceLaser) => {
                "Shoots a heavy damaging laser that pierces through enemies"
            }
            UpgradeEvent::Weapon(TurretClass::RocketLauncher) => {
                "Shoots a seeking missile that explodes on impact"
            }
            UpgradeEvent::Weapon(TurretClass::ShrapnelCannon) => {
                "Shoots a spray of bullets in a cone towards the target"
            }
            UpgradeEvent::Passive(Passive::Armor) => "Increase armor by 25",
            UpgradeEvent::Passive(Passive::Crit) => {
                "Increase chance to deal double damage by 12.5%"
            }
            UpgradeEvent::Passive(Passive::Experience) => {
                "Increase chance to triple experience by 10%"
            }
            UpgradeEvent::Passive(Passive::FireRate) => "Increase turret fire rate by 10%",
            UpgradeEvent::Passive(Passive::Magnet) => {
                "Increase range and speed of experience magnetism"
            }
            UpgradeEvent::Passive(Passive::ShieldRecharge) => {
                "Decrease shield hit and regeneration cooldown"
            }
            UpgradeEvent::Passive(Passive::Speed) => "Increase engine power and max speed",
            UpgradeEvent::Heal => "Restore 50 armor or shields",
        }
        .to_string()
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum Passive {
    Speed,
    Magnet,
    ShieldRecharge,
    Armor,
    FireRate,
    Crit,
    Experience,
}

impl Display for Passive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Passive::Speed => write!(f, "Speed"),
            Passive::Magnet => write!(f, "Magnet"),
            Passive::ShieldRecharge => write!(f, "Shield Boost"),
            Passive::Armor => write!(f, "Reinforced Armor"),
            Passive::FireRate => write!(f, "Rapid Fire"),
            Passive::Crit => write!(f, "Critical Strikes"),
            Passive::Experience => write!(f, "Experience Booster"),
        }
    }
}

impl Distribution<Passive> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Passive {
        match rng.gen_range(0..7) {
            0 => Passive::Speed,
            1 => Passive::ShieldRecharge,
            2 => Passive::Armor,
            3 => Passive::FireRate,
            4 => Passive::Crit,
            5 => Passive::Experience,
            _ => Passive::Magnet,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(PlayerUpgrades(HashMap::new()))
        .add_event::<UpgradeEvent>()
        .add_systems(
            Update,
            (
                record_upgrade,
                upgrade_weapon_event,
                upgrade_magnet_event,
                upgrade_speed_event,
                upgrade_health_events,
                upgrade_fire_rate_events,
                upgrade_experience_event,
                upgrade_heal_event,
            )
                .distributive_run_if(in_state(AppState::InGame)),
        );
}

fn record_upgrade(
    mut upgrade_event: EventReader<UpgradeEvent>,
    mut player_upgrades: ResMut<PlayerUpgrades>,
) {
    for ev in upgrade_event.read() {
        match ev {
            UpgradeEvent::Heal => (), // No need to record this
            _ => {
                let level = player_upgrades.0.entry(*ev).or_insert(0);
                *level += 1;
            }
        }
    }
}

fn upgrade_weapon_event(
    upgrades: Res<PlayerUpgrades>,
    mut upgrade_event: EventReader<UpgradeEvent>,
    mut commands: Commands,
    player_query: Query<(Entity, Option<&Children>), With<IsPlayer>>,
    turret_query: Query<&TurretClass>,
    mut existing_auto_cannon: Query<&mut FireRate>,
    mut existing_rocket_launcher: Query<&mut MultiShot>,
    mut existing_shrapnel_cannon: Query<&mut DoesDamage>,
    mut existing_mine_launcher: Query<&mut EffectSize>,
) {
    for ev in upgrade_event.read() {
        match ev {
            UpgradeEvent::Weapon(weapon) => {
                // Get player
                for (player_entity, children) in &player_query {
                    // Search for existing
                    let existing = match children {
                        Some(children) => children.iter().find(|child| {
                            if let Ok(turret) = turret_query.get(**child) {
                                return turret == weapon;
                            }
                            return false;
                        }),
                        None => None,
                    };

                    match existing {
                        Some(entity) => {
                            // TODO split up logic into systems
                            match weapon {
                                TurretClass::AutoCannon => {
                                    let mut fire_rate =
                                        existing_auto_cannon.get_mut(*entity).unwrap();
                                    let new_rate = fire_rate.rate * 2.0;
                                    fire_rate.set_rate_in_seconds(new_rate);
                                }
                                TurretClass::BlastLaser => {
                                    let mut fire_rate =
                                        existing_auto_cannon.get_mut(*entity).unwrap();
                                    let new_rate = fire_rate.rate * 2.0;
                                    fire_rate.set_rate_in_seconds(new_rate);
                                }
                                TurretClass::RocketLauncher => {
                                    let mut shots =
                                        existing_rocket_launcher.get_mut(*entity).unwrap();
                                    shots.amount += 1;
                                }
                                TurretClass::ShrapnelCannon => {
                                    let mut damage =
                                        existing_shrapnel_cannon.get_mut(*entity).unwrap();
                                    damage.amount += 1;
                                }
                                TurretClass::MineLauncher => {
                                    let mut size = existing_mine_launcher.get_mut(*entity).unwrap();
                                    size.0 *= 1.5;
                                }
                                TurretClass::ChainLaser => {
                                    let mut shots =
                                        existing_rocket_launcher.get_mut(*entity).unwrap();
                                    shots.amount += 1;
                                }
                                TurretClass::PierceLaser => {
                                    let mut size = existing_mine_launcher.get_mut(*entity).unwrap();
                                    size.0 += 2.0;
                                }
                                TurretClass::Emp => {
                                    let mut size = existing_mine_launcher.get_mut(*entity).unwrap();
                                    size.0 += 20.0;
                                }
                            }
                        }
                        None => {
                            commands.entity(player_entity).with_children(|parent| {
                                let mut bundle = TurretBundle::from_class(weapon);

                                // Apply existing upgrades
                                for (upgrade, level) in upgrades.0.iter() {
                                    match upgrade {
                                        UpgradeEvent::Passive(passive) => apply_turret_upgrade(
                                            (&mut bundle.fire_rate, &mut bundle.damage),
                                            passive,
                                            *level,
                                        ),
                                        _ => (),
                                    }
                                }

                                parent.spawn(bundle);
                            });
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

fn upgrade_magnet_event(
    mut upgrade_event: EventReader<UpgradeEvent>,
    mut query: Query<&mut Magnet, With<IsPlayer>>,
) {
    for ev in upgrade_event.read() {
        match ev {
            UpgradeEvent::Passive(Passive::Magnet) => {
                for mut magnet in &mut query {
                    magnet.range += 200.0;
                    magnet.strength += 2.0;
                }
            }
            _ => (),
        }
    }
}

fn upgrade_speed_event(
    mut upgrade_event: EventReader<UpgradeEvent>,
    mut query: Query<&mut Engine, With<IsPlayer>>,
) {
    for ev in upgrade_event.read() {
        match ev {
            UpgradeEvent::Passive(Passive::Speed) => {
                for mut engine in &mut query {
                    engine.power += 2.0;
                    engine.max_speed += 4.0;
                }
            }
            _ => (),
        }
    }
}

fn upgrade_health_events(
    mut upgrade_event: EventReader<UpgradeEvent>,
    mut query: Query<&mut Health, With<IsPlayer>>,
) {
    for ev in upgrade_event.read() {
        match ev {
            UpgradeEvent::Passive(Passive::ShieldRecharge) => {
                for mut health in &mut query {
                    let mut new_timer = health.shield_recharge_timer.duration().as_secs_f32() - 0.5;
                    if new_timer < 0.1 {
                        new_timer = 0.1;
                    }
                    health
                        .shield_recharge_timer
                        .set_duration(Duration::from_secs_f32(new_timer));
                    let mut new_timer =
                        health.shield_recharge_cooldown.duration().as_secs_f32() - 1.0;
                    if new_timer < 0.5 {
                        new_timer = 0.5;
                    }
                    health
                        .shield_recharge_cooldown
                        .set_duration(Duration::from_secs_f32(new_timer));
                }
            }
            UpgradeEvent::Passive(Passive::Armor) => {
                for mut health in &mut query {
                    health.max_health += 25;
                    health.health += 25;
                }
            }
            _ => (),
        }
    }
}

fn upgrade_experience_event(
    mut upgrade_event: EventReader<UpgradeEvent>,
    mut query: Query<&mut Cargo, With<IsPlayer>>,
) {
    for ev in upgrade_event.read() {
        match ev {
            UpgradeEvent::Passive(Passive::Experience) => {
                for mut cargo in &mut query {
                    cargo.bonus_chance += 0.1;
                }
            }
            _ => (),
        }
    }
}

fn upgrade_heal_event(
    mut upgrade_event: EventReader<UpgradeEvent>,
    mut query: Query<&mut Health, With<IsPlayer>>,
) {
    for ev in upgrade_event.read() {
        match ev {
            UpgradeEvent::Heal => {
                for mut health in &mut query {
                    health.heal(50);
                }
            }
            _ => (),
        }
    }
}

fn upgrade_fire_rate_events(
    mut upgrade_event: EventReader<UpgradeEvent>,
    player_query: Query<&Children, With<IsPlayer>>,
    mut turret_query: Query<(&mut FireRate, &mut DoesDamage)>,
) {
    for ev in upgrade_event.read() {
        match ev {
            UpgradeEvent::Passive(passive) => {
                let turrets = player_query.iter().flat_map(|children| children.iter());
                for turret in turrets {
                    if let Ok((mut fire_rate, mut damage)) = turret_query.get_mut(*turret) {
                        apply_turret_upgrade((&mut fire_rate, &mut damage), passive, 1);
                    }
                }
            }
            _ => (),
        }
    }
}

fn apply_turret_upgrade(turret: (&mut FireRate, &mut DoesDamage), passive: &Passive, times: u8) {
    let (fire_rate, damage) = turret;
    for _ in 0..times {
        match passive {
            Passive::FireRate => {
                let new_rate = fire_rate.rate * 1.1;
                fire_rate.set_rate_in_seconds(new_rate);
            }
            Passive::Crit => {
                damage.crit_chance += 0.125;
            }
            _ => (),
        }
    }
}
