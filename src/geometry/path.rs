#![allow(clippy::too_many_arguments)]
use bitflags::bitflags;

use core::{
    mem,
    ops::{self, Range},
    ptr, slice,
};

use crate::{
    bl_range,
    error::{errcode_to_result, Result},
    geometry::{Box, FillRule, Point, Rect},
    ImplType, Matrix2D,
};

use ffi::BLPathCmd::*;
bl_enum! {
    pub enum PathCommand {
        Move = BL_PATH_CMD_MOVE,
        On = BL_PATH_CMD_ON,
        Quad = BL_PATH_CMD_QUAD,
        Cubic = BL_PATH_CMD_CUBIC,
        Close = BL_PATH_CMD_CLOSE,
    }
    Default => Move
}

use ffi::BLPathFlags::*;
bitflags! {
    pub struct PathFlags: u32 {
        const EMPTY = BL_PATH_FLAG_EMPTY as u32;
        const MULTIPLE = BL_PATH_FLAG_MULTIPLE as u32;
        const QUADS = BL_PATH_FLAG_QUADS as u32;
        const CUBICS = BL_PATH_FLAG_CUBICS as u32;
        const INVALID = BL_PATH_FLAG_INVALID as u32;
        const DIRTY = BL_PATH_FLAG_DIRTY as u32;
    }
}

bitflags! {
    pub struct PathFitFlags: u32 {
        #[doc(hidden)]
        const PLACEHOLDER = 0;
    }
}

use ffi::BLPathReverseMode::*;
bl_enum! {
    pub enum PathReverseMode {
        Complete = BL_PATH_REVERSE_MODE_COMPLETE,
        Separate = BL_PATH_REVERSE_MODE_SEPARATE,
    }
    Default => Complete
}

use ffi::BLStrokeJoin::*;
bl_enum! {
    pub enum StrokeJoin {
        MiterClip = BL_STROKE_JOIN_MITER_CLIP,
        MiterBevel = BL_STROKE_JOIN_MITER_BEVEL,
        MiterRound = BL_STROKE_JOIN_MITER_ROUND,
        Bevel = BL_STROKE_JOIN_BEVEL,
        Round = BL_STROKE_JOIN_ROUND,
    }
    Default => MiterClip
}

use ffi::BLStrokeCap::*;
bl_enum! {
    pub enum StrokeCap {
        Butt = BL_STROKE_CAP_BUTT,
        Square = BL_STROKE_CAP_SQUARE,
        Round = BL_STROKE_CAP_ROUND,
        RoundRev = BL_STROKE_CAP_ROUND_REV,
        Triangle = BL_STROKE_CAP_TRIANGLE,
        TriangleRev = BL_STROKE_CAP_TRIANGLE_REV,
    }
    Default => Butt
}

use ffi::BLStrokeTransformOrder::*;
bl_enum! {
    pub enum StrokeTransformOrder {
        After = BL_STROKE_TRANSFORM_ORDER_AFTER,
        Before = BL_STROKE_TRANSFORM_ORDER_BEFORE,
    }
    Default => After
}

use ffi::BLFlattenMode::*;
bl_enum! {
    pub enum FlattenMode {
        Default = BL_FLATTEN_MODE_DEFAULT,
        Recursive = BL_FLATTEN_MODE_RECURSIVE,
    }
    Default => Default
}

