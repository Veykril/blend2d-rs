mod private {
    use super::*;
    use crate::array::ArrayType;

    pub trait Sealed {}
    impl Sealed for Arc {}
    impl Sealed for BoxD {}
    impl Sealed for BoxI {}
    impl Sealed for Chord {}
    impl Sealed for Circle {}
    impl Sealed for Ellipse {}
    impl Sealed for Line {}
    impl Sealed for Pie {}
    impl Sealed for PointD {}
    impl Sealed for PointI {}
    impl Sealed for RectD {}
    impl Sealed for RectI {}
    impl Sealed for RoundRect {}
    impl Sealed for Triangle {}
    impl Sealed for crate::path::Path {}
    impl Sealed for crate::region::Region {}
    impl<P: Sealed> Sealed for [P] {}
    impl<T: ArrayType> Sealed for Array<T> where [T]: Sealed {}
}

pub trait Geometry: private::Sealed {
    #[doc(hidden)]
    const GEO_TYPE: u32;
}

impl Geometry for crate::path::Path {
    const GEO_TYPE: u32 = GeometryType::Path as u32;
}

impl Geometry for crate::region::Region {
    const GEO_TYPE: u32 = GeometryType::Region as u32;
}

// TODO: Should [Point] really implement Geometry? Given that it could be either
// a Polygon or a Polyline it might be better to make it explicit.
impl Geometry for [PointI] {
    const GEO_TYPE: u32 = GeometryType::PolygonI as u32;
}
impl Geometry for [PointD] {
    const GEO_TYPE: u32 = GeometryType::PolygonD as u32;
}
impl Geometry for [BoxD] {
    const GEO_TYPE: u32 = GeometryType::ArrayViewBoxD as u32;
}
impl Geometry for [BoxI] {
    const GEO_TYPE: u32 = GeometryType::ArrayViewBoxI as u32;
}
impl Geometry for [RectI] {
    const GEO_TYPE: u32 = GeometryType::ArrayViewRectI as u32;
}
impl Geometry for [RectD] {
    const GEO_TYPE: u32 = GeometryType::ArrayViewRectD as u32;
}

use crate::array::{Array, ArrayType};
impl<T> Geometry for Array<T>
where
    [T]: Geometry,
    T: ArrayType,
{
    const GEO_TYPE: u32 = <[T]>::GEO_TYPE;
}

// trait for overloading
pub trait GeoViewArray: private::Sealed {}
impl GeoViewArray for BoxD {}
impl GeoViewArray for BoxI {}
impl GeoViewArray for RectD {}
impl GeoViewArray for RectI {}

type BlitImageFn<T> = unsafe extern "C" fn(
    *mut ffi::BLContextCore,
    *const T,
    *const ffi::BLImageCore,
    *const ffi::BLRectI,
) -> ffi::BLResult;
type FillTextFn<T> = unsafe extern "C" fn(
    *mut ffi::BLContextCore,
    *const T,
    *const ffi::BLFontCore,
    *const std::ffi::c_void,
    usize,
    u32,
) -> ffi::BLResult;
// trait for overloading
pub trait Point: private::Sealed + Copy {
    #[doc(hidden)]
    type FfiType;
    #[doc(hidden)]
    const POLYLINE_TYPE: u32;
    #[doc(hidden)]
    fn into_f64(self) -> [f64; 2];
    #[doc(hidden)]
    const BLIT_IMAGE: BlitImageFn<Self::FfiType>;
    #[doc(hidden)]
    const FILL_TEXT: FillTextFn<Self::FfiType>;
}
impl Point for PointI {
    #[doc(hidden)]
    type FfiType = ffi::BLPointI;
    #[doc(hidden)]
    const POLYLINE_TYPE: u32 = GeometryType::PolyLineI as u32;
    #[doc(hidden)]
    fn into_f64(self) -> [f64; 2] {
        [self.x as f64, self.y as f64]
    }
    #[doc(hidden)]
    const BLIT_IMAGE: BlitImageFn<Self::FfiType> = ffi::blContextBlitImageI;
    #[doc(hidden)]
    const FILL_TEXT: FillTextFn<Self::FfiType> = ffi::blContextFillTextI;
}
impl Point for PointD {
    #[doc(hidden)]
    type FfiType = ffi::BLPoint;
    #[doc(hidden)]
    const POLYLINE_TYPE: u32 = GeometryType::PolyLineD as u32;
    #[doc(hidden)]
    fn into_f64(self) -> [f64; 2] {
        [self.x, self.y]
    }
    #[doc(hidden)]
    const BLIT_IMAGE: BlitImageFn<Self::FfiType> = ffi::blContextBlitImageD;
    #[doc(hidden)]
    const FILL_TEXT: FillTextFn<Self::FfiType> = ffi::blContextFillTextD;
}

type BlitScaledImageFn<T> = unsafe extern "C" fn(
    *mut ffi::BLContextCore,
    *const T,
    *const ffi::BLImageCore,
    *const ffi::BLRectI,
) -> ffi::BLResult;
type ClipToRectFn<T> = unsafe extern "C" fn(*mut ffi::BLContextCore, *const T) -> ffi::BLResult;
type ClearRectFn<T> = unsafe extern "C" fn(*mut ffi::BLContextCore, *const T) -> ffi::BLResult;
// trait for overloading
pub trait Rect: private::Sealed {
    #[doc(hidden)]
    type FfiType;
    #[doc(hidden)]
    const CLIP_TO_RECT: ClipToRectFn<Self::FfiType>;
    #[doc(hidden)]
    const CLEAR_RECT: ClearRectFn<Self::FfiType>;
    #[doc(hidden)]
    const BLIT_SCALED_IMAGE: BlitImageFn<Self::FfiType>;
}

