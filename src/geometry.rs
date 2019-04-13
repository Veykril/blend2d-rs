use ffi::BLGeometryDirection::*;
bl_enum! {
    pub enum GeometryDirection {
        None = BL_GEOMETRY_DIRECTION_NONE,
        Clockwise = BL_GEOMETRY_DIRECTION_CW,
        CounterClockwise = BL_GEOMETRY_DIRECTION_CCW,
    }
    Default => None
}

use ffi::BLGeometryType::*;
bl_enum! {
    enum GeometryType {
        None = BL_GEOMETRY_TYPE_NONE,
        BoxI = BL_GEOMETRY_TYPE_BOXI,
        BoxD= BL_GEOMETRY_TYPE_BOXD,
        RectI = BL_GEOMETRY_TYPE_RECTI,
        RectT = BL_GEOMETRY_TYPE_RECTD,
        Circle = BL_GEOMETRY_TYPE_CIRCLE,
        Ellipse = BL_GEOMETRY_TYPE_ELLIPSE,
        RoundRect = BL_GEOMETRY_TYPE_ROUND_RECT,
        Arc = BL_GEOMETRY_TYPE_ARC,
        Chord = BL_GEOMETRY_TYPE_CHORD,
        Pie = BL_GEOMETRY_TYPE_PIE,
        Line = BL_GEOMETRY_TYPE_LINE,
        Triangle = BL_GEOMETRY_TYPE_TRIANGLE,
        PolyLineI = BL_GEOMETRY_TYPE_POLYLINEI,
        PolyLineD = BL_GEOMETRY_TYPE_POLYLINED,
        PolygonI = BL_GEOMETRY_TYPE_POLYGONI,
        PolygonD = BL_GEOMETRY_TYPE_POLYGOND,
        ArrayViewBoxI = BL_GEOMETRY_TYPE_ARRAY_VIEW_BOXI,
        ArrayViewBoxD = BL_GEOMETRY_TYPE_ARRAY_VIEW_BOXD,
        ArrayViewRectI  = BL_GEOMETRY_TYPE_ARRAY_VIEW_RECTI,
        ArrayViewRectD = BL_GEOMETRY_TYPE_ARRAY_VIEW_RECTD,
        Path = BL_GEOMETRY_TYPE_PATH,
        Region = BL_GEOMETRY_TYPE_REGION,
    }
    Default => None
}

use ffi::BLFillRule::*;
bl_enum! {
    pub enum FillRule {
        NonZero = BL_FILL_RULE_NON_ZERO,
        EvenOdd = BL_FILL_RULE_EVEN_ODD,
    }
    Default => NonZero
}

use ffi::BLHitTest::*;
bl_enum! {
    pub enum HitTest {
        In = BL_HIT_TEST_IN,
        Part = BL_HIT_TEST_PART,
        Out = BL_HIT_TEST_OUT,
    }
    Default => In
}

#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq)]
pub struct PointI {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq)]
pub struct SizeI {
    pub w: i32,
    pub h: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq)]
pub struct Size {
    pub w: f64,
    pub h: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq)]
pub struct BoxI {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq)]
pub struct Box {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq)]
pub struct RectI {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq)]
pub struct Line {
    pub p0: Point,
    pub p1: Point,
}

#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq)]
pub struct Triangle {
    pub p0: Point,
    pub p1: Point,
    pub p2: Point,
}

#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq)]
pub struct BoundRect {
    pub rect: Rect,
    pub radius: Point,
}

#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq)]
pub struct Ellipse {
    pub center: Point,
    pub radius: Point,
}

#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq)]
pub struct Arc {
    pub center: Point,
    pub radius: Point,
    pub start: f64,
    pub sweep: f64,
}
