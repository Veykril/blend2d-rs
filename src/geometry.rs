pub mod path;
pub use self::path::*;

use ffi::BLFillRule::*;
bl_enum! {
    pub enum FillRule {
        NonZero = BL_FILL_RULE_NON_ZERO,
        EvenOdd = BL_FILL_RULE_EVEN_ODD,
    }
    Default => NonZero
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Box2D {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl From<ffi::BLPoint> for Point2D {
    fn from(p: ffi::BLPoint) -> Self {
        Point2D { x: p.x, y: p.y }
    }
}

impl Into<ffi::BLPoint> for Point2D {
    fn into(self) -> ffi::BLPoint {
        ffi::BLPoint {
            x: self.x,
            y: self.y,
        }
    }
}