impl Rect for RectI {
    #[doc(hidden)]
    type FfiType = ffi::BLRectI;
    #[doc(hidden)]
    const CLIP_TO_RECT: ClipToRectFn<Self::FfiType> = ffi::blContextClipToRectI;
    #[doc(hidden)]
    const CLEAR_RECT: ClearRectFn<Self::FfiType> = ffi::blContextClearRectI;
    #[doc(hidden)]
    const BLIT_SCALED_IMAGE: BlitScaledImageFn<Self::FfiType> = ffi::blContextBlitScaledImageI;
}

impl Rect for RectD {
    #[doc(hidden)]
    type FfiType = ffi::BLRect;
    #[doc(hidden)]
    const CLIP_TO_RECT: ClipToRectFn<Self::FfiType> = ffi::blContextClipToRectD;
    #[doc(hidden)]
    const CLEAR_RECT: ClearRectFn<Self::FfiType> = ffi::blContextClearRectD;
    #[doc(hidden)]
    const BLIT_SCALED_IMAGE: BlitScaledImageFn<Self::FfiType> = ffi::blContextBlitScaledImageD;
}

use ffi::BLGeometryDirection::*;
bl_enum! {
    pub enum GeometryDirection {
        None             = BL_GEOMETRY_DIRECTION_NONE,
        Clockwise        = BL_GEOMETRY_DIRECTION_CW,
        CounterClockwise = BL_GEOMETRY_DIRECTION_CCW,
    }
    Default => None
}

use ffi::BLGeometryType::*;
bl_enum! {
    pub(in crate) enum GeometryType {
        None           = BL_GEOMETRY_TYPE_NONE,
        BoxI           = BL_GEOMETRY_TYPE_BOXI,
        BoxD           = BL_GEOMETRY_TYPE_BOXD,
        RectI          = BL_GEOMETRY_TYPE_RECTI,
        RectD          = BL_GEOMETRY_TYPE_RECTD,
        Circle         = BL_GEOMETRY_TYPE_CIRCLE,
        Ellipse        = BL_GEOMETRY_TYPE_ELLIPSE,
        RoundRect      = BL_GEOMETRY_TYPE_ROUND_RECT,
        Arc            = BL_GEOMETRY_TYPE_ARC,
        Chord          = BL_GEOMETRY_TYPE_CHORD,
        Pie            = BL_GEOMETRY_TYPE_PIE,
        Line           = BL_GEOMETRY_TYPE_LINE,
        Triangle       = BL_GEOMETRY_TYPE_TRIANGLE,
        PolyLineI      = BL_GEOMETRY_TYPE_POLYLINEI,
        PolyLineD      = BL_GEOMETRY_TYPE_POLYLINED,
        PolygonI       = BL_GEOMETRY_TYPE_POLYGONI,
        PolygonD       = BL_GEOMETRY_TYPE_POLYGOND,
        ArrayViewBoxI  = BL_GEOMETRY_TYPE_ARRAY_VIEW_BOXI,
        ArrayViewBoxD  = BL_GEOMETRY_TYPE_ARRAY_VIEW_BOXD,
        ArrayViewRectI = BL_GEOMETRY_TYPE_ARRAY_VIEW_RECTI,
        ArrayViewRectD = BL_GEOMETRY_TYPE_ARRAY_VIEW_RECTD,
        Path           = BL_GEOMETRY_TYPE_PATH,
        Region         = BL_GEOMETRY_TYPE_REGION,
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
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct PointI {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct PointD {
    pub x: f64,
    pub y: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct SizeI {
    pub w: i32,
    pub h: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct SizeD {
    pub w: f64,
    pub h: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct BoxI {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
}

impl Geometry for BoxI {
    const GEO_TYPE: u32 = GeometryType::BoxI as u32;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct BoxD {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

impl Geometry for BoxD {
    const GEO_TYPE: u32 = GeometryType::BoxD as u32;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct RectI {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Geometry for RectI {
    const GEO_TYPE: u32 = GeometryType::RectI as u32;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct RectD {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Geometry for RectD {
    const GEO_TYPE: u32 = GeometryType::RectD as u32;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Line {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

impl Geometry for Line {
    const GEO_TYPE: u32 = GeometryType::Line as u32;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Triangle {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

impl Geometry for Triangle {
    const GEO_TYPE: u32 = GeometryType::Triangle as u32;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct RoundRect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub rx: f64,
    pub ry: f64,
}

impl Geometry for RoundRect {
    const GEO_TYPE: u32 = GeometryType::RoundRect as u32;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Circle {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
}

impl Geometry for Circle {
    const GEO_TYPE: u32 = GeometryType::Circle as u32;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Ellipse {
    pub cx: f64,
    pub cy: f64,
    pub rx: f64,
    pub ry: f64,
}

impl Geometry for Ellipse {
    const GEO_TYPE: u32 = GeometryType::Ellipse as u32;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Arc {
    pub cx: f64,
    pub cy: f64,
    pub rx: f64,
    pub ry: f64,
    pub start: f64,
    pub sweep: f64,
}

impl Geometry for Arc {
    const GEO_TYPE: u32 = GeometryType::Arc as u32;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Chord {
    pub cx: f64,
    pub cy: f64,
    pub rx: f64,
    pub ry: f64,
    pub start: f64,
    pub sweep: f64,
}

impl Geometry for Chord {
    const GEO_TYPE: u32 = GeometryType::Chord as u32;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Pie {
    pub cx: f64,
    pub cy: f64,
    pub rx: f64,
    pub ry: f64,
    pub start: f64,
    pub sweep: f64,
}

impl Geometry for Pie {
    const GEO_TYPE: u32 = GeometryType::Pie as u32;
}
