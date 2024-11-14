use crate::config::GameConfig;
use crate::AppSet;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use std::time::Duration;

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

pub(super) fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, change_bgm.before(AppSet::RecordInput));
    app.add_systems(Update, update_bgm_volume);
    app.init_resource::<NextBgm>();
    app.init_resource::<CurrentBGM>();
}
