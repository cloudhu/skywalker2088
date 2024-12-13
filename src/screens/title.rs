//! The title screen that appears when the game starts.

use crate::assets::audio_assets::Fonts;
use crate::components::audio::{BGMusicType, ChangeBackgroundMusicEvent};
use crate::config::GameConfig;
use crate::{screens::AppStates, theme::prelude::*};
use bevy::prelude::*;
use bevy::window::WindowMode;
use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppStates::MainMenu), setup_title_screen);
}

fn setup_title_screen(
    mut commands: Commands,
    mut localize: ResMut<Localize>,
    config: Res<GameConfig>,
    fonts: Res<Fonts>,
    mut change_bg_music_event_writer: EventWriter<ChangeBackgroundMusicEvent>,
) {
    localize.set_language(config.language.clone());
    commands
        .ui_root()
        .insert(StateScoped(AppStates::MainMenu))
        .with_children(|children| {
            children
                .button("Play", fonts.primary.clone())
                .observe(enter_gameplay_screen);
            children
                .button("Credits", fonts.primary.clone())
                .observe(enter_credits_screen);
            children
                .button("Duolingo", fonts.primary.clone())
                .observe(set_lang);
            children
                .button("Toggle Fullscreen", fonts.primary.clone())
                .observe(toggle_fullscreen);
            #[cfg(not(target_family = "wasm"))]
            children
                .button("Exit", fonts.primary.clone())
                .observe(exit_app);
        });
    // change music
    change_bg_music_event_writer.send(ChangeBackgroundMusicEvent {
        bg_music_type: Some(BGMusicType::Main),
        loop_from: Some(0.0),
        fade_in: Some(Duration::from_secs(2)),
        fade_out: Some(Duration::from_secs(2)),
    });
}

fn enter_gameplay_screen(
    _trigger: Trigger<OnPress>,
    mut next_screen: ResMut<NextState<AppStates>>,
) {
    next_screen.set(AppStates::Game);
}

fn enter_credits_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<AppStates>>) {
    next_screen.set(AppStates::Credits);
}

fn set_lang(
    _trigger: Trigger<OnPress>,
    mut config: ResMut<GameConfig>,
    mut localize: ResMut<Localize>,
) {
    match config.language.as_str() {
        "English" => {
            config.set_lang("Chinese");
        }
        "Chinese" => {
            config.set_lang("English");
        }
        &_ => {}
    }
    localize.set_language(config.language.clone());
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_trigger: Trigger<OnPress>, mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit::Success);
}

fn toggle_fullscreen(_trigger: Trigger<OnPress>, mut window: Single<&mut Window>) {
    window.mode = match window.mode {
        WindowMode::Windowed => WindowMode::SizedFullscreen(MonitorSelection::Current),
        WindowMode::BorderlessFullscreen(MonitorSelection::Current) => WindowMode::Windowed,
        WindowMode::SizedFullscreen(MonitorSelection::Current) => WindowMode::Windowed,
        WindowMode::Fullscreen(MonitorSelection::Current) => WindowMode::Windowed,
        _ => WindowMode::Windowed,
    };
}
