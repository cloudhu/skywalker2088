use crate::components::spawnable::ItemType;
use bevy::{
    asset::Handle,
    prelude::{Image, Res, Resource},
    sprite::TextureAtlasLayout,
};
use bevy_asset_loader::asset_collection::AssetCollection;

/// Collection of texture atlases and images for item sprites
#[derive(AssetCollection, Resource)]
pub struct ItemAssets {
    #[asset(key = "item_placeholder.layout")]
    pub item_placeholder_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "item_placeholder.image")]
    pub item_placeholder_image: Handle<Image>,
}

impl ItemAssets {
    /// Use a ItemType enum to access a texture atlas layout
    pub fn get_texture_atlas_layout(&self, item_type: &ItemType) -> Handle<TextureAtlasLayout> {
        match item_type {
            ItemType::EnhancedPlating => self.item_placeholder_layout.clone(),
            /*
            ItemType::SteelBarrel => self.item_placeholder.clone(),
            ItemType::PlasmaBlasts => self.item_placeholder.clone(),
            ItemType::HazardousReactor => self.item_placeholder.clone(),
            ItemType::WarpThruster => self.item_placeholder.clone(),
            ItemType::Tentaclover => self.item_placeholder.clone(),
            ItemType::DefenseSatellite => self.item_placeholder.clone(),
            ItemType::DoubleBarrel => self.item_placeholder.clone(),
            ItemType::YithianPlague => self.item_placeholder.clone(),
            ItemType::Spice => self.item_placeholder.clone(),
            ItemType::StructureReinforcement => self.item_placeholder.clone(),
            ItemType::BlasterSizeEnhancer => self.item_placeholder.clone(),
            ItemType::FrequencyAugmentor => self.item_placeholder.clone(),
            ItemType::TractorBeam => self.item_placeholder.clone(),
            ItemType::BlastRepeller => self.item_placeholder.clone(),
            */
        }
    }

    /// Use a ItemType enum to access an item image handle
    pub fn get_image(&self, item_type: &ItemType) -> Handle<Image> {
        match item_type {
            ItemType::EnhancedPlating => self.item_placeholder_image.clone(),
        }
    }
}
