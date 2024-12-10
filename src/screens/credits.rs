//! A credits screen that can be accessed from the title screen.
use bevy::prelude::*;
use std::time::Duration;

use crate::assets::audio_assets::Fonts;
use crate::components::audio::{BGMusicType, ChangeBackgroundMusicEvent};
use crate::{screens::AppStates, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppStates::Credits), spawn_credits_screen);
}

fn spawn_credits_screen(
    mut commands: Commands,
    fonts: Res<Fonts>,
    mut change_bg_music_event_writer: EventWriter<ChangeBackgroundMusicEvent>,
) {
    commands
        .ui_root()
        .insert(StateScoped(AppStates::Credits))
        .with_children(|children| {
            children.header("Made by", fonts.primary.clone());
            children.label("Developer", fonts.primary.clone());

            children.header("Assets", fonts.primary.clone());
            children.label("logo", fonts.primary.clone());

            children
                .button("Back", fonts.primary.clone())
                .observe(enter_title_screen);
        });
    change_bg_music_event_writer.send(ChangeBackgroundMusicEvent {
        bg_music_type: Some(BGMusicType::Game),
        loop_from: Some(0.0),
        fade_in: Some(Duration::from_secs(2)),
        fade_out: Some(Duration::from_secs(2)),
    });
}

fn enter_title_screen(
    _trigger: Trigger<OnPress>,
    mut next_screen: ResMut<NextState<AppStates>>,
    mut change_bg_music_event_writer: EventWriter<ChangeBackgroundMusicEvent>,
) {
    // fade music out
    change_bg_music_event_writer.send(ChangeBackgroundMusicEvent {
        fade_out: Some(Duration::from_secs(1)),
        ..default()
    });
    next_screen.set(AppStates::MainMenu);
}
