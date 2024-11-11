//! The screen state for the main gameplay.

use crate::gameplay::level::spawn_level as spawn_level_command;
use crate::gameplay::loot::Points;
use crate::gameplay::GameState;
use crate::ship::platform::Fonts;
use crate::theme::interaction::OnPress;
use crate::{asset_tracking::LoadResource, audio::Music, screens::AppState, theme::prelude::*};
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::InGame), spawn_level);

    app.load_resource::<GameplayMusic>();
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

fn setup_game_over(mut commands: Commands, fonts: Res<Fonts>, points: Res<Points>) {
    commands
        .ui_root()
        .insert(StateScoped(GameState::GameOver))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("{} points!", points.into_inner()),
                TextStyle {
                    font: fonts.primary.clone(),
                    font_size: 30.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                },
            ));
            parent
                .button("Return To Title")
                .observe(return_title_screen);
        });
}

fn return_title_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<AppState>>) {
    next_screen.set(AppState::Title);
}
