use crate::screens::AppStates;
use bevy::app::App;
use bevy::asset::Handle;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_asset_loader::standard_dynamic_asset::StandardDynamicAssetCollection;
use bevy_rapier2d::plugin::{RapierConfiguration, TimestepMode};
use crate::assets::player_assets::PlayerAssets;

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

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(AppStates::Loading)
            .continue_to_state(AppStates::MainMenu)
            .load_collection::<Fonts>()
            .load_collection::<AudioAssets>()
            .load_collection::<Music>()
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                "player_assets.assets.ron",
            )
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                "projectile_assets.assets.ron",
            )
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>("mob_assets.assets.ron")
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                "consumable_assets.assets.ron",
            )
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                "item_assets.assets.ron",
            )
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                "effect_assets.assets.ron",
            )
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                "game_audio_assets.assets.ron",
            )
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>("ui_assets.assets.ron")
            .load_collection::<PlayerAssets>()
            // .load_collection::<ProjectileAssets>()//TODO:Assets need to be loaded
            // .load_collection::<MobAssets>()
            // .load_collection::<ItemAssets>()
            // .load_collection::<ConsumableAssets>()
            // .load_collection::<EffectAssets>()
            // .load_collection::<GameAudioAssets>()
            // .load_collection::<UiAssets>(),
    );

    app.edit_schedule(OnEnter(AppStates::InGame), |schedule| {
        schedule.configure_sets(
            (
                GameEnterSet::Initialize,
                GameEnterSet::BuildLevel,
                GameEnterSet::SpawnPlayer,
                GameEnterSet::BuildUi,
            )
                .chain(),
        );
    });

    app.configure_sets(
        Update,
        (
            //GameUpdateSet::Enter,
            GameUpdateSet::Level,
            GameUpdateSet::Spawn,
            GameUpdateSet::NextLevel,
            GameUpdateSet::UpdateUi,
            GameUpdateSet::SetTargetBehavior,
            GameUpdateSet::ContactCollision,
            GameUpdateSet::IntersectionCollision,
            GameUpdateSet::ExecuteBehavior,
            GameUpdateSet::ApplyDisconnectedBehaviors,
            GameUpdateSet::Movement,
            GameUpdateSet::Abilities,
            GameUpdateSet::ChangeState,
            GameUpdateSet::Cleanup,
        )
            .chain(),
    );

    app.add_systems(
        OnEnter(AppStates::InGame),
        setup_physics.in_set(GameEnterSet::Initialize),
    );
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum GameEnterSet {
    Initialize,
    BuildLevel,
    SpawnPlayer,
    BuildUi,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum GameUpdateSet {
    Enter,
    Level,
    Spawn,
    NextLevel,
    UpdateUi,
    Movement,
    Abilities,
    SetTargetBehavior, // TODO: replace with more general set
    ExecuteBehavior,
    ContactCollision,
    IntersectionCollision,
    ApplyDisconnectedBehaviors,
    ChangeState,
    Cleanup,
}

// setup rapier
fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.timestep_mode = TimestepMode::Fixed {
        dt: 1.0 / 60.0,
        substeps: 1,
    };
    rapier_config.physics_pipeline_active = true;
    rapier_config.query_pipeline_active = true;
    rapier_config.gravity = Vec2::ZERO;
}