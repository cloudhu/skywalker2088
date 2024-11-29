//! The game's main screen states and transitions between them.

mod credits;
mod gameplay;
mod loading;
mod main_menu;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        credits::plugin,
        gameplay::plugin,
        loading::plugin,
        main_menu::plugin,
    ));
}
