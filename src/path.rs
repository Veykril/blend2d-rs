#![allow(clippy::too_many_arguments)]
use bitflags::bitflags;

use core::{
    borrow::Borrow,
    fmt, mem,
    ops::{self, Range},
    ptr, slice,
};

use crate::{
    array::Array,
    error::{errcode_to_result, expect_mem_err, OutOfMemory},
    geometry::{BoxD, FillRule, Geometry, GeometryDirection, HitTest, Point, PointD, RectD},
    matrix::Matrix2D,
    util::bl_range,
    variant::WrappedBlCore,
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

use ffi::BLStrokeCapPosition::*;
bl_enum! {
    pub enum StrokeCapPosition {
        Start = BL_STROKE_CAP_POSITION_START,
        End = BL_STROKE_CAP_POSITION_END,
    }
    Default => Start
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
#[derive(Copy, Clone, Debug)]
pub struct ApproximationOptions {
    flatten_mode: u8,
    offset_mode: u8,
    _reserved_flags: [u8; 6],
    pub flatten_tolerance: f64,
    pub simplyify_tolerance: f64,
    pub offset_parameter: f64,
}

impl ApproximationOptions {
    #[inline]
    pub fn set_flatten_mode(&mut self, mode: FlattenMode) {
        self.flatten_mode = mode as u8;
    }

    #[inline]
    pub fn flatten_mode(&self) -> FlattenMode {
        (self.flatten_mode as u32).into()
    }

    #[inline]
    pub fn set_offset_mode(&mut self, mode: OffsetMode) {
        self.offset_mode = mode as u8;
    }

    #[inline]
    pub fn offset_mode(&self) -> OffsetMode {
        (self.offset_mode as u32).into()
    }
}

impl Default for ApproximationOptions {
    #[inline]
    fn default() -> Self {
        unsafe { *(&ffi::blDefaultApproximationOptions as *const _ as *const ApproximationOptions) }
    }
}

#[repr(transparent)]
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

    #[inline]
    pub fn width(&self) -> f64 {
        self.core.width
    }

    #[inline]
    pub fn miter_limit(&self) -> f64 {
        self.core.miterLimit
    }

    #[inline]
    pub fn join(&self) -> StrokeJoin {
        unsafe { (self.core.__bindgen_anon_1.__bindgen_anon_1.join as u32).into() }
    }

    #[inline]
    pub fn dash_offset(&self) -> f64 {
        self.core.dashOffset
    }

    #[inline]
    pub fn start_cap(&self) -> StrokeCap {
        unsafe { (self.core.__bindgen_anon_1.__bindgen_anon_1.startCap as u32).into() }
    }

    #[inline]
    pub fn end_cap(&self) -> StrokeCap {
        unsafe { (self.core.__bindgen_anon_1.__bindgen_anon_1.endCap as u32).into() }
    }

    #[inline]
    pub fn dash_array(&self) -> &Array<f64> {
        unsafe { &*(&self.core.dashArray as *const _ as *const _) }
    }

    #[inline]
    pub fn transform_order(&self) -> StrokeTransformOrder {
        unsafe { (self.core.__bindgen_anon_1.__bindgen_anon_1.transformOrder as u32).into() }
    }

    #[inline]
    pub fn set_caps(&mut self, cap: StrokeCap) {
        unsafe {
            self.core.__bindgen_anon_1.__bindgen_anon_1.startCap = cap as u8;
            self.core.__bindgen_anon_1.__bindgen_anon_1.endCap = cap as u8;
        }
    }
}

impl Default for StrokeOptions {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for StrokeOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StrokeOptions").finish()
    }
}

impl Drop for StrokeOptions {
    fn drop(&mut self) {
        unsafe { ffi::blStrokeOptionsReset(&mut self.core) };
    }
}

/// A 2D vector path.
#[repr(transparent)]
pub struct Path {
    core: ffi::BLPathCore,
}