use ffi::BLOffsetMode::*;
bl_enum! {
    pub enum OffsetMode {
        Default = BL_OFFSET_MODE_DEFAULT,
        Iterative = BL_OFFSET_MODE_ITERATIVE,
    }
    Default => Default
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ApproximationOptions {
    flatten_mode: u8,
    offset_mode: u8,
    _reserved_flags: [u8; 6],
    pub flatten_tolerance: f64,
    pub simplyify_tolerance: f64,
    pub offset_parameter: f64,
}

impl ApproximationOptions {
    pub fn set_flatten_mode(&mut self, mode: FlattenMode) {
        self.flatten_mode = mode as u8;
    }

    pub fn flatten_mode(&self) -> FlattenMode {
        (self.flatten_mode as u32).into()
    }

    pub fn set_offset_mode(&mut self, mode: OffsetMode) {
        self.offset_mode = mode as u8;
    }

    pub fn offset_mode(&self) -> OffsetMode {
        (self.offset_mode as u32).into()
    }
}

impl Default for ApproximationOptions {
    fn default() -> Self {
        unsafe { *(&ffi::blDefaultApproximationOptions as *const _ as *const ApproximationOptions) }
    }
}

pub struct StrokeOptions {
    pub(in crate) core: ffi::BLStrokeOptionsCore,
}

impl StrokeOptions {
    #[inline]
    pub fn new() -> Self {
        unsafe {
            let mut core = mem::zeroed();
            ffi::blStrokeOptionsInit(&mut core);
            StrokeOptions { core }
        }
    }

    pub fn set_caps(&mut self, cap: StrokeCap) {
        unsafe {
            self.core.__bindgen_anon_1.__bindgen_anon_1.startCap = cap as u8;
            self.core.__bindgen_anon_1.__bindgen_anon_1.endCap = cap as u8;
        }
    }
}

impl Default for StrokeOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for StrokeOptions {
    fn drop(&mut self) {
        unsafe { ffi::blStrokeOptionsReset(&mut self.core) };
    }
}

#[repr(transparent)]
pub struct Path {
    pub(in crate) core: ffi::BLPathCore,
}

impl Path {
    #[inline]
    pub fn new() -> Self {
        Path {
            core: ffi::BLPathCore {
                impl_: Self::none().impl_,
            },
        }
    }

    pub fn with_capacity(n: usize) -> Self {
        let mut this = Self::new();
        this.reserve(n);
        this
    }

    #[inline]
    pub fn clear(&mut self) {
        unsafe { ffi::blPathClear(&mut self.core) };
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        unsafe { ffi::blPathShrink(&mut self.core) };
    }

    #[inline]
    pub fn reserve(&mut self, n: usize) {
        self.try_reserve(n).unwrap();
    }

    pub fn try_reserve(&mut self, n: usize) -> Result<()> {
        unsafe { errcode_to_result(ffi::blPathReserve(&mut self.core, n)) }
    }

    #[inline]
    pub fn reset(&mut self) {
        unsafe { ffi::blPathReset(&mut self.core) };
    }

    #[inline]
    pub fn len(&self) -> usize {
        unsafe { self.impl_().__bindgen_anon_1.view.size }
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.impl_().capacity as usize
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.impl_().__bindgen_anon_1.view.commandData, self.len()) }
    }

    #[inline]
    fn impl_(&self) -> &ffi::BLPathImpl {
        unsafe { &*self.core.impl_ }
    }

    pub fn info_flags(&self) -> Result<PathFlags> {
        unsafe {
            let mut flags = 0;
            errcode_to_result(ffi::blPathGetInfoFlags(&self.core, &mut flags))
                .map(|_| PathFlags::from_bits_truncate(flags))
        }
    }

    pub fn control_box(&self) -> Result<Box> {
        unsafe {
            let mut box2d = Box::default();
            errcode_to_result(ffi::blPathGetControlBox(
                &self.core,
                &mut box2d as *mut _ as *mut _,
            ))
            .map(|_| box2d)
        }
    }

    pub fn bounding_box(&self) -> Result<Box> {
        unsafe {
            let mut box2d = Box::default();
            errcode_to_result(ffi::blPathGetBoundingBox(
                &self.core,
                &mut box2d as *mut _ as *mut _,
            ))
            .map(|_| box2d)
        }
    }

    pub fn figure_range(&self, index: usize) -> Result<Range<usize>> {
        unsafe {
            let mut range = ffi::BLRange { start: 0, end: 0 };
            errcode_to_result(ffi::blPathGetFigureRange(&self.core, index, &mut range)).map(|_| {
                Range {
                    start: range.start,
                    end: range.end,
                }
            })
        }
    }

    pub fn last_vertex(&self) -> Result<Point> {
        unsafe {
            let mut point = Point::default();
            errcode_to_result(ffi::blPathGetLastVertex(
                &self.core,
                &mut point as *mut _ as *mut _,
            ))
            .map(|_| point)
        }
    }

    pub fn closest_vertex(&self, p: Point, max_distance: f64) -> Result<(usize, f64)> {
        unsafe {
            let mut idx = 0;
            let mut dout = 0.0;
            errcode_to_result(ffi::blPathGetClosestVertex(
                &self.core,
                &p as *const _ as *const _,
                max_distance,
                &mut idx,
                &mut dout,
            ))
            .map(|_| (idx, dout))
        }
    }

    pub fn hit_test(&self, p: Point, fill_rule: FillRule) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathHitTest(
                &self.core,
                &p as *const _ as *const _,
                fill_rule as u32,
            ))
        }
    }
}

