//! A credits screen that can be accessed from the title screen.
use bevy::prelude::*;

use crate::assets::{Fonts, Music};
use crate::audio::NextBgm;
use crate::{screens::AppState, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Credits), spawn_credits_screen);

    app.add_systems(OnEnter(AppState::Credits), play_credits_music);
    app.add_systems(OnExit(AppState::Credits), stop_music);
}

fn spawn_credits_screen(mut commands: Commands, fonts: Res<Fonts>) {
    commands
        .ui_root()
        .insert(StateScoped(AppState::Credits))
        .with_children(|children| {
            children.header("Made by", fonts.primary.clone());
            children.label("Developer", fonts.primary.clone());

            children.header("Assets", fonts.primary.clone());
            children.label("logo", fonts.primary.clone());

            children
                .button("Back", fonts.primary.clone())
                .observe(enter_title_screen);
        });
}

fn enter_title_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<AppState>>) {
    next_screen.set(AppState::Title);
}

fn play_credits_music(mut next_bgm: ResMut<NextBgm>, music: Res<Music>) {
    *next_bgm = NextBgm(Some(music.monkeys.clone()));
}

fn stop_music(mut next_bgm: ResMut<NextBgm>) {
    *next_bgm = NextBgm(None);
}