unsafe impl WrappedBlCore for Path {
    type Core = ffi::BLPathCore;
    const IMPL_TYPE_INDEX: usize = crate::variant::ImplType::Path as usize;

    #[inline]
    fn from_core(core: Self::Core) -> Self {
        Path { core }
    }
}

impl Path {
    /// Creates a new empty path.
    #[inline]
    pub fn new() -> Self {
        Path::from_core(*Self::none())
    }

    /// Creates a new path with space for n elements before having to
    /// reallocate.
    #[inline]
    pub fn with_capacity(n: usize) -> Self {
        let mut this = Self::new();
        this.reserve(n);
        this
    }

    /// Clears the path.
    #[inline]
    pub fn clear(&mut self) {
        unsafe { ffi::blPathClear(self.core_mut()) };
    }

    /// Shrinks the path's allocated capacity down to its currently used size.
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        unsafe { expect_mem_err(ffi::blPathShrink(self.core_mut())) };
    }

    /// Reserves capacity for at least n items.
    ///
    /// # Panics
    ///
    /// Panics if blend2d returns an
    /// [`OutOfMemory`](../error/enum.Error.html#variant.OutOfMemory) error
    #[inline]
    pub fn reserve(&mut self, n: usize) {
        self.try_reserve(n).expect("memory allocation failed");
    }

    /// Reserves capacity for at least n items.
    pub fn try_reserve(&mut self, n: usize) -> std::result::Result<(), OutOfMemory> {
        unsafe { OutOfMemory::from_errcode(ffi::blPathReserve(self.core_mut(), n)) }
    }

    /// Returns the current number of vertices in the path.
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { self.impl_().__bindgen_anon_1.view.size }
    }

    /// Returns the currently allocated capacity of the path.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.impl_().capacity as usize
    }

    /// Returns true if the path has no vertices.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the path's command data.
    #[inline]
    pub fn command_data(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.impl_().__bindgen_anon_1.view.commandData, self.len()) }
    }

    /// Returns the path's vertex data.
    #[inline]
    pub fn vertex_data(&self) -> &[PointD] {
        unsafe {
            slice::from_raw_parts(
                ffi::blPathGetVertexData(self.core()) as *const _,
                self.len(),
            )
        }
    }

    /// Returns this path's flags, or `None` if its geometry is invalid.
    #[inline]
    pub fn info_flags(&self) -> Option<PathFlags> {
        unsafe {
            let mut flags = 0;
            errcode_to_result(ffi::blPathGetInfoFlags(self.core(), &mut flags))
                .map(|_| PathFlags::from_bits_truncate(flags))
                .ok()
        }
    }

    /// Retrieves a bounding box of all vertices and control points.
    #[inline]
    pub fn control_box(&self) -> Option<BoxD> {
        unsafe {
            let mut box2d = BoxD::default();
            errcode_to_result(ffi::blPathGetControlBox(
                self.core(),
                &mut box2d as *mut _ as *mut _,
            ))
            .map(|_| box2d)
            .ok()
        }
    }

    /// Retrieves a bounding box of all on-path vertices and curve extremas.
    #[inline]
    pub fn bounding_box(&self) -> Option<BoxD> {
        unsafe {
            let mut box2d = BoxD::default();
            errcode_to_result(ffi::blPathGetBoundingBox(
                self.core(),
                &mut box2d as *mut _ as *mut _,
            ))
            .map(|_| box2d)
            .ok()
        }
    }

    /// Returns the range describing a figure at the given index.
    #[inline]
    pub fn figure_range(&self, index: usize) -> Option<Range<usize>> {
        unsafe {
            let mut range = ffi::BLRange { start: 0, end: 0 };
            errcode_to_result(ffi::blPathGetFigureRange(self.core(), index, &mut range))
                .map(|_| Range {
                    start: range.start,
                    end: range.end,
                })
                .ok()
        }
    }

    #[inline]
    pub fn last_vertex(&self) -> Option<PointD> {
        unsafe {
            let mut point = PointD::default();
            errcode_to_result(ffi::blPathGetLastVertex(
                self.core(),
                &mut point as *mut _ as *mut _,
            ))
            .map(|_| point)
            .ok()
        }
    }

    #[inline]
    pub fn closest_vertex(&self, p: &PointD, max_distance: f64) -> Option<(usize, f64)> {
        unsafe {
            let mut idx = 0;
            let mut dout = 0.0;
            errcode_to_result(ffi::blPathGetClosestVertex(
                self.core(),
                p as *const _ as *const _,
                max_distance,
                &mut idx,
                &mut dout,
            ))
            .map(|_| (idx, dout))
            .ok()
        }
    }

    /// Hit tests the given point p by respecting the given [`FillRule`].
    #[inline]
    pub fn hit_test(&self, p: &PointD, fill_rule: FillRule) -> HitTest {
        unsafe {
            ffi::blPathHitTest(self.core(), p as *const _ as *const _, fill_rule as u32).into()
        }
    }

    /// Sets the vertex at the index to the given [`PathCommand`] and point.
    #[inline]
    pub fn set_vertex_at(&mut self, index: usize, cmd: PathCommand, x: f64, y: f64) {
        unsafe { ffi::blPathSetVertexAt(self.core_mut(), index, cmd as u32, x, y) };
    }

    /// Sets the vertex at the index to the given [`PathCommand`] and point.
    #[inline]
    pub fn set_vertex_at_point(&mut self, index: usize, cmd: PathCommand, point: PointD) {
        unsafe { ffi::blPathSetVertexAt(self.core_mut(), index, cmd as u32, point.x, point.y) };
    }
}