impl Path {
    #[inline]
    pub fn set_vertex_at(&mut self, index: usize, cmd: PathCommand, x: f64, y: f64) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathSetVertexAt(
                &mut self.core,
                index,
                cmd as u32,
                x,
                y,
            ))
        }
    }

    #[inline]
    pub fn set_vertex_at_point(
        &mut self,
        index: usize,
        cmd: PathCommand,
        point: Point,
    ) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathSetVertexAt(
                &mut self.core,
                index,
                cmd as u32,
                point.x,
                point.y,
            ))
        }
    }

    #[inline]
    pub fn move_to(&mut self, x: f64, y: f64) -> Result<()> {
        unsafe { errcode_to_result(ffi::blPathMoveTo(&mut self.core, x, y)) }
    }

    #[inline]
    pub fn move_to_point(&mut self, point: Point) -> Result<()> {
        unsafe { errcode_to_result(ffi::blPathMoveTo(&mut self.core, point.x, point.y)) }
    }

    #[inline]
    pub fn line_to(&mut self, x: f64, y: f64) -> Result<()> {
        unsafe { errcode_to_result(ffi::blPathLineTo(&mut self.core, x, y)) }
    }

    #[inline]
    pub fn line_to_point(&mut self, point: Point) -> Result<()> {
        unsafe { errcode_to_result(ffi::blPathLineTo(&mut self.core, point.x, point.y)) }
    }

    #[inline]
    pub fn poly_to(&mut self, poly: &[Point]) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathPolyTo(
                &mut self.core,
                poly.as_ptr() as *const _,
                poly.len(),
            ))
        }
    }

    #[inline]
    pub fn quad_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) -> Result<()> {
        unsafe { errcode_to_result(ffi::blPathQuadTo(&mut self.core, x1, y1, x2, y2)) }
    }

    #[inline]
    pub fn quad_to_points(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) -> Result<()> {
        unsafe { errcode_to_result(ffi::blPathQuadTo(&mut self.core, x1, y1, x2, y2)) }
    }

    #[inline]
    pub fn cubic_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> Result<()> {
        unsafe { errcode_to_result(ffi::blPathCubicTo(&mut self.core, x1, y1, x2, y2, x3, y3)) }
    }

    #[inline]
    pub fn cubic_to_points(&mut self, p1: Point, p2: Point, p3: Point) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathCubicTo(
                &mut self.core,
                p1.x,
                p1.y,
                p2.x,
                p2.y,
                p3.x,
                p3.y,
            ))
        }
    }

    #[inline]
    pub fn smooth_quad_to(&mut self, x2: f64, y2: f64) -> Result<()> {
        unsafe { errcode_to_result(ffi::blPathSmoothQuadTo(&mut self.core, x2, y2)) }
    }

    #[inline]
    pub fn smooth_quad_to_point(&mut self, p2: Point) -> Result<()> {
        unsafe { errcode_to_result(ffi::blPathSmoothQuadTo(&mut self.core, p2.x, p2.y)) }
    }

    #[inline]
    pub fn smooth_cubic_to(&mut self, x2: f64, y2: f64, x3: f64, y3: f64) -> Result<()> {
        unsafe { errcode_to_result(ffi::blPathSmoothCubicTo(&mut self.core, x2, y2, x3, y3)) }
    }

    #[inline]
    pub fn smooth_cubic_to_points(&mut self, p2: Point, p3: Point) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathSmoothCubicTo(
                &mut self.core,
                p2.x,
                p2.y,
                p3.x,
                p3.y,
            ))
        }
    }

    //FIXME change bool to a 2-variant enum
    #[inline]
    pub fn arc_to(
        &mut self,
        cx: f64,
        cy: f64,
        rx: f64,
        ry: f64,
        start: f64,
        sweep: f64,
        force_move_to: bool,
    ) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathArcTo(
                &mut self.core,
                cx,
                cy,
                rx,
                ry,
                start,
                sweep,
                force_move_to,
            ))
        }
    }

    //FIXME change bool to a 2-variant enum
    #[inline]
    pub fn arc_to_points(
        &mut self,
        cp: Point,
        rp: Point,
        start: f64,
        sweep: f64,
        force_move_to: bool,
    ) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathArcTo(
                &mut self.core,
                cp.x,
                cp.y,
                rp.x,
                rp.y,
                start,
                sweep,
                force_move_to,
            ))
        }
    }

    #[inline]
    pub fn arc_quadrant_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) -> Result<()> {
        unsafe { errcode_to_result(ffi::blPathArcQuadrantTo(&mut self.core, x1, y1, x2, y2)) }
    }

    #[inline]
    pub fn arc_quadrant_to_points(&mut self, p1: Point, p2: Point) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathArcQuadrantTo(
                &mut self.core,
                p1.x,
                p1.y,
                p2.x,
                p2.y,
            ))
        }
    }

    //FIXME change bools to 2-variant enums
    #[inline]
    pub fn elliptic_arc_to(
        &mut self,
        rx: f64,
        ry: f64,
        x_axis_rotation: f64,
        large_arc_flag: bool,
        sweep_flag: bool,
        x1: f64,
        y1: f64,
    ) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathEllipticArcTo(
                &mut self.core,
                rx,
                ry,
                x_axis_rotation,
                large_arc_flag,
                sweep_flag,
                x1,
                y1,
            ))
        }
    }
    //FIXME change bools to 2-variant enums
    #[inline]
    pub fn elliptic_arc_to_points(
        &mut self,
        rp: Point,
        x_axis_rotation: f64,
        large_arc_flag: bool,
        sweep_flag: bool,
        p1: Point,
    ) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathEllipticArcTo(
                &mut self.core,
                rp.x,
                rp.y,
                x_axis_rotation,
                large_arc_flag,
                sweep_flag,
                p1.x,
                p1.y,
            ))
        }
    }

    #[inline]
    pub fn close(&mut self) -> Result<()> {
        unsafe { errcode_to_result(ffi::blPathClose(&mut self.core)) }
    }
}

