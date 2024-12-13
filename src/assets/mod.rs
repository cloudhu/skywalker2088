pub mod audio_assets;
pub mod consumable;
pub mod effect;
pub mod enemy_assets;
pub mod item;
pub mod player_assets;
pub mod projectile;
pub mod ui;

use crate::{
    assets::{
        audio_assets::{AudioAssets, Fonts, GameAudioAssets, Music},
        consumable::ConsumableAssets,
        effect::EffectAssets,
        enemy_assets::MobAssets,
        item::ItemAssets,
        player_assets::PlayerAssets,
        projectile::ProjectileAssets,
        ui::UiAssets,
    },
    components::abilities::{AbilitiesResource, AbilityDescriptionsResource, ActivateAbilityEvent},
    components::character::CharactersResource,
    components::player::{InputRestrictionsAtSpawn, PlayersResource},
    screens::AppStates,
};
use bevy::app::App;
use bevy::asset::ron::de::from_bytes;
use bevy_asset_loader::loading_state::LoadingState;
use bevy_asset_loader::prelude::{
    ConfigureLoadingState, LoadingStateAppExt, StandardDynamicAssetCollection,
};
// use leafwing_input_manager::prelude::InputManagerPlugin;

pub(super) fn plugin(app: &mut App) {
    // app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
    app.add_event::<ActivateAbilityEvent>();
    app.add_loading_state(
        LoadingState::new(AppStates::Loading)
            .continue_to_state(AppStates::MainMenu)
            .load_collection::<Fonts>()
            .load_collection::<AudioAssets>()
            .load_collection::<Music>()
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>("player_assets.assets.ron")
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                "projectile_assets.assets.ron",
            )
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>("mob_assets.assets.ron")
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                "consumable_assets.assets.ron",
            )
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>("item_assets.assets.ron")
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>("effect_assets.assets.ron")
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                "game_audio_assets.assets.ron",
            )
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>("ui_assets.assets.ron")
            .load_collection::<PlayerAssets>()
            .load_collection::<ProjectileAssets>()
            .load_collection::<MobAssets>()
            .load_collection::<ItemAssets>()
            .load_collection::<ConsumableAssets>()
            .load_collection::<EffectAssets>()
            .load_collection::<GameAudioAssets>()
            .load_collection::<UiAssets>(),
    );
    app.insert_resource(
        from_bytes::<CharactersResource>(include_bytes!("../../assets/data/characters.ron"))
            .unwrap(),
    );

    app.insert_resource(
        from_bytes::<AbilitiesResource>(include_bytes!("../../assets/data/abilities.ron")).unwrap(),
    );

    app.insert_resource(
        from_bytes::<AbilityDescriptionsResource>(include_bytes!(
            "../../assets/data/ability_descriptions.ron"
        ))
        .unwrap(),
    );

    app.insert_resource(PlayersResource::default())
        .insert_resource(InputRestrictionsAtSpawn::default());
}