impl Path {
    /// Moves to the point.
    #[inline]
    pub fn move_to(&mut self, x: f64, y: f64) {
        unsafe { expect_mem_err(ffi::blPathMoveTo(self.core_mut(), x, y)) };
    }

    /// Moves to the point.
    #[inline]
    pub fn move_to_point(&mut self, point: &PointD) {
        unsafe { expect_mem_err(ffi::blPathMoveTo(self.core_mut(), point.x, point.y)) };
    }

    /// Adds a line from the current to the given point to this path.
    #[inline]
    pub fn line_to(&mut self, x: f64, y: f64) {
        unsafe { expect_mem_err(ffi::blPathLineTo(self.core_mut(), x, y)) };
    }

    /// Adds a line from the current to the given point to this path.
    #[inline]
    pub fn line_to_point(&mut self, point: &PointD) {
        unsafe { expect_mem_err(ffi::blPathLineTo(self.core_mut(), point.x, point.y)) };
    }

    /// Adds a polyline (LineTo) of the given points.
    #[inline]
    pub fn poly_to(&mut self, poly: &[PointD]) {
        unsafe {
            expect_mem_err(ffi::blPathPolyTo(
                self.core_mut(),
                poly.as_ptr() as *const _,
                poly.len(),
            ))
        };
    }

