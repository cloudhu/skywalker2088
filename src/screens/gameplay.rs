//! The screen state for the main gameplay.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::gameplay::level::spawn_level as spawn_level_command;
use crate::{asset_tracking::LoadResource, audio::Music, screens::AppState};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::InGame), spawn_level);

    app.load_resource::<GameplayMusic>();
    app.add_systems(OnEnter(AppState::InGame), play_gameplay_music);
    app.add_systems(OnExit(AppState::InGame), stop_music);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(AppState::InGame).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_level(mut commands: Commands) {
    commands.add(spawn_level_command);
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct GameplayMusic {
    #[dependency]
    handle: Handle<AudioSource>,
    entity: Option<Entity>,
}

impl FromWorld for GameplayMusic {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            handle: assets.load("audio/music/Fluffing A Duck.ogg"),
            entity: None,
        }
    }
}

fn play_gameplay_music(mut commands: Commands, mut music: ResMut<GameplayMusic>) {
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

fn stop_music(mut commands: Commands, mut music: ResMut<GameplayMusic>) {
    if let Some(entity) = music.entity.take() {
        commands.entity(entity).despawn_recursive();
    }
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<AppState>>) {
    next_screen.set(AppState::Title);
}