impl Path {
    #[inline]
    pub fn add_path(&mut self, other: &Path) -> Result<()> {
        unsafe { errcode_to_result(ffi::blPathAddPath(&mut self.core, &other.core, ptr::null())) }
    }

    pub fn add_path_range<R: ops::RangeBounds<usize>>(
        &mut self,
        other: &Path,
        range: R,
    ) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathAddPath(
                &mut self.core,
                &other.core,
                &bl_range(range),
            ))
        }
    }

    #[inline]
    pub fn add_translated_path(&mut self, other: &Path, p: &Point) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathAddTranslatedPath(
                &mut self.core,
                &other.core,
                ptr::null(),
                &p as *const _ as *const _,
            ))
        }
    }

    pub fn add_translated_path_range<R: ops::RangeBounds<usize>>(
        &mut self,
        other: &Path,
        range: R,
        p: &Point,
    ) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathAddTranslatedPath(
                &mut self.core,
                &other.core,
                &bl_range(range),
                &p as *const _ as *const _,
            ))
        }
    }

    #[inline]
    pub fn add_transformed_path(&mut self, other: &Path, m: &Matrix2D) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathAddTransformedPath(
                &mut self.core,
                &other.core,
                ptr::null(),
                &m as *const _ as *const _,
            ))
        }
    }

    pub fn add_transformed_path_range<R: ops::RangeBounds<usize>>(
        &mut self,
        other: &Path,
        range: R,
        m: &Matrix2D,
    ) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathAddTransformedPath(
                &mut self.core,
                &other.core,
                &bl_range(range),
                &m as *const _ as *const _,
            ))
        }
    }

    #[inline]
    pub fn add_reversed_path(&mut self, other: &Path, mode: PathReverseMode) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathAddReversedPath(
                &mut self.core,
                &other.core,
                ptr::null(),
                mode.into(),
            ))
        }
    }

    pub fn add_reversed_path_range<R: ops::RangeBounds<usize>>(
        &mut self,
        other: &Path,
        range: R,
        mode: PathReverseMode,
    ) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathAddReversedPath(
                &mut self.core,
                &other.core,
                &bl_range(range),
                mode.into(),
            ))
        }
    }

    #[inline]
    pub fn add_stroked_path(
        &mut self,
        other: &Path,
        options: &StrokeOptions,
        approx: &ApproximationOptions,
    ) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathAddStrokedPath(
                &mut self.core,
                &other.core,
                ptr::null(),
                options as *const _ as *const _,
                approx as *const _ as *const _,
            ))
        }
    }

    pub fn add_stroked_path_range<R: ops::RangeBounds<usize>>(
        &mut self,
        other: &Path,
        range: R,
        options: &StrokeOptions,
        approx: &ApproximationOptions,
    ) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathAddStrokedPath(
                &mut self.core,
                &other.core,
                &bl_range(range),
                options as *const _ as *const _,
                approx as *const _ as *const _,
            ))
        }
    }
}