    /// Adds a quadratic curve to the first and second point.
    ///
    /// Matches SVG 'Q' path command:
    ///   - https://www.w3.org/TR/SVG/paths.html#PathDataQuadraticBezierCommands
    #[inline]
    pub fn quad_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        unsafe { expect_mem_err(ffi::blPathQuadTo(self.core_mut(), x1, y1, x2, y2)) };
    }

    /// Adds a quadratic curve to the first and second point.
    ///
    /// Matches SVG 'Q' path command:
    ///   - https://www.w3.org/TR/SVG/paths.html#PathDataQuadraticBezierCommands
    #[inline]
    pub fn quad_to_points(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        unsafe { expect_mem_err(ffi::blPathQuadTo(self.core_mut(), x1, y1, x2, y2)) };
    }

    /// Adds a cubic curve to the first, second and third point.
    ///
    /// Matches SVG 'C' path command:
    ///   - https://www.w3.org/TR/SVG/paths.html#PathDataCubicBezierCommands
    #[inline]
    pub fn cubic_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        unsafe { expect_mem_err(ffi::blPathCubicTo(self.core_mut(), x1, y1, x2, y2, x3, y3)) };
    }

    /// Adds a cubic curve to the first, second and third point.
    ///
    /// Matches SVG 'C' path command:
    ///   - https://www.w3.org/TR/SVG/paths.html#PathDataCubicBezierCommands
    #[inline]
    #[rustfmt::skip]
    pub fn cubic_to_points(&mut self, p1: &PointD, p2: &PointD, p3: &PointD) {
        unsafe {
            expect_mem_err(
                ffi::blPathCubicTo(self.core_mut(), p1.x, p1.y, p2.x, p2.y, p3.x, p3.y)
            )
        };
    }

    /// Adds a smooth quadratic curve to the given point, calculating the first from previous points.
    ///
    /// Matches SVG 'T' path command:
    ///   - https://www.w3.org/TR/SVG/paths.html#PathDataQuadraticBezierCommands
    #[inline]
    pub fn smooth_quad_to(&mut self, x2: f64, y2: f64) {
        unsafe { expect_mem_err(ffi::blPathSmoothQuadTo(self.core_mut(), x2, y2)) };
    }

    /// Adds a smooth quadratic curve to the given point, calculating the first from previous points.
    ///
    /// Matches SVG 'T' path command:
    ///   - https://www.w3.org/TR/SVG/paths.html#PathDataQuadraticBezierCommands
    #[inline]
    pub fn smooth_quad_to_point(&mut self, p2: &PointD) {
        unsafe { expect_mem_err(ffi::blPathSmoothQuadTo(self.core_mut(), p2.x, p2.y)) };
    }

    /// Adds a smooth cubic curve to the given points, calculating the first from previous points.
    ///
    /// Matches SVG 'S' path command:
    ///   - https://www.w3.org/TR/SVG/paths.html#PathDataCubicBezierCommands
    #[inline]
    pub fn smooth_cubic_to(&mut self, x2: f64, y2: f64, x3: f64, y3: f64) {
        unsafe { expect_mem_err(ffi::blPathSmoothCubicTo(self.core_mut(), x2, y2, x3, y3)) };
    }

    /// Adds a smooth cubic curve to the given points, calculating the first from previous points.
    ///
    /// Matches SVG 'S' path command:
    ///   - https://www.w3.org/TR/SVG/paths.html#PathDataCubicBezierCommands
    #[inline]
    #[rustfmt::skip]
    pub fn smooth_cubic_to_points(&mut self, p2: &PointD, p3: &PointD) {
        unsafe {
            expect_mem_err(ffi::blPathSmoothCubicTo(self.core_mut(), p2.x, p2.y, p3.x, p3.y))
        };
    }

    /// Adds an arc to the path.
    ///
    /// The center of the arc is specified by `c` and radius by `r`. Both `start`
    /// and `sweep` angles are in radians. If the last vertex doesn't match the
    /// start of the arc then a [`Path::line_to`] would be emitted before adding the arc.
    /// Pass `true` in `force_move_to` to always emit [`Path::move_to`] at the beginning of
    /// the arc, which starts a new figure.
    /// 
    /// [`Path::line_to`](struct.Path.html#method.line_to)
    /// [`Path::move_to`](struct.Path.html#method.move_to)
    #[inline]
    #[rustfmt::skip]
    pub fn arc_to(&mut self, cx: f64, cy: f64, rx: f64, ry: f64, start: f64, sweep: f64, force_move_to: bool) {
        unsafe {
            expect_mem_err(
                ffi::blPathArcTo(self.core_mut(), cx, cy, rx, ry, start, sweep, force_move_to)
            )
        };
    }

    /// Adds an arc to the path.
    ///
    /// The center of the arc is specified by `cp` and radius by `rp`. Both `start`
    /// and `sweep` angles are in radians. If the last vertex doesn't match the
    /// start of the arc then a [`Path::line_to`] would be emitted before adding the arc.
    /// Pass `true` in `force_move_to` to always emit [`Path::move_to`] at the beginning of
    /// the arc, which starts a new figure.
    /// 
    /// [`Path::line_to`](struct.Path.html#method.line_to)
    /// [`Path::move_to`](struct.Path.html#method.move_to)
    #[inline]
    #[rustfmt::skip]
    pub fn arc_to_points(&mut self, cp: &PointD, rp: &PointD, start: f64, sweep: f64, force_move_to: bool) {
        unsafe {
            expect_mem_err(
                ffi::blPathArcTo(self.core_mut(), cp.x, cp.y, rp.x, rp.y, start, sweep, force_move_to)
            )
        };
    }

    /// Adds an arc quadrant (90deg) to the path. The first point specifies
    /// the quadrant corner and the last point specifies the end point.
    #[inline]
    pub fn arc_quadrant_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        unsafe { expect_mem_err(ffi::blPathArcQuadrantTo(self.core_mut(), x1, y1, x2, y2)) }
    }

    /// Adds an arc quadrant (90deg) to the path. The first point specifies
    /// the quadrant corner and the last point specifies the end point.
    #[inline]
    #[rustfmt::skip]
    pub fn arc_quadrant_to_points(&mut self, p1: &PointD, p2: &PointD) {
        unsafe {
            expect_mem_err(
                ffi::blPathArcQuadrantTo(self.core_mut(), p1.x, p1.y, p2.x, p2.y)
            )
        };
    }

    /// Adds an elliptic arc to the path that follows the SVG specification.
    ///
    /// Matches SVG 'A' path command:
    ///   - https://www.w3.org/TR/SVG/paths.html#PathDataEllipticalArcCommands
    #[inline]
    #[rustfmt::skip]
    pub fn elliptic_arc_to(&mut self, rx: f64, ry: f64, x_axis_rotation: f64, large_arc_flag: bool, sweep_flag: bool, x1: f64, y1: f64) {
        unsafe {
            expect_mem_err(
                ffi::blPathEllipticArcTo(self.core_mut(), rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, x1, y1)
            )
        };
    }

    /// Adds an elliptic arc to the path that follows the SVG specification.
    ///
    /// Matches SVG 'A' path command:
    ///   - https://www.w3.org/TR/SVG/paths.html#PathDataEllipticalArcCommands
    #[inline]
    #[rustfmt::skip]
    pub fn elliptic_arc_to_points(&mut self, rp: &PointD, x_axis_rotation: f64, large_arc_flag: bool, sweep_flag: bool, p1: &PointD) {
        unsafe {
            expect_mem_err(
                ffi::blPathEllipticArcTo(self.core_mut(), rp.x, rp.y, x_axis_rotation, large_arc_flag, sweep_flag, p1.x, p1.y)
            )
        };
    }

    /// Closes the current figure.
    ///
    /// Matches SVG 'Z' path command:
    ///   - https://www.w3.org/TR/SVG/paths.html#PathDataClosePathCommand
    #[inline]
    pub fn close(&mut self) {
        unsafe { expect_mem_err(ffi::blPathClose(self.core_mut())) };
    }
}

