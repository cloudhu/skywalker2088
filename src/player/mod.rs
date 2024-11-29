//! Exposes a plugin and resources to deal with player behavior such as spawning, moving, firing, and dying.
pub use self::resources::CharactersResource;
use self::systems::{
    abilities::{
        player_ability_cooldown_system, player_ability_input_system,
        standard_weapon_ability_system, start_charge_ability_system, update_charge_ability_system,
    },
    movement::{player_movement_system, player_tilt_system},
    player_death_system, players_reset_system,
    upgrades::scale_ability_cooldowns_system,
};
use crate::assets::game_assets::AppStates;
use crate::assets::game_assets::GameUpdateSet;
use crate::components::abilities::{
    AbilitiesResource, AbilityDescriptionsResource, ActivateAbilityEvent,
};
use crate::components::input::PlayerAction;
use crate::components::player::{InputRestrictionsAtSpawn, PlayerComponent, PlayersResource};
use crate::gameplay::GameStates;
use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
    state::{
        condition::in_state,
        state::{OnEnter, OnExit},
    },
};
use leafwing_input_manager::prelude::InputManagerPlugin;
use ron::de::from_bytes;

mod resources;
pub mod spawn;
mod systems;

/// Contains systems to allow the player to do most (all?) of its required behaviors.
pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlayerComponent>();
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
        app.add_event::<ActivateAbilityEvent>();

        app.insert_resource(
            from_bytes::<CharactersResource>(include_bytes!("../../assets/data/characters.ron"))
                .unwrap(),
        );

        app.insert_resource(
            from_bytes::<AbilitiesResource>(include_bytes!("../../assets/data/abilities.ron"))
                .unwrap(),
        );

        app.insert_resource(
            from_bytes::<AbilityDescriptionsResource>(include_bytes!(
                "../../assets/data/ability_descriptions.ron"
            ))
            .unwrap(),
        );

        app.insert_resource(PlayersResource::default())
            .insert_resource(InputRestrictionsAtSpawn::default());

        app.add_systems(
            Update,
            (
                player_death_system,
                player_movement_system.in_set(GameUpdateSet::Movement),
                player_tilt_system.in_set(GameUpdateSet::Movement),
                player_ability_cooldown_system,
                player_ability_input_system,
                standard_weapon_ability_system,
                start_charge_ability_system,
                update_charge_ability_system,
                scale_ability_cooldowns_system,
            )
                .run_if(in_state(AppStates::Game))
                .run_if(in_state(GameStates::Playing)),
        );

        // reset the run after exiting the end game screens and when entering the main menu
        app.add_systems(OnExit(AppStates::GameOver), players_reset_system);
        app.add_systems(OnExit(AppStates::Victory), players_reset_system);
        app.add_systems(OnEnter(AppStates::MainMenu), players_reset_system);
    }
}