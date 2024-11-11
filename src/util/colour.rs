use bevy::prelude::Color;

pub struct Colour;

#[allow(dead_code)]
impl Colour {
    pub const BLACK: Color = Color::srgb(31.0 / 255.0, 26.0 / 255.0, 36.0 / 255.0);
    pub const PLAYER: Color = Color::srgb(168.0 / 255.0, 207.0 / 255.0, 218.0 / 255.0);
    pub const WHITE: Color = Color::srgb(238.0 / 255.0, 236.0 / 255.0, 222.0 / 255.0);
    pub const SHIELD: Color = Color::srgb(120.0 / 255.0, 149.0 / 255.0, 171.0 / 255.0);
    pub const INACTIVE: Color = Color::srgb(119.0 / 255.0, 117.0 / 255.0, 103.0 / 255.0);
    pub const ENEMY: Color = Color::srgb(172.0 / 255.0, 138.0 / 255.0, 113.0 / 255.0);
    pub const RED: Color = Color::srgb(1.0, 138.0 / 255.0, 113.0 / 255.0);
    pub const GREEN: Color = Color::srgb(130.0 / 255.0, 170.0 / 255.0, 119.0 / 255.0);
    pub const YELLOW: Color = Color::srgb(237.0 / 255.0, 225.0 / 255.0, 158.0 / 255.0);
    pub const PURPLE: Color = Color::srgb(138.0 / 255.0, 112.0 / 255.0, 225.0 / 255.0);
    pub const PINK: Color = Color::srgb(1.0, 113.0 / 255.0, 159.0 / 255.0);
}