impl Path {
    /// Adds a [`Geometry`] to the path.
    #[inline]
    pub fn add_geometry<T: Geometry + ?Sized>(
        &mut self,
        g: &T,
        matrix: Option<&Matrix2D>,
        dir: GeometryDirection,
    ) {
        unsafe {
            expect_mem_err(ffi::blPathAddGeometry(
                self.core_mut(),
                T::GEO_TYPE,
                g as *const _ as *const _,
                matrix.map_or(ptr::null(), |m| m as *const _ as *const _),
                dir.into(),
            ))
        };
    }

    /// Adds a polygon.
    #[inline]
    pub fn add_polygon<R, P>(&mut self, p: R, matrix: Option<&Matrix2D>, dir: GeometryDirection)
    where
        R: AsRef<[P]>,
        P: Point + Geometry,
    {
        unsafe {
            expect_mem_err(ffi::blPathAddGeometry(
                self.core_mut(),
                P::GEO_TYPE,
                p.as_ref().as_ptr() as *const _,
                matrix.map_or(ptr::null(), |m| m as *const _ as *const _),
                dir.into(),
            ))
        };
    }

    /// Adds a polyline.
    #[inline]
    pub fn add_polyline<R, P>(&mut self, p: R, matrix: Option<&Matrix2D>, dir: GeometryDirection)
    where
        R: AsRef<[P]>,
        P: Point,
    {
        unsafe {
            expect_mem_err(ffi::blPathAddGeometry(
                self.core_mut(),
                P::POLYLINE_TYPE,
                p.as_ref().as_ptr() as *const _,
                matrix.map_or(ptr::null(), |m| m as *const _ as *const _),
                dir.into(),
            ))
        };
    }

