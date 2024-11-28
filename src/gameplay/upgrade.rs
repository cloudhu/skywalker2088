use crate::components::health::HealthComponent;
use crate::components::player::PlayerComponent;
use crate::gameplay::loot::{Cargo, Magnet};
use crate::screens::AppStates;
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
            .filter(|(upgrade, _)| matches!(upgrade, UpgradeEvent::Passive(_)))
            .count()
            >= 6
    }

    pub fn reached_max_weapons(&self) -> bool {
        self.0
            .iter()
            .filter(|(upgrade, _)| matches!(upgrade, UpgradeEvent::Weapon(_)))
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
            UpgradeEvent::Weapon(TurretClass::AutoCannon) => "AutoCannonDes",
            UpgradeEvent::Weapon(TurretClass::BlastLaser) => "BlastLaserDes",
            UpgradeEvent::Weapon(TurretClass::ChainLaser) => "ChainLaserDes",
            UpgradeEvent::Weapon(TurretClass::Emp) => "EmpDes",
            UpgradeEvent::Weapon(TurretClass::MineLauncher) => "MineLauncherDes",
            UpgradeEvent::Weapon(TurretClass::PierceLaser) => "PierceLaserDes",
            UpgradeEvent::Weapon(TurretClass::RocketLauncher) => "RocketLauncherDes",
            UpgradeEvent::Weapon(TurretClass::ShrapnelCannon) => "ShrapnelCannonDes",
            UpgradeEvent::Passive(Passive::Armor) => "ArmorDes",
            UpgradeEvent::Passive(Passive::Crit) => "CritDes",
            UpgradeEvent::Passive(Passive::Experience) => "ExperienceDes",
            UpgradeEvent::Passive(Passive::FireRate) => "FireRateDes",
            UpgradeEvent::Passive(Passive::Magnet) => "MagnetDes",
            UpgradeEvent::Passive(Passive::ShieldRecharge) => "ShieldRechargeDes",
            UpgradeEvent::Passive(Passive::Speed) => "SpeedDes",
            UpgradeEvent::Heal => "HealDes",
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
                .distributive_run_if(in_state(AppStates::Game)),
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
    player_query: Query<(Entity, Option<&Children>), With<PlayerComponent>>,
    turret_query: Query<&TurretClass>,
    mut existing_auto_cannon: Query<&mut FireRate>,
    mut existing_rocket_launcher: Query<&mut MultiShot>,
    mut existing_shrapnel_cannon: Query<&mut DoesDamage>,
    mut existing_mine_launcher: Query<&mut EffectSize>,
) {
    for ev in upgrade_event.read() {
        if let UpgradeEvent::Weapon(weapon) = ev {
            // Get player
            for (player_entity, children) in &player_query {
                // Search for existing
                let existing = match children {
                    Some(children) => children.iter().find(|child| {
                        if let Ok(turret) = turret_query.get(**child) {
                            return turret == weapon;
                        }
                        false
                    }),
                    None => None,
                };

                match existing {
                    Some(entity) => {
                        // TODO split up logic into systems
                        match weapon {
                            TurretClass::AutoCannon => {
                                let mut fire_rate = existing_auto_cannon.get_mut(*entity).unwrap();
                                let new_rate = fire_rate.rate * 2.0;
                                fire_rate.set_rate_in_seconds(new_rate);
                            }
                            TurretClass::BlastLaser => {
                                let mut fire_rate = existing_auto_cannon.get_mut(*entity).unwrap();
                                let new_rate = fire_rate.rate * 2.0;
                                fire_rate.set_rate_in_seconds(new_rate);
                            }
                            TurretClass::RocketLauncher => {
                                let mut shots = existing_rocket_launcher.get_mut(*entity).unwrap();
                                shots.amount += 1;
                            }
                            TurretClass::ShrapnelCannon => {
                                let mut damage = existing_shrapnel_cannon.get_mut(*entity).unwrap();
                                damage.amount += 1;
                            }
                            TurretClass::MineLauncher => {
                                let mut size = existing_mine_launcher.get_mut(*entity).unwrap();
                                size.0 *= 1.5;
                            }
                            TurretClass::ChainLaser => {
                                let mut shots = existing_rocket_launcher.get_mut(*entity).unwrap();
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
                                if let UpgradeEvent::Passive(passive) = upgrade {
                                    apply_turret_upgrade(
                                        (&mut bundle.fire_rate, &mut bundle.damage),
                                        passive,
                                        *level,
                                    )
                                }
                            }

                            parent.spawn(bundle);
                        });
                    }
                }
            }
        }
    }
}

fn upgrade_magnet_event(
    mut upgrade_event: EventReader<UpgradeEvent>,
    mut query: Query<&mut Magnet, With<PlayerComponent>>,
) {
    for ev in upgrade_event.read() {
        if let UpgradeEvent::Passive(Passive::Magnet) = ev {
            for mut magnet in &mut query {
                magnet.range += 200.0;
                magnet.strength += 2.0;
            }
        }
    }
}

fn upgrade_speed_event(
    mut upgrade_event: EventReader<UpgradeEvent>,
    mut query: Query<&mut Engine, With<PlayerComponent>>,
) {
    for ev in upgrade_event.read() {
        if let UpgradeEvent::Passive(Passive::Speed) = ev {
            for mut engine in &mut query {
                engine.power += 2.0;
                engine.max_speed += 4.0;
            }
        }
    }
}

fn upgrade_health_events(
    mut upgrade_event: EventReader<UpgradeEvent>,
    mut query: Query<&mut HealthComponent, With<PlayerComponent>>,
) {
    for ev in upgrade_event.read() {
        match ev {
            UpgradeEvent::Passive(Passive::ShieldRecharge) => {
                for mut health in &mut query {
                    let mut new_timer =
                        health.shields_recharge_timer.duration().as_secs_f32() - 0.5;
                    if new_timer < 0.1 {
                        new_timer = 0.1;
                    }
                    health
                        .shields_recharge_timer
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
                    health.increase_max_health(25);
                    health.full_heal();
                }
            }
            _ => (),
        }
    }
}

fn upgrade_experience_event(
    mut upgrade_event: EventReader<UpgradeEvent>,
    mut query: Query<&mut Cargo, With<PlayerComponent>>,
) {
    for ev in upgrade_event.read() {
        if let UpgradeEvent::Passive(Passive::Experience) = ev {
            for mut cargo in &mut query {
                cargo.bonus_chance += 0.1;
            }
        }
    }
}

fn upgrade_heal_event(
    mut upgrade_event: EventReader<UpgradeEvent>,
    mut query: Query<&mut HealthComponent, With<PlayerComponent>>,
) {
    for ev in upgrade_event.read() {
        if let UpgradeEvent::Heal = ev {
            for mut health in &mut query {
                health.heal(50);
            }
        }
    }
}

fn upgrade_fire_rate_events(
    mut upgrade_event: EventReader<UpgradeEvent>,
    player_query: Query<&Children, With<PlayerComponent>>,
    mut turret_query: Query<(&mut FireRate, &mut DoesDamage)>,
) {
    for ev in upgrade_event.read() {
        if let UpgradeEvent::Passive(passive) = ev {
            let turrets = player_query.iter().flat_map(|children| children.iter());
            for turret in turrets {
                if let Ok((mut fire_rate, mut damage)) = turret_query.get_mut(*turret) {
                    apply_turret_upgrade((&mut fire_rate, &mut damage), passive, 1);
                }
            }
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
