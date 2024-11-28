pub mod audio;
pub mod consumable;
pub mod effect;
pub mod game_assets;
pub mod item;
pub mod mob;
pub mod player_assets;
pub mod projectile;
pub mod ui;

use bevy::prelude::*;
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((game_assets::plugin,));
}
