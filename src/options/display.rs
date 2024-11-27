//! Systems to configure minor display settings.
use bevy::{
    prelude::*,
    window::{PrimaryWindow, Window, WindowMode},
    winit::WinitWindows,
};
use serde::Deserialize;
use winit::window::Icon;

/// Display settings of the window
#[derive(Deserialize)]
pub struct DisplayConfig {
    /// Width of window
    pub width: f32,
    /// Height of window
    pub height: f32,
    /// True of fullsceen, false if windowed
    pub fullscreen: bool,
}

impl From<DisplayConfig> for Window {
    fn from(display_config: DisplayConfig) -> Self {
        Window {
            title: "Thetawave".to_string(),
            resolution: (display_config.width, display_config.height).into(),
            resizable: true,
            mode: if display_config.fullscreen {
                WindowMode::SizedFullscreen
            } else {
                WindowMode::Windowed
            },
            ..Default::default()
        }
    }
}

/// set the window icon. This needs to be run once, near app start up.
pub(super) fn set_window_icon(
    windows: NonSend<WinitWindows>,
    window_query: Query<Entity, With<PrimaryWindow>>,
) {
    let window = windows
        .get_window(window_query.get_single().unwrap())
        .unwrap();

    // set icon using image crate
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/texture/window_icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    window.set_window_icon(Some(icon));
}
