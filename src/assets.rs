use crate::screens::AppState;
use bevy::app::App;
use bevy::asset::Handle;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(Resource, AssetCollection)]
pub struct Fonts {
    #[asset(path = "fonts/song_GB2312.ttf")]
    pub primary: Handle<Font>,
    #[asset(path = "fonts/DejaVuLGCSansMono.ttf")]
    pub unicode: Handle<Font>,
}

#[allow(dead_code)]
#[derive(Resource, AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/sound_effects/big_explosion.ogg")]
    pub(crate) big_explosion: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/bullet_explosion.ogg")]
    pub(crate) bullet_explosion: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/bullet_hit_1.ogg")]
    pub(crate) bullet_hit_1: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/bullet_hit_2.ogg")]
    pub(crate) bullet_hit_2: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/game_over.ogg")]
    pub(crate) game_over: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/game_pause.ogg")]
    pub(crate) game_pause: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/mode_switch.ogg")]
    pub(crate) mode_switch: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/player_fire.ogg")]
    pub(crate) player_fire: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/powerup_appear.ogg")]
    pub(crate) powerup_appear: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/powerup_pick.ogg")]
    pub(crate) powerup_pick: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/sfx_laser1.ogg")]
    pub(crate) laser1: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/sfx_laser2.ogg")]
    pub(crate) laser2: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/sfx_lose.ogg")]
    pub(crate) lose: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/sfx_shieldDown.ogg")]
    pub(crate) shield_down: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/sfx_shieldUp.ogg")]
    pub(crate) shield_up: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/sfx_twoTone.ogg")]
    pub(crate) two_tone: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/sfx_zap.ogg")]
    pub(crate) zap: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/button_hover.ogg")]
    pub(crate) hover: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/button_press.ogg")]
    pub(crate) press: Handle<AudioSource>,
}

#[derive(Resource, AssetCollection)]
pub struct Music {
    #[asset(path = "audio/music/Monkeys Spinning Monkeys.ogg")]
    pub monkeys: Handle<AudioSource>,
    #[asset(path = "audio/music/start_menu.ogg")]
    pub title: Handle<AudioSource>,
    #[asset(path = "audio/music/Fluffing A Duck.ogg")]
    pub gameplay: Handle<AudioSource>,
}

pub(crate) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(AppState::Loading)
            .continue_to_state(AppState::Title)
            .load_collection::<Fonts>()
            .load_collection::<AudioAssets>()
            .load_collection::<Music>(),
    );
}
