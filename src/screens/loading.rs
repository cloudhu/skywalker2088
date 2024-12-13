//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::prelude::*;

use crate::{screens::AppStates, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppStates::Loading), spawn_loading_screen);
}

fn spawn_loading_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(AppStates::Loading))
        .with_children(|children| {
            children.content("Loading...").insert(TextLayout {
                justify: JustifyText::Center,
                ..default()
            });
        });
}
