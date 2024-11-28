use crate::assets::player_assets::PlayerAssets;
use crate::components::abilities::{
    AbilitiesResource, ChargeAbilityBundle, SlotOneAbilityType, SlotTwoAbilityType,
    StandardWeaponAbilityBundle,
};
use crate::components::health::{HealthComponent, ShipBundle};
use crate::components::input::{InputsResource, PlayerAction};
use crate::components::player::{PlayerBundle, PlayerIDComponent, PlayerInput};
use crate::gameplay::gamelogic::{Allegiance, Targettable, WillTarget};
use crate::gameplay::loot::{Cargo, Magnet};
use crate::gameplay::physics::{BaseGlyphRotation, Physics};
use crate::options::resources::GameParametersResource;
use crate::player::PlayersResource;
use crate::screens::AppStates;
use crate::ship::engine::Engine;
use crate::util::RenderLayer;
use bevy::color::Color;
use bevy::core::Name;
use bevy::ecs::system::{Commands, Res, RunSystemOnce};
use bevy::ecs::world::Command;
use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::input::gamepad::Gamepad;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::transform::components::Transform;
use bevy_rapier2d::dynamics::{ExternalImpulse, LockedAxes, RigidBody, Velocity};
use bevy_rapier2d::geometry::{ActiveEvents, ColliderMassProperties, Restitution};
use leafwing_input_manager::{prelude::ActionState, InputManagerBundle};
use std::f32::consts::PI;

trait PlayerAbilityChildBuilderExt {
    fn spawn_slot_1_ability(
        &mut self,
        abilities_res: &AbilitiesResource,
        ability_type: &Option<SlotOneAbilityType>,
    );

    fn spawn_slot_2_ability(
        &mut self,
        abilities_res: &AbilitiesResource,
        ability_type: &Option<SlotTwoAbilityType>,
    );
}

impl PlayerAbilityChildBuilderExt for ChildBuilder<'_> {
    fn spawn_slot_1_ability(
        &mut self,
        abilities_res: &AbilitiesResource,
        ability_type: &Option<SlotOneAbilityType>,
    ) {
        if let Some(ability_type) = ability_type {
            match ability_type {
                SlotOneAbilityType::StandardBlast => self.spawn(StandardWeaponAbilityBundle::from(
                    &abilities_res.standard_blast_ability,
                )),
                SlotOneAbilityType::StandardBullet => self.spawn(
                    StandardWeaponAbilityBundle::from(&abilities_res.standard_bullet_ability),
                ),
            };
        }
    }

    fn spawn_slot_2_ability(
        &mut self,
        abilities_res: &AbilitiesResource,
        ability_type: &Option<SlotTwoAbilityType>,
    ) {
        if let Some(ability_type) = ability_type {
            match ability_type {
                SlotTwoAbilityType::Charge => {
                    self.spawn(ChargeAbilityBundle::from(&abilities_res.charge_ability))
                }
                SlotTwoAbilityType::MegaBlast => self.spawn(StandardWeaponAbilityBundle::from(
                    &abilities_res.mega_blast_ability,
                )),
            };
        }
    }
}

/// A command to spawn the player character.
#[derive(Debug)]
pub struct SpawnPlayer {
    /// 可配置的飞船参数.
    pub max_speed: f32,
    pub drag: f32,
    pub power: f32,
    pub steering_factor: f32,
    pub max_health: usize,
    pub max_shield: usize,
    pub radius: f32,
}

impl Default for SpawnPlayer {
    fn default() -> Self {
        SpawnPlayer::new(16.0, 5.0, 8.0, 10.0, 100, 100, 10.0)
    }
}

impl SpawnPlayer {
    pub fn new(
        max_speed: f32,
        drag: f32,
        power: f32,
        steering_factor: f32,
        max_health: usize,
        max_shield: usize,
        radius: f32,
    ) -> SpawnPlayer {
        SpawnPlayer {
            max_speed,
            drag,
            power,
            steering_factor,
            max_health,
            max_shield,
            radius,
        }
    }
}

impl Command for SpawnPlayer {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_player);
    }
}

