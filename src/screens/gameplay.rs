//! The screen state for the main gameplay.
use crate::assets::audio_assets::Fonts;
use crate::components::audio::{BGMusicType, ChangeBackgroundMusicEvent};
use crate::gameplay::level::spawn_level as spawn_level_command;
use crate::gameplay::loot::Points;
use crate::gameplay::GameStates;
use crate::theme::interaction::OnPress;
use crate::{screens::AppStates, theme::prelude::*};
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppStates::Game), spawn_level);
    app.add_systems(OnEnter(GameStates::GameOver), setup_game_over);
    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(AppStates::Game).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_level(
    mut commands: Commands,
    mut change_bg_music_event_writer: EventWriter<ChangeBackgroundMusicEvent>,
) {
    change_bg_music_event_writer.send(ChangeBackgroundMusicEvent {
        bg_music_type: Some(BGMusicType::Game),
        loop_from: Some(0.0),
        fade_in: Some(Duration::from_secs(2)),
        fade_out: Some(Duration::from_secs(2)),
    });
    commands.add(spawn_level_command);
}

fn return_to_title_screen(
    mut next_screen: ResMut<NextState<AppStates>>,
    mut change_bg_music_event_writer: EventWriter<ChangeBackgroundMusicEvent>,
) {
    // fade music out
    change_bg_music_event_writer.send(ChangeBackgroundMusicEvent {
        fade_out: Some(Duration::from_secs(2)),
        ..default()
    });
    next_screen.set(AppStates::MainMenu);
}

fn setup_game_over(mut commands: Commands, points: Res<Points>, fonts: Res<Fonts>) {
    commands
        .ui_root()
        .insert(StateScoped(GameStates::GameOver))
        .with_children(|children| {
            children.content(format!("{}", points.into_inner()));
            children.label("points", fonts.primary.clone());
            children
                .button("Return To Title", fonts.primary.clone())
                .observe(return_title_screen);
        });
}

fn return_title_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<AppStates>>) {
    next_screen.set(AppStates::MainMenu);
}
