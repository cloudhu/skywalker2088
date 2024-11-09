use crate::asset_tracking::LoadResource;
use crate::components::common::Health;
use crate::gameplay::effects::HitFlash;
use crate::gameplay::gamelogic::{DespawnWithScene, ExplodesOnDespawn, Targettable, WillTarget};
use crate::gameplay::physics::{Collider, Physics};
use crate::ship::engine::Engine;
use bevy::app::App;
use bevy::asset::{Asset, AssetServer, Handle};
use bevy::audio::AudioSource;
use bevy::prelude::{Bundle, Font, FromWorld, Reflect, Resource, Text2dBundle, World};

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
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
}

impl Fonts {
    pub const PATH_PRIMARY: &'static str = "fonts/AnonymousPro-Regular.ttf";
    pub const PATH_UNICODE: &'static str = "fonts/DejaVuLGCSansMono.ttf";
    pub const PATH_STEP_1: &'static str = "audio/sound_effects/step1.ogg";
    pub const PATH_STEP_2: &'static str = "audio/sound_effects/step2.ogg";
    pub const PATH_STEP_3: &'static str = "audio/sound_effects/step3.ogg";
    pub const PATH_STEP_4: &'static str = "audio/sound_effects/step4.ogg";
}

impl FromWorld for Fonts {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            primary: assets.load(Fonts::PATH_PRIMARY),
            unicode: assets.load(Fonts::PATH_UNICODE),
            steps: vec![
                assets.load(Fonts::PATH_STEP_1),
                assets.load(Fonts::PATH_STEP_2),
                assets.load(Fonts::PATH_STEP_3),
                assets.load(Fonts::PATH_STEP_4),
            ],
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<Fonts>();
}
