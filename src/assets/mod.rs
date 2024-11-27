pub mod game_assets;
pub mod player_assets;
use bevy::prelude::*;
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((game_assets::plugin,));
}
