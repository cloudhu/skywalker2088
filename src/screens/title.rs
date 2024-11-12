//! The title screen that appears when the game starts.
use crate::asset_tracking::LoadResource;
use crate::audio::Music;
use crate::{screens::AppState, theme::prelude::*};
use bevy::prelude::*;

#[derive(Resource, Asset, Reflect, Clone)]
pub struct TitleMusic {
    #[dependency]
    handle: Handle<AudioSource>,
    entity: Option<Entity>,
}

impl FromWorld for TitleMusic {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            handle: assets.load("audio/music/start_menu.ogg"),
            entity: None,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<TitleMusic>();
    app.add_systems(OnEnter(AppState::Title), spawn_title_screen);
    app.add_systems(OnEnter(AppState::Title), play_title_music);
    app.add_systems(OnExit(AppState::Title), stop_title_music);
}

fn spawn_title_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(AppState::Title))
        .with_children(|children| {
            children.button("Play").observe(enter_gameplay_screen);
            children.button("Credits").observe(enter_credits_screen);

            #[cfg(not(target_family = "wasm"))]
            children.button("Exit").observe(exit_app);
        });
}

fn enter_gameplay_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<AppState>>) {
    next_screen.set(AppState::InGame);
}

fn enter_credits_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<AppState>>) {
    next_screen.set(AppState::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_trigger: Trigger<OnPress>, mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit::Success);
}

fn play_title_music(mut commands: Commands, mut music: ResMut<TitleMusic>) {
    music.entity = Some(
        commands
            .spawn((
                AudioBundle {
                    source: music.handle.clone(),
                    settings: PlaybackSettings::LOOP,
                },
                Music,
            ))
            .id(),
    );
}

fn stop_title_music(mut commands: Commands, mut music: ResMut<TitleMusic>) {
    if let Some(entity) = music.entity.take() {
        commands.entity(entity).despawn_recursive();
    }
}
