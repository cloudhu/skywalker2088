use crate::asset_tracking::LoadResource;
use crate::audio::SoundEffect;
use crate::components::common::Health;
use crate::gameplay::effects::HitFlash;
use crate::gameplay::gamelogic::{DespawnWithScene, ExplodesOnDespawn, Targettable, WillTarget};
use crate::gameplay::physics::{Collider, Physics};
use crate::ship::engine::Engine;
use bevy::app::App;
use bevy::asset::{Asset, AssetServer, Handle};
use bevy::audio::{AudioBundle, AudioSource, PlaybackSettings};
use bevy::prelude::{Bundle, Commands, Font, FromWorld, Reflect, Resource, Text2dBundle, World};

// Bundles
#[derive(Bundle, Default)]
pub struct ShipBundle {
    pub glyph: Text2dBundle,
    pub physics: Physics,
    pub engine: Engine,
    pub health: Health,
    pub collider: Collider,
    pub targettable: Targettable,
    pub will_target: WillTarget,
    pub despawn_with_scene: DespawnWithScene,
    pub explodes_on_despawn: ExplodesOnDespawn,
    pub hit_flash: HitFlash,
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct Fonts {
    // This #[dependency] attribute marks the field as a dependency of the Asset.
    // This means that it will not finish loading until the labeled asset is also loaded.
    #[dependency]
    pub primary: Handle<Font>,
    #[dependency]
    pub unicode: Handle<Font>,
}

impl Fonts {
    pub const PATH_PRIMARY: &'static str = "fonts/song_GB2312.ttf";
    pub const PATH_UNICODE: &'static str = "fonts/DejaVuLGCSansMono.ttf";
}

impl FromWorld for Fonts {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            primary: assets.load(Fonts::PATH_PRIMARY),
            unicode: assets.load(Fonts::PATH_UNICODE),
        }
    }
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct SoundAssets {
    #[dependency]
    pub(crate) big_explosion: Handle<AudioSource>,
    #[dependency]
    pub(crate) bullet_explosion: Handle<AudioSource>,
    #[dependency]
    pub(crate) bullet_hit_1: Handle<AudioSource>,
    #[dependency]
    pub(crate) bullet_hit_2: Handle<AudioSource>,
    #[dependency]
    pub(crate) game_over: Handle<AudioSource>,
    #[dependency]
    pub(crate) game_pause: Handle<AudioSource>,
    #[dependency]
    pub(crate) mode_switch: Handle<AudioSource>,
    #[dependency]
    pub(crate) player_fire: Handle<AudioSource>,
    #[dependency]
    pub(crate) powerup_appear: Handle<AudioSource>,
    #[dependency]
    pub(crate) powerup_pick: Handle<AudioSource>,
    #[dependency]
    pub(crate) laser1: Handle<AudioSource>,
    #[dependency]
    pub(crate) laser2: Handle<AudioSource>,
    #[dependency]
    pub(crate) lose: Handle<AudioSource>,
    #[dependency]
    pub(crate) shield_down: Handle<AudioSource>,
    #[dependency]
    pub(crate) shield_up: Handle<AudioSource>,
    #[dependency]
    pub(crate) two_tone: Handle<AudioSource>,
    #[dependency]
    pub(crate) zap: Handle<AudioSource>,
}

impl SoundAssets {
    pub const PATH_BIG_EXPLOSION: &'static str = "audio/sound_effects/big_explosion.ogg";
    pub const PATH_BULLET_EXPLOSION: &'static str = "audio/sound_effects/bullet_explosion.ogg";
    pub const PATH_BULLET_HIT_1: &'static str = "audio/sound_effects/bullet_hit_1.ogg";
    pub const PATH_BULLET_HIT_2: &'static str = "audio/sound_effects/bullet_hit_2.ogg";
    pub const PATH_GAME_OVER: &'static str = "audio/sound_effects/game_over.ogg";
    pub const PATH_GAME_PAUSE: &'static str = "audio/sound_effects/game_pause.ogg";
    pub const PATH_MODE_SWITCH: &'static str = "audio/sound_effects/mode_switch.ogg";
    pub const PATH_PLAYER_FIRE: &'static str = "audio/sound_effects/player_fire.ogg";
    pub const PATH_POWERUP_APPEAR: &'static str = "audio/sound_effects/powerup_appear.ogg";
    pub const PATH_POWERUP_PICK: &'static str = "audio/sound_effects/powerup_pick.ogg";
    pub const PATH_SFX_LASER1: &'static str = "audio/sound_effects/sfx_laser1.ogg";
    pub const PATH_SFX_LASER2: &'static str = "audio/sound_effects/sfx_laser2.ogg";
    pub const PATH_SFX_LOSE: &'static str = "audio/sound_effects/sfx_lose.ogg";
    pub const PATH_SFX_SHIELD_DOWN: &'static str = "audio/sound_effects/sfx_shieldDown.ogg";
    pub const PATH_SFX_SHIELD_UP: &'static str = "audio/sound_effects/sfx_shieldUp.ogg";
    pub const PATH_SFX_TWO_TONE: &'static str = "audio/sound_effects/sfx_twoTone.ogg";
    pub const PATH_SFX_ZAP: &'static str = "audio/sound_effects/sfx_zap.ogg";
}

impl FromWorld for SoundAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            big_explosion: assets.load(Self::PATH_BIG_EXPLOSION),
            bullet_explosion: assets.load(Self::PATH_BULLET_EXPLOSION),
            bullet_hit_1: assets.load(Self::PATH_BULLET_HIT_1),
            bullet_hit_2: assets.load(Self::PATH_BULLET_HIT_2),
            game_over: assets.load(Self::PATH_GAME_OVER),
            game_pause: assets.load(Self::PATH_GAME_PAUSE),
            mode_switch: assets.load(Self::PATH_MODE_SWITCH),
            player_fire: assets.load(Self::PATH_PLAYER_FIRE),
            powerup_appear: assets.load(Self::PATH_POWERUP_APPEAR),
            powerup_pick: assets.load(Self::PATH_POWERUP_PICK),
            laser1: assets.load(Self::PATH_SFX_LASER1),
            laser2: assets.load(Self::PATH_SFX_LASER2),
            lose: assets.load(Self::PATH_SFX_LOSE),
            shield_down: assets.load(Self::PATH_SFX_SHIELD_DOWN),
            shield_up: assets.load(Self::PATH_SFX_SHIELD_UP),
            two_tone: assets.load(Self::PATH_SFX_TWO_TONE),
            zap: assets.load(Self::PATH_SFX_ZAP),
        }
    }
}
pub(super) fn plugin(app: &mut App) {
    app.load_resource::<Fonts>();
    app.load_resource::<SoundAssets>();
}

pub fn play_sound_effects(commands: &mut Commands, source: Handle<AudioSource>) {
    commands.spawn((
        AudioBundle {
            source,
            settings: PlaybackSettings::DESPAWN,
        },
        SoundEffect,
    ));
}
