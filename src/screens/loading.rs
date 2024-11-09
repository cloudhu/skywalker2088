//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::prelude::*;

use crate::{
    screens::{credits::CreditsMusic, gameplay::GameplayMusic, AppState},
    ship::platform::Fonts,
    theme::{interaction::InteractionAssets, prelude::*},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Loading), spawn_loading_screen);

    app.add_systems(
        Update,
        continue_to_title_screen.run_if(in_state(AppState::Loading).and_then(all_assets_loaded)),
    );
}

fn spawn_loading_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(AppState::Loading))
        .with_children(|children| {
            children.label("Loading...").insert(Style {
                justify_content: JustifyContent::Center,
                ..default()
            });
        });
}

fn continue_to_title_screen(mut next_screen: ResMut<NextState<AppState>>) {
    next_screen.set(AppState::Title);
}

fn all_assets_loaded(
    player_assets: Option<Res<Fonts>>,
    interaction_assets: Option<Res<InteractionAssets>>,
    credits_music: Option<Res<CreditsMusic>>,
    gameplay_music: Option<Res<GameplayMusic>>,
) -> bool {
    player_assets.is_some()
        && interaction_assets.is_some()
        && credits_music.is_some()
        && gameplay_music.is_some()
}
