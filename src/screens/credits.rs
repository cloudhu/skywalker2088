//! A credits screen that can be accessed from the title screen.
use bevy::prelude::*;

use crate::assets::Music;
use crate::audio::NextBgm;
use crate::{screens::AppState, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Credits), spawn_credits_screen);

    app.add_systems(OnEnter(AppState::Credits), play_credits_music);
    app.add_systems(OnExit(AppState::Credits), stop_music);
}

fn spawn_credits_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(AppState::Credits))
        .with_children(|children| {
            children.header("Made by");
            children.label("Joe Shmoe - Implemented aligator wrestling AI");
            children.label("Jane Doe - Made the music for the alien invasion");

            children.header("Assets");
            children.label("Bevy logo - All rights reserved by the Bevy Foundation. Permission granted for splash screen use when unmodified.");
            children.label("Ducky sprite - CC0 by Caz Creates Games");
            children.label("Button SFX - CC0 by Jaszunio15");
            children.label("Music - CC BY 3.0 by Kevin MacLeod");

            children.button("Back").observe(enter_title_screen);
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
