use crate::audio::MenuAudioChannel;
use crate::components::input::{MainMenuExplorer, MenuAction};
use crate::components::states::{AppStates, GameStates};
use bevy::asset::AssetServer;
use bevy::prelude::{NextState, Query, Res, ResMut, With};
use bevy_kira_audio::{AudioChannel, AudioControl};
use bevy_rapier2d::plugin::RapierConfiguration;
use leafwing_input_manager::prelude::ActionState;

pub fn start_mainmenu_system(
    menu_input_query: Query<&ActionState<MenuAction>, With<MainMenuExplorer>>,
    mut next_app_state: ResMut<NextState<AppStates>>,
    mut next_game_state: ResMut<NextState<GameStates>>,
) {
    // read menu input action
    let action_state = menu_input_query.single();

    // if reset input provided reset th run
    if action_state.just_released(&MenuAction::Reset) {
        // go to the main menu state
        next_app_state.set(AppStates::MainMenu);
        next_game_state.set(GameStates::Playing);
    }
}

// close pause menu if input given
pub(super) fn close_pause_menu_system(
    menu_input_query: Query<&ActionState<MenuAction>, With<MainMenuExplorer>>,
    mut next_game_state: ResMut<NextState<GameStates>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<MenuAudioChannel>>,
) {
    // read menu input action
    let action_state = menu_input_query.single();

    // pop the pause state if input read
    if action_state.just_released(&MenuAction::ExitPauseMenu) {
        next_game_state.set(GameStates::Playing);

        // play sound effect
        audio_channel.play(asset_server.load("sounds/menu_input_success.wav"));

        // resume the physics engine
        rapier_config.physics_pipeline_active = true;
        rapier_config.query_pipeline_active = true;
    }
}

pub(super) fn open_pause_menu_system(
    menu_input_query: Query<&ActionState<MenuAction>, With<MainMenuExplorer>>,
    mut next_game_state: ResMut<NextState<GameStates>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<MenuAudioChannel>>,
) {
    let action_state = menu_input_query.single();

    // switch to pause menu state if input read
    if action_state.just_released(&MenuAction::PauseGame) {
        next_game_state.set(GameStates::Paused);

        // play sound effect
        audio_channel.play(asset_server.load("sounds/menu_input_success.wav"));

        // suspend the physics engine
        rapier_config.physics_pipeline_active = false;
        rapier_config.query_pipeline_active = false;
    }
}
