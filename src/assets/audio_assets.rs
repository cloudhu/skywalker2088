use crate::components::audio::{BGMusicType, CollisionSoundType, SoundEffectType};
use bevy::asset::Handle;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use rand::Rng;

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

/// Collection of all audio assets in the game including sound effects and background music
#[derive(AssetCollection, Resource)]
pub struct GameAudioAssets {
    #[asset(key = "sounds.main_music")]
    pub main_music: Handle<AudioSource>,
    #[asset(key = "sounds.game_music")]
    pub game_music: Handle<AudioSource>,
    #[asset(key = "sounds.boss_music")]
    pub boss_music: Handle<AudioSource>,
    #[asset(key = "sounds.boss_trans_music")]
    pub boss_trans_music: Handle<AudioSource>,
    #[asset(key = "sounds.barrier_bounce")]
    pub barrier_bounce: Handle<AudioSource>,
    #[asset(key = "sounds.collision")]
    pub collision: Handle<AudioSource>,
    #[asset(key = "sounds.squishy_collision")]
    pub squishy_collision: Handle<AudioSource>,
    #[asset(key = "sounds.consumable_pickup")]
    pub consumable_pickup: Handle<AudioSource>,
    #[asset(key = "sounds.defense_damage")]
    pub defense_damage: Handle<AudioSource>,
    #[asset(key = "sounds.defense_heal")]
    pub defense_heal: Handle<AudioSource>,
    #[asset(key = "sounds.enemy_fire_blast")]
    pub enemy_fire_blast: Handle<AudioSource>,
    #[asset(key = "sounds.menu_input_success")]
    pub menu_input_success: Handle<AudioSource>,
    #[asset(key = "sounds.mob_explosion")]
    pub mob_explosion: Handle<AudioSource>,
    #[asset(key = "sounds.mob_hit")]
    pub mob_hit: Handle<AudioSource>,
    #[asset(key = "sounds.player_explosion")]
    pub player_explosion: Handle<AudioSource>,
    #[asset(key = "sounds.player_fire_blast")]
    pub player_fire_blast: Handle<AudioSource>,
    #[asset(key = "sounds.player_hit")]
    pub player_hit: Handle<AudioSource>,
    #[asset(key = "sounds.bullet_ding")]
    pub bullet_ding: Handle<AudioSource>,
    #[asset(key = "sounds.bullet_bounce")]
    pub bullet_bounce: Handle<AudioSource>,
    #[asset(key = "sounds.megablast_ability")]
    pub megablast_ability: Handle<AudioSource>,
    #[asset(key = "sounds.objective_completed")]
    pub objective_completed: Handle<AudioSource>,
    #[asset(key = "sounds.button_select_1")]
    pub button_select_1: Handle<AudioSource>,
    #[asset(key = "sounds.button_select_2")]
    pub button_select_2: Handle<AudioSource>,
    #[asset(key = "sounds.button_select_3")]
    pub button_select_3: Handle<AudioSource>,
    #[asset(key = "sounds.button_select_4")]
    pub button_select_4: Handle<AudioSource>,
    #[asset(key = "sounds.button_select_5")]
    pub button_select_5: Handle<AudioSource>,
    #[asset(key = "sounds.button_release_1")]
    pub button_release_1: Handle<AudioSource>,
    #[asset(key = "sounds.button_release_2")]
    pub button_release_2: Handle<AudioSource>,
    #[asset(key = "sounds.button_release_3")]
    pub button_release_3: Handle<AudioSource>,
    #[asset(key = "sounds.button_confirm")]
    pub button_confirm: Handle<AudioSource>,
}

impl GameAudioAssets {
    /// Use a BGMusicType enum to access a handle for a track of music
    pub fn get_bg_music_asset(&self, bg_music_type: &BGMusicType) -> Handle<AudioSource> {
        match bg_music_type {
            BGMusicType::Game => self.game_music.clone(),
            BGMusicType::Boss => self.boss_music.clone(),
            BGMusicType::BossTransition => self.boss_trans_music.clone(),
            BGMusicType::Main => self.main_music.clone(),
        }
    }

    /// Use a SoundEffectType enum to access a handle for a sound effect
    /// Sound effects that produced a randomized sound will we return a random effect from a subset
    pub fn get_sound_effect(&self, sound_type: &SoundEffectType) -> Handle<AudioSource> {
        match sound_type {
            SoundEffectType::Collision(collsion_type) => match collsion_type {
                CollisionSoundType::Squishy => self.squishy_collision.clone(),
                CollisionSoundType::Normal => self.collision.clone(),
            },
            SoundEffectType::BarrierBounce => self.barrier_bounce.clone(),
            SoundEffectType::ConsumablePickup => self.consumable_pickup.clone(),
            SoundEffectType::DefenseDamage => self.defense_damage.clone(),
            SoundEffectType::DefenseHeal => self.defense_heal.clone(),
            SoundEffectType::EnemyFireBlast => self.enemy_fire_blast.clone(),
            SoundEffectType::MenuInputSuccess => self.menu_input_success.clone(),
            SoundEffectType::MobExplosion => self.mob_explosion.clone(),
            SoundEffectType::MobHit => self.mob_hit.clone(),
            SoundEffectType::PlayerExplosion => self.player_explosion.clone(),
            SoundEffectType::PlayerFireBlast => self.player_fire_blast.clone(),
            SoundEffectType::PlayerHit => self.player_hit.clone(),
            SoundEffectType::BulletDing => self.bullet_ding.clone(),
            SoundEffectType::BulletBounce => self.bullet_bounce.clone(),
            SoundEffectType::MegaBlastAbility => self.megablast_ability.clone(),
            SoundEffectType::ObjectiveCompleted => self.objective_completed.clone(),
            SoundEffectType::ButtonRelease => {
                let idx: u8 = rand::thread_rng().gen_range(1..=3);
                match idx {
                    1 => self.button_release_1.clone(),
                    2 => self.button_release_2.clone(),
                    _ => self.button_release_3.clone(),
                }
            }
            SoundEffectType::ButtonSelect => {
                let idx: u8 = rand::thread_rng().gen_range(1..=5);
                match idx {
                    1 => self.button_select_1.clone(),
                    2 => self.button_select_2.clone(),
                    3 => self.button_select_3.clone(),
                    4 => self.button_select_4.clone(),
                    _ => self.button_select_5.clone(),
                }
            }
            SoundEffectType::ButtonConfirm => self.button_confirm.clone(),
        }
    }
}