/// Spawns player into the game
fn spawn_player(
    In(config): In<SpawnPlayer>,
    mut commands: Commands,
    characters: Res<crate::components::character::CharactersResource>,
    game_parameters: Res<GameParametersResource>,
    player_assets: Res<PlayerAssets>,
    players_resource: Res<PlayersResource>,
    inputs_res: Res<InputsResource>,
    abilities_res: Res<AbilitiesResource>,
) {
    // check if more than one player is playing
    let is_multiplayer = players_resource.player_data.get(1).is_some();
    println!(
        "spawning player is_multiplayer:{},player_data:{}",
        is_multiplayer,
        players_resource.player_data.get(0).is_some()
    );
    for (player_id, maybe_player_data) in players_resource
        .player_data
        .iter()
        .enumerate()
        .map(|(id, pd)| (PlayerIDComponent::from(id), pd))
    {
        if let Some(player_data) = maybe_player_data {
            // choose a character
            let character = &characters.characters[&player_data.character];

            // scale collider to align with the sprite
            let collider_size_hx =
                character.collider_dimensions.x * game_parameters.sprite_scale / 2.0;
            let collider_size_hy =
                character.collider_dimensions.y * game_parameters.sprite_scale / 2.0;
            println!("spawning Player:{}", player_data);
            // create player component from character
            let player_bundle = PlayerBundle::from(character).with_id(player_id);

            // spawn the player
            let mut player_entity = commands.spawn_empty();
            player_entity
                .insert(SpriteBundle {
                    texture: player_assets.get_asset(&character.character_type),
                    // transform: Transform::from_translation(Vec3 {
                    //     x: 100.0,
                    //     y: 100.0,
                    //     z: RenderLayer::Player.as_z(),
                    // }),
                    ..Default::default()
                })
                .insert(RigidBody::Dynamic)
                .insert(LockedAxes::ROTATION_LOCKED_Z)
                .insert(Transform {
                    translation: if is_multiplayer {
                        Vec3::new(
                            if matches!(player_id, PlayerIDComponent::One) {
                                -game_parameters.player_spawn_distance - 100.0
                            } else {
                                game_parameters.player_spawn_distance + 100.0
                            },
                            100.0,
                            if matches!(player_id, PlayerIDComponent::One) {
                                RenderLayer::Player.as_z()
                            } else {
                                RenderLayer::Player.as_z() + 0.2
                            },
                        )
                    } else {
                        Vec3 {
                            x: 100.0,
                            y: 100.0,
                            z: RenderLayer::Player.as_z(),
                        }
                    },
                    scale: Vec3::new(
                        game_parameters.sprite_scale * 8.0,
                        game_parameters.sprite_scale * 8.0,
                        1.0,
                    ),
                    ..Default::default()
                })
                .insert(InputManagerBundle::<PlayerAction> {
                    action_state: ActionState::default(),
                    input_map: match player_data.input {
                        PlayerInput::Keyboard => inputs_res.player_keyboard.clone(),
                        PlayerInput::Gamepad(id) => inputs_res
                            .player_gamepad
                            .clone()
                            .set_gamepad(Gamepad { id })
                            .to_owned(),
                    },
                })
                .insert(bevy_rapier2d::geometry::Collider::cuboid(
                    collider_size_hx,
                    collider_size_hy,
                ))
                .insert(Velocity::default())
                .insert(Restitution::new(1.0))
                .insert(ColliderMassProperties::Density(character.collider_density))
                .insert(player_bundle)
                .insert(HealthComponent::from(character))
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(ExternalImpulse::default())
                .insert(Name::new("Player"))
                .with_children(|parent| {
                    parent.spawn_slot_1_ability(&abilities_res, &character.slot_1_ability);
                    parent.spawn_slot_2_ability(&abilities_res, &character.slot_2_ability);
                })
                .insert(ShipBundle {
                    physics: Physics::new(config.drag),
                    engine: Engine::new_with_steering(
                        config.power,
                        config.max_speed,
                        config.steering_factor,
                    ),
                    targettable: Targettable(Allegiance::Friend),
                    will_target: WillTarget(vec![Allegiance::Enemy]),
                    ..default()
                })
                .insert(BaseGlyphRotation {
                    rotation: Quat::from_rotation_z(PI / 2.0),
                })
                .insert(Cargo::default())
                .insert(Magnet::default())
                .insert(StateScoped(AppStates::Game));

            // add colored outline to player if multiplayer
            if is_multiplayer {
                player_entity.with_children(|parent| {
                    parent
                        .spawn(SpriteBundle {
                            texture: player_assets.get_outline_asset(&character.character_type),
                            sprite: Sprite {
                                color: if matches!(player_id, PlayerIDComponent::One) {
                                    Color::srgb(0.7, 0.0, 0.0)
                                } else {
                                    Color::srgb(0.0, 0.0, 1.0)
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Transform::from_translation(Vec3::new(
                            100.0,
                            100.0,
                            RenderLayer::Player.as_z() + 0.1,
                        )));
                });
            }
        }
    }
}
