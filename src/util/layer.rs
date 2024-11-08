/// Warning: Cannot go above 998 as the Camera defaults to z = 999
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum RenderLayer {
    Background = 100,
    Bullet = 300,
    Player = 500,
    Enemy = 700,
    Effects = 900,
}

impl RenderLayer {
    /// Gets named z position with offset to stay in allocated range.
    /// The offset must be from -100 to 100 (exclusive) or the app will panic.
    pub fn as_z_with_offset(self, offset: f32) -> f32 {
        if offset >= 100.0 || offset <= -100.0 {
            panic!("Layer offset can not be 100 or more");
        }
        let base = self as u16 as f32;
        return (base + offset) as f32;
    }

    pub const fn as_z(self) -> f32 {
        self as u16 as f32
    }
}