impl Path {
    pub fn translate(&mut self, p: Point) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathTranslate(
                &mut self.core,
                ptr::null(),
                &p as *const _ as *const _,
            ))
        }
    }

    pub fn translate_range<R: ops::RangeBounds<usize>>(
        &mut self,
        range: R,
        p: Point,
    ) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathTranslate(
                &mut self.core,
                &bl_range(range),
                &p as *const _ as *const _,
            ))
        }
    }

    pub fn transform(&mut self, m: Matrix2D) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathTransform(
                &mut self.core,
                ptr::null(),
                &m as *const _ as *const _,
            ))
        }
    }

    pub fn transform_range<R: ops::RangeBounds<usize>>(
        &mut self,
        range: R,
        m: Matrix2D,
    ) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathTransform(
                &mut self.core,
                &bl_range(range),
                &m as *const _ as *const _,
            ))
        }
    }

    pub fn fit_to(&mut self, rect: Rect, flags: PathFitFlags) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathFitTo(
                &mut self.core,
                ptr::null(),
                &rect as *const _ as *const _,
                flags.bits(),
            ))
        }
    }

    pub fn fit_to_range<R: ops::RangeBounds<usize>>(
        &mut self,
        range: R,
        rect: Rect,
        flags: PathFitFlags,
    ) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPathFitTo(
                &mut self.core,
                &bl_range(range),
                &rect as *const _ as *const _,
                flags.bits(),
            ))
        }
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.core.impl_ as *const _ == other.core.impl_ as *const _
    }
}

impl Drop for Path {
    fn drop(&mut self) {
        self.reset();
    }
}

impl ImplType for Path {
    type CoreType = ffi::BLPathCore;
    const IMPL_TYPE_ID: usize = ffi::BLImplType::BL_IMPL_TYPE_PATH2D as usize;
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Path {
    fn clone(&self) -> Self {
        let mut core = ffi::BLPathCore {
            impl_: ptr::null_mut(),
        };
        unsafe {
            ffi::blVariantInitWeak(
                &mut core as *mut _ as *mut _,
                &self.core as *const _ as *const _,
            )
        };
        Path { core }
    }
}
