//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use crate::components::states::AppStates;
use crate::theme::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppStates::LoadingAssets), spawn_loading_screen);
}

fn spawn_loading_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(AppStates::LoadingAssets))
        .with_children(|children| {
            children.content("Loading...").insert(Style {
                justify_content: JustifyContent::Center,
                ..default()
            });
        });
}
