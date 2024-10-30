// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use skywalker2077::AppPlugin;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}