    /// Adds another path.
    #[inline]
    pub fn add_path(&mut self, other: &Path) {
        unsafe {
            expect_mem_err(ffi::blPathAddPath(
                self.core_mut(),
                other.core(),
                ptr::null(),
            ))
        };
    }

    /// Adds a part of another path.
    #[inline]
    pub fn add_path_range<R: ops::RangeBounds<usize>>(&mut self, other: &Path, range: R) {
        unsafe {
            expect_mem_err(ffi::blPathAddPath(
                self.core_mut(),
                other.core(),
                &bl_range(range),
            ))
        };
    }

    /// Adds a translated path.
    #[inline]
    pub fn add_translated_path(&mut self, other: &Path, p: &PointD) {
        unsafe {
            expect_mem_err(ffi::blPathAddTranslatedPath(
                self.core_mut(),
                other.core(),
                ptr::null(),
                p as *const _ as *const _,
            ))
        };
    }

    /// Adds a translated part of another path.
    #[inline]
    pub fn add_translated_path_range<R: ops::RangeBounds<usize>>(
        &mut self,
        other: &Path,
        range: R,
        p: &PointD,
    ) {
        unsafe {
            expect_mem_err(ffi::blPathAddTranslatedPath(
                self.core_mut(),
                other.core(),
                &bl_range(range),
                p as *const _ as *const _,
            ))
        };
    }

    /// Adds a transformed path.
    #[inline]
    pub fn add_transformed_path(&mut self, other: &Path, m: &Matrix2D) {
        unsafe {
            expect_mem_err(ffi::blPathAddTransformedPath(
                self.core_mut(),
                other.core(),
                ptr::null(),
                m as *const _ as *const _,
            ))
        };
    }

    /// Adds a transformed part of another path.
    #[inline]
    pub fn add_transformed_path_range<R: ops::RangeBounds<usize>>(
        &mut self,
        other: &Path,
        range: R,
        m: &Matrix2D,
    ) {
        unsafe {
            expect_mem_err(ffi::blPathAddTransformedPath(
                self.core_mut(),
                other.core(),
                &bl_range(range),
                m as *const _ as *const _,
            ))
        };
    }

    /// Adds a reversed path.
    #[inline]
    pub fn add_reversed_path(&mut self, other: &Path, mode: PathReverseMode) {
        unsafe {
            expect_mem_err(ffi::blPathAddReversedPath(
                self.core_mut(),
                other.core(),
                ptr::null(),
                mode.into(),
            ))
        };
    }

    /// Adds a reversed part of another path.
    #[inline]
    pub fn add_reversed_path_range<R: ops::RangeBounds<usize>>(
        &mut self,
        other: &Path,
        range: R,
        mode: PathReverseMode,
    ) {
        unsafe {
            expect_mem_err(ffi::blPathAddReversedPath(
                self.core_mut(),
                other.core(),
                &bl_range(range),
                mode.into(),
            ))
        };
    }

