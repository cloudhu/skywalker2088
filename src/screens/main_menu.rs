//! The title screen that appears when the game starts.
use crate::assets::game_assets::{AppStates, Fonts, Music};
use crate::config::GameConfig;
use crate::theme::prelude::*;
use bevy::prelude::*;
use bevy::window::WindowMode;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppStates::MainMenu), spawn_title_screen);
}

fn spawn_title_screen(
    mut commands: Commands,
    mut localize: ResMut<Localize>,
    config: Res<GameConfig>,
    fonts: Res<Fonts>,
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
            #[cfg(not(target_family = "wasm"))]
            children
                .button("Toggle Fullscreen", fonts.primary.clone())
                .observe(toggle_fullscreen);
            #[cfg(not(target_family = "wasm"))]
            children
                .button("Exit", fonts.primary.clone())
                .observe(exit_app);
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

#[cfg(not(target_family = "wasm"))]
fn toggle_fullscreen(_trigger: Trigger<OnPress>, mut window_query: Query<&mut Window>) {
    let mut window = window_query.single_mut();
    window.mode = match window.mode {
        WindowMode::Windowed => WindowMode::SizedFullscreen,
        WindowMode::BorderlessFullscreen => WindowMode::Windowed,
        WindowMode::SizedFullscreen => WindowMode::Windowed,
        WindowMode::Fullscreen => WindowMode::Windowed,
    };
}