//! The screen state for the main gameplay.

use crate::assets::Music;
use crate::audio::NextBgm;
use crate::gameplay::level::spawn_level as spawn_level_command;
use crate::gameplay::loot::Points;
use crate::gameplay::GameState;
use crate::theme::interaction::OnPress;
use crate::{screens::AppState, theme::prelude::*};
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::InGame), spawn_level);
    app.add_systems(OnEnter(AppState::InGame), play_gameplay_music);
    app.add_systems(OnExit(AppState::InGame), stop_music);
    app.add_systems(OnEnter(GameState::GameOver), setup_game_over);
    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(AppState::InGame).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_level(mut commands: Commands) {
    commands.add(spawn_level_command);
}

fn play_gameplay_music(mut next_bgm: ResMut<NextBgm>, music: Res<Music>) {
    *next_bgm = NextBgm(Some(music.gameplay.clone()));
}

fn stop_music(mut next_bgm: ResMut<NextBgm>) {
    *next_bgm = NextBgm(None);
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<AppState>>) {
    next_screen.set(AppState::Title);
}

fn setup_game_over(mut commands: Commands, points: Res<Points>) {
    commands
        .ui_root()
        .insert(StateScoped(GameState::GameOver))
        .with_children(|children| {
            children.label(format!("{} points!", points.into_inner()));
            children
                .button("Return To Title")
                .observe(return_title_screen);
        });
}

fn return_title_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<AppState>>) {
    next_screen.set(AppState::Title);
}