    /// Adds a stroked path.
    #[inline]
    pub fn add_stroked_path(
        &mut self,
        other: &Path,
        options: &StrokeOptions,
        approx: &ApproximationOptions,
    ) {
        unsafe {
            expect_mem_err(ffi::blPathAddStrokedPath(
                self.core_mut(),
                other.core(),
                ptr::null(),
                options as *const _ as *const _,
                approx as *const _ as *const _,
            ))
        };
    }

    /// Adds a stroked part of another path.
    #[inline]
    pub fn add_stroked_path_range<R: ops::RangeBounds<usize>>(
        &mut self,
        other: &Path,
        range: R,
        options: &StrokeOptions,
        approx: &ApproximationOptions,
    ) {
        unsafe {
            expect_mem_err(ffi::blPathAddStrokedPath(
                self.core_mut(),
                other.core(),
                &bl_range(range),
                options as *const _ as *const _,
                approx as *const _ as *const _,
            ))
        }
    }
}

impl Path {
    /// Translates the whole path by the given point.
    #[inline]
    pub fn translate(&mut self, p: &PointD) {
        unsafe {
            expect_mem_err(ffi::blPathTranslate(
                self.core_mut(),
                ptr::null(),
                p as *const _ as *const _,
            ))
        };
    }

    /// Translates a part of the path by the given point.
    #[inline]
    pub fn translate_range<R: ops::RangeBounds<usize>>(&mut self, range: R, p: &PointD) {
        unsafe {
            expect_mem_err(ffi::blPathTranslate(
                self.core_mut(),
                &bl_range(range),
                p as *const _ as *const _,
            ))
        };
    }

    /// Transforms the whole path by the given transformation matrix.
    #[inline]
    pub fn transform(&mut self, m: &Matrix2D) {
        unsafe {
            expect_mem_err(ffi::blPathTransform(
                self.core_mut(),
                ptr::null(),
                m as *const _ as *const _,
            ))
        };
    }

    /// Transforms a part of the path by the given transformation matrix.
    #[inline]
    pub fn transform_range<R: ops::RangeBounds<usize>>(&mut self, range: R, m: &Matrix2D) {
        unsafe {
            expect_mem_err(ffi::blPathTransform(
                self.core_mut(),
                &bl_range(range),
                m as *const _ as *const _,
            ))
        };
    }

    /// Fits the whole path into the given rect by taking into account fit flags
    /// passed by [`PathFitFlags`].
    #[inline]
    pub fn fit_to(&mut self, rect: &RectD, flags: PathFitFlags) {
        unsafe {
            expect_mem_err(ffi::blPathFitTo(
                self.core_mut(),
                ptr::null(),
                rect as *const _ as *const _,
                flags.bits(),
            ))
        };
    }

    /// Fits a part of the path specified by the given range into the given rect
    /// by taking into account the given [`PathFitFlags`].
    #[inline]
    pub fn fit_to_range<R: ops::RangeBounds<usize>>(
        &mut self,
        range: R,
        rect: &RectD,
        flags: PathFitFlags,
    ) {
        unsafe {
            expect_mem_err(ffi::blPathFitTo(
                self.core_mut(),
                &bl_range(range),
                rect as *const _ as *const _,
                flags.bits(),
            ))
        };
    }
}

impl PartialEq for Path {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::blPathEquals(self.core(), other.core()) }
    }
}

impl Drop for Path {
    fn drop(&mut self) {
        unsafe { ffi::blPathReset(&mut self.core) };
    }
}

impl Default for Path {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Borrow<[PointD]> for Path {
    #[inline]
    fn borrow(&self) -> &[PointD] {
        self.vertex_data()
    }
}

impl AsRef<[PointD]> for Path {
    #[inline]
    fn as_ref(&self) -> &[PointD] {
        self.vertex_data()
    }
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Path")
            .field("vertex_data", &self.vertex_data())
            .finish()
    }
}

impl Clone for Path {
    fn clone(&self) -> Self {
        Self::from_core(self.init_weak())
    }
}
