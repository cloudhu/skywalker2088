//! Exposes a plugin that starts, stops, and modulates in-game audio when events are emitted
use crate::config::GameConfig;
use crate::AppSet;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use std::time::Duration;
use crate::assets::audio::GameAudioAssets;
use crate::components::audio::{ChangeBackgroundMusicEvent, PlaySoundEffectEvent};
use crate::screens::AppStates;

const SPATIAL_AUDIO_MAX_DISTANCE: f32 = 400.0;

pub fn play_sound_effects(
    audio: &Res<Audio>,
    config: &GameConfig,
    source: Handle<AudioSource>,
    position: &Option<Vec2>,
    camera_position: Vec2,
) {
    let volume = match position {
        None => 1.0,
        Some(p) => {
            1.0 - ((*p - camera_position).length() / SPATIAL_AUDIO_MAX_DISTANCE)
                .clamp(0.0, 1.0)
                .powf(2.0)
        }
    };

    let panning = match position {
        None => 0.5,
        Some(p) => 0.5 + ((p.x - camera_position.x) / SPATIAL_AUDIO_MAX_DISTANCE).clamp(-0.5, 0.5),
    };

    audio
        .play(source.clone())
        .with_volume(Volume::Amplitude((config.sfx_volume * volume) as f64))
        .with_panning(panning as f64)
        .handle();
}

/// 表示下一个播放的BGM的资源
#[derive(Resource, Default)]
pub struct NextBgm(pub Option<Handle<AudioSource>>);

struct SourceAndInstance {
    instance: Handle<AudioInstance>,
    source: Handle<AudioSource>,
}

#[derive(Resource, Default)]
struct CurrentBGM(Option<SourceAndInstance>);

fn change_bgm(
    mut current_bgm: ResMut<CurrentBGM>,
    next_bgm: ResMut<NextBgm>,
    audio: Res<Audio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    config: Res<GameConfig>,
) {
    let NextBgm(ref next_bgm_or_none) = *next_bgm;

    if let Some(ref mut current_handle) = &mut current_bgm.0 {
        if let Some(ref next) = *next_bgm_or_none {
            if current_handle.source.id() != next.id() {
                if let Some(instance) = audio_instances.get_mut(&current_handle.instance) {
                    instance.stop(AudioTween::new(Duration::from_secs(1), AudioEasing::Linear));
                }
                let instance = audio
                    .play(next.clone())
                    .with_volume(Volume::Amplitude(config.bgm_volume as f64))
                    .looped()
                    .handle();
                current_bgm.0 = Some(SourceAndInstance {
                    instance: instance.clone(),
                    source: next.clone(),
                });
            }
        }
    } else if let Some(ref next) = *next_bgm_or_none {
        let instance = audio
            .play(next.clone())
            .with_volume(Volume::Amplitude(config.bgm_volume as f64))
            .looped()
            .handle();
        current_bgm.0 = Some(SourceAndInstance {
            instance: instance.clone(),
            source: next.clone(),
        });
    }
}

fn update_bgm_volume(
    mut current_bgm: ResMut<CurrentBGM>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    config: Res<GameConfig>,
) {
    if config.is_changed() {
        if let Some(ref mut current_handle) = &mut current_bgm.0 {
            if let Some(instance) = audio_instances.get_mut(&current_handle.instance) {
                instance.set_volume(
                    Volume::Amplitude(config.bgm_volume as f64),
                    AudioTween::linear(Duration::from_millis(100)),
                );
            }
        }
    }
}

/// Starts, stops, and modulates in-game audio when we receive a `PlaySoundEffectEvent` or `ChangeBackgroundMusicEvent`.
pub(super) fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, change_bgm.before(AppSet::RecordInput));
    app.add_systems(Update, update_bgm_volume);
    app.init_resource::<NextBgm>();
    app.init_resource::<CurrentBGM>();
    app.add_event::<PlaySoundEffectEvent>();
    app.add_event::<ChangeBackgroundMusicEvent>();

    app.add_audio_channel::<BackgroundMusicAudioChannel>()
        .add_audio_channel::<MenuAudioChannel>()
        .add_audio_channel::<SoundEffectsAudioChannel>();

    app.add_systems(Startup, set_audio_volume_system);

    app.add_systems(
        Update,
        (play_sound_effect_system, change_bg_music_system)
            .run_if(not(in_state(AppStates::LoadingAssets))),
    );
}

// audio channels
#[derive(Resource)]
pub struct BackgroundMusicAudioChannel;
#[derive(Resource)]
pub struct MenuAudioChannel;
#[derive(Resource)]
pub struct SoundEffectsAudioChannel;

/// Sets the volume of the audio channels to "sane defaults"
fn set_audio_volume_system(
    background_audio_channel: Res<AudioChannel<BackgroundMusicAudioChannel>>,
    menu_audio_channel: Res<AudioChannel<MenuAudioChannel>>,
    effects_audio_channel: Res<AudioChannel<SoundEffectsAudioChannel>>,
) {
    background_audio_channel.set_volume(0.20);
    menu_audio_channel.set_volume(0.05);
    effects_audio_channel.set_volume(0.80);
}

/// Play sound effects when we receive events. This should be called every frame for snappy audio.
fn play_sound_effect_system(
    mut play_sound_event_reader: EventReader<PlaySoundEffectEvent>,
    audio_channel: Res<AudioChannel<SoundEffectsAudioChannel>>,
    audio_assets: Res<GameAudioAssets>,
) {
    for event in play_sound_event_reader.read() {
        audio_channel.play(audio_assets.get_sound_effect(&event.sound_effect_type));
    }
}

/// System to handle background music changes based on events.
///
/// This system listens for `ChangeBackgroundMusicEvent` events and updates
/// the background music accordingly. It can stop the current music, start new
/// music, handle looping, and apply fade-in and fade-out effects if specified in the event.
///
/// - If an event specifies a fade-out duration, the current track will fade out before stopping.
/// - If a new background music type is provided, it will play the corresponding track from the `GameAudioAssets`.
/// - The system supports looping the new track from a specified point and applying a fade-in effect if specified.
///
/// Parameters:
/// - `EventReader<ChangeBackgroundMusicEvent>`: Reads events that dictate when and how to change background music.
/// - `AudioChannel<BackgroundMusicAudioChannel>`: Controls the background music audio channel, allowing for stop, play, and fade effects.
/// - `GameAudioAssets`: A resource that holds all available audio assets.
fn change_bg_music_system(
    mut change_bg_music_event_reader: EventReader<ChangeBackgroundMusicEvent>,
    audio_channel: Res<AudioChannel<BackgroundMusicAudioChannel>>,
    audio_assets: Res<GameAudioAssets>,
) {
    for event in change_bg_music_event_reader.read() {
        // stop audio if playing sound
        if audio_channel.is_playing_sound() {
            let mut stop_command = audio_channel.stop();

            // use fade if specified
            if let Some(fade_out) = event.fade_out {
                stop_command.fade_out(AudioTween::new(fade_out, AudioEasing::Linear));
            }
        }

        // play music if provided a type
        if let Some(bg_music_type) = event.bg_music_type.clone() {
            let mut start_command =
                audio_channel.play(audio_assets.get_bg_music_asset(&bg_music_type));

            // loop if true
            if let Some(loop_from) = event.loop_from {
                start_command.loop_from(loop_from);
            }

            // use fade if specified
            if let Some(fade_in) = event.fade_in {
                start_command.fade_in(AudioTween::new(fade_in, AudioEasing::Linear));
            }
        }
    }
}