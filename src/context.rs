use bitflags::bitflags;

use core::{mem, ptr};

use crate::{
    array::Array,
    error::{errcode_to_result, Result},
    geometry::{
        Arc, BoxD, Chord, Circle, Ellipse, GeoViewArray, Geometry, Pie, Point, Rect, RectD,
        RoundRect, SizeD, Triangle,
    },
    gradient::{Conical, DynamicGradient, Gradient, GradientType, Linear, LinearGradient, Radial},
    image::Image,
    matrix::{Matrix2DOp, MatrixTransform},
    path::{
        ApproximationOptions, FlattenMode, StrokeCap, StrokeCapPosition, StrokeJoin, StrokeOptions,
    },
    pattern::Pattern,
    variant::{BlVariantCore, BlVariantImpl, WrappedBlCore},
    StyleType,
};

use ffi::BLContextType::*;
bl_enum! {
    pub enum ContextType {
        None = BL_CONTEXT_TYPE_NONE,
        Dummy = BL_CONTEXT_TYPE_DUMMY,
        Raster = BL_CONTEXT_TYPE_RASTER,
        RasterAsync = BL_CONTEXT_TYPE_RASTER_ASYNC,
    }
    Default => None
}

use ffi::BLContextHint::*;
bl_enum! {
    pub enum ContextHint {
        RenderingQuality = BL_CONTEXT_HINT_RENDERING_QUALITY,
        GradientQuality = BL_CONTEXT_HINT_GRADIENT_QUALITY,
        PatternQuality = BL_CONTEXT_HINT_PATTERN_QUALITY,
    }
    Default => RenderingQuality
}

use ffi::BLContextOpType::*;
bl_enum! {
    pub enum ContextOpType {
        Fill = BL_CONTEXT_OP_TYPE_FILL,
        Stroke = BL_CONTEXT_OP_TYPE_STROKE,
    }
    Default => Fill
}

use ffi::BLContextFlushFlags;
bitflags! {
    pub struct ContextFlushFlags: u32 {
        const FLUSH_SYNC = BLContextFlushFlags::BL_CONTEXT_FLUSH_SYNC as u32;
    }
}

use ffi::BLContextCreateFlags;
bitflags! {
    pub struct ContextCreateFlags: u32 {
        const ISOLATED_RUNTIME = BLContextCreateFlags::BL_CONTEXT_CREATE_FLAG_ISOLATED_RUNTIME as u32;
        const OVERRIDE_FEATURES = BLContextCreateFlags::BL_CONTEXT_CREATE_FLAG_OVERRIDE_FEATURES as u32;
    }
}

use ffi::BLClipOp::*;
bl_enum! {
    pub enum ClipOP {
        Replace = BL_CLIP_OP_REPLACE,
        Intersect = BL_CLIP_OP_INTERSECT,
    }
    Default => Replace
}

use ffi::BLClipMode::*;
bl_enum! {
    pub enum ClipMode {
        AlignedRect = BL_CLIP_MODE_ALIGNED_RECT,
        UnalignedRect = BL_CLIP_MODE_UNALIGNED_RECT,
        Mask = BL_CLIP_MODE_MASK,
    }
    Default => AlignedRect
}

use ffi::BLCompOp::*;
bl_enum! {
    pub enum CompOp {
        SrcOver = BL_COMP_OP_SRC_OVER,
        SrcCopy = BL_COMP_OP_SRC_COPY,
        SrcIn = BL_COMP_OP_SRC_IN,
        SrcOut = BL_COMP_OP_SRC_OUT,
        SrcAtop = BL_COMP_OP_SRC_ATOP,
        DstOver = BL_COMP_OP_DST_OVER,
        DstCopy = BL_COMP_OP_DST_COPY,
        DstIn = BL_COMP_OP_DST_IN,
        DstOut = BL_COMP_OP_DST_OUT,
        DstAtop = BL_COMP_OP_DST_ATOP,
        Xor = BL_COMP_OP_XOR,
        Clear = BL_COMP_OP_CLEAR,
        Plus = BL_COMP_OP_PLUS,
        Minus = BL_COMP_OP_MINUS,
        Multiply = BL_COMP_OP_MULTIPLY,
        Screen = BL_COMP_OP_SCREEN,
        Overlay = BL_COMP_OP_OVERLAY,
        Darken = BL_COMP_OP_DARKEN,
        Lighten = BL_COMP_OP_LIGHTEN,
        ColorDodge = BL_COMP_OP_COLOR_DODGE,
        ColorBurn = BL_COMP_OP_COLOR_BURN,
        LinearBurn = BL_COMP_OP_LINEAR_BURN,
        LinearLight = BL_COMP_OP_LINEAR_LIGHT,
        PinLight = BL_COMP_OP_PIN_LIGHT,
        HardLight = BL_COMP_OP_HARD_LIGHT,
        SoftLight = BL_COMP_OP_SOFT_LIGHT,
        Difference = BL_COMP_OP_DIFFERENCE,
        Exclusion = BL_COMP_OP_EXCLUSION,
    }
    Default => SrcOver
}

use ffi::BLGradientQuality::*;
bl_enum! {
    pub enum GradientQuality {
        Nearest = BL_GRADIENT_QUALITY_NEAREST,
    }
    Default => Nearest
}

use ffi::BLPatternQuality::*;
bl_enum! {
    pub enum PatternQuality {
        Nearest = BL_PATTERN_QUALITY_NEAREST,
        Bilinear = BL_PATTERN_QUALITY_BILINEAR,
    }
    Default => Nearest
}

use crate::{
    geometry::{Line, RectI},
    path::Path,
};
use ffi::BLRenderingQuality::*;
bl_enum! {
    pub enum RenderingQuality {
        AntiAliasing = BL_RENDERING_QUALITY_ANTIALIAS,
    }
    Default => AntiAliasing
}

#[repr(C)]
#[derive(Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct ContextCookie(u128);

#[repr(C)]
pub struct ContextHints {
    pub rendering_quality: u8,
    pub gradient_quality: u8,
    pub pattern_quality: u8,
}

#[repr(transparent)]
pub struct Context {
    core: ffi::BLContextCore,
}

unsafe impl WrappedBlCore for Context {
    type Core = ffi::BLContextCore;
    const IMPL_TYPE_INDEX: usize = ffi::BLImplType::BL_IMPL_TYPE_CONTEXT as usize;
}

impl Context {
    #[inline]
    pub fn new(target: &mut Image) -> Result<Context> {
        Self::new_with_options(target, None)
    }

    // FIXME figure out how ContextCreateOptions is used
    fn new_with_options(
        target: &mut Image,
        options: Option<ffi::BLContextCreateOptions>,
    ) -> Result<Context> {
        unsafe {
            let mut this = Context {
                core: *Self::none(),
            };
            errcode_to_result(ffi::blContextInitAs(
                this.core_mut(),
                target.core_mut(),
                options.as_ref().map_or(ptr::null(), |ptr| ptr as *const _),
            ))
            .map(move |_| this)
        }
    }

    #[inline]
    pub fn target_size(&self) -> SizeD {
        let ffi::BLSize { w, h } = self.impl_().targetSize;
        SizeD { w, h }
    }

    #[inline]
    pub fn target_width(&self) -> f64 {
        self.target_size().w
    }

    #[inline]
    pub fn target_height(&self) -> f64 {
        self.target_size().h
    }

    #[inline]
    pub fn context_type(&self) -> ContextType {
        self.impl_().contextType.into()
    }

    #[inline]
    pub fn begin(&mut self, image: &mut Image) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blContextBegin(
                self.core_mut(),
                image.core_mut(),
                ptr::null(),
            ))
        }
    }

    #[inline]
    pub fn end(mut self) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextEnd(self.core_mut())) }
    }

    #[inline]
    pub fn flush(&mut self, flags: ContextFlushFlags) {
        unsafe { ffi::blContextFlush(self.core_mut(), flags.bits()) };
    }

    #[inline]
    pub fn saved_state_count(&self) -> usize {
        self.state().savedStateCount
    }

    #[inline]
    pub fn save(&mut self) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextSave(self.core_mut(), ptr::null_mut())) }
    }

    #[inline]
    pub fn save_cookie(&mut self) -> Result<ContextCookie> {
        unsafe {
            let mut cookie = ContextCookie::default();
            errcode_to_result(ffi::blContextSave(
                self.core_mut(),
                &mut cookie as *mut _ as *mut _,
            ))
            .map(|_| cookie)
        }
    }

    #[inline]
    pub fn restore(&mut self) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextRestore(self.core_mut(), ptr::null_mut())) }
    }

    #[inline]
    pub fn restore_cookie(&mut self, cookie: ContextCookie) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blContextRestore(
                self.core_mut(),
                &cookie as *const _ as *const _,
            ))
        }
    }

    #[inline]
    pub fn with_pushed_context<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut Self) -> Result<()>,
    {
        self.save()?;
        f(self)?;
        self.restore()
    }

    #[inline]
    pub fn hints(&self) -> &ContextHints {
        unsafe { &*(&self.state().__bindgen_anon_1.hints as *const _ as *const _) }
    }

    #[inline]
    pub fn set_hint(&mut self, hint: ContextHint, value: u32) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextSetHint(self.core_mut(), hint.into(), value)) }
    }

    #[inline]
    pub fn approximation_options(&self) -> &ApproximationOptions {
        unsafe { &*(&self.state().approximationOptions as *const _ as *const _) }
    }

    #[inline]
    pub fn flatten_mode(&self) -> FlattenMode {
        self.approximation_options().flatten_mode()
    }

    #[inline]
    pub fn set_flatten_mode(&mut self, mode: FlattenMode) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextSetFlattenMode(self.core_mut(), mode.into())) }
    }

    #[inline]
    pub fn flatten_tolerance(&self) -> f64 {
        self.approximation_options().flatten_tolerance
    }

    #[inline]
    pub fn set_flatten_tolerance(&mut self, tolerance: f64) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blContextSetFlattenTolerance(
                self.core_mut(),
                tolerance,
            ))
        }
    }

    #[inline]
    pub fn comp_op(&self) -> CompOp {
        (self.state().compOp as u32).into()
    }

    #[inline]
    pub fn set_comp_op(&mut self, comp_op: CompOp) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextSetCompOp(self.core_mut(), comp_op.into())) }
    }

    #[inline]
    pub fn global_alpha(&self) -> f64 {
        self.state().globalAlpha
    }

    #[inline]
    pub fn set_global_alpha(&mut self, alpha: f64) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextSetGlobalAlpha(self.core_mut(), alpha)) }
    }

    #[inline]
    fn state(&self) -> &ffi::BLContextState {
        unsafe { &*self.impl_().state }
    }
}

// FIXME? make functions generic over a Stroke/FillStyle trait?
impl Context {
    #[inline]
    pub fn fill_style_type(&self) -> StyleType {
        unsafe { (self.state().__bindgen_anon_2.__bindgen_anon_1.fillStyleType as u32).into() }
    }

    #[inline]
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn set_fill_style(&mut self, core: &ffi::BLVariantCore) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blContextSetFillStyle(
                self.core_mut(),
                core as *const _ as *const _,
            ))
        }
    }

    #[inline]
    pub fn set_fill_style_gradient<T: GradientType>(
        &mut self,
        gradient: &Gradient<T>,
    ) -> Result<()> {
        self.set_fill_style(gradient.core().as_variant_core())
    }

    #[inline]
    pub fn set_fill_style_pattern(&mut self, pattern: &Pattern) -> Result<()> {
        self.set_fill_style(pattern.core().as_variant_core())
    }

    #[inline]
    pub fn set_fill_style_image(&mut self, image: &Image) -> Result<()> {
        self.set_fill_style(image.core().as_variant_core())
    }

    #[inline]
    pub fn set_fill_style_rgba32(&mut self, color: u32) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextSetFillStyleRgba32(self.core_mut(), color)) }
    }

    #[inline]
    pub fn set_fill_style_rgba64(&mut self, color: u64) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextSetFillStyleRgba64(self.core_mut(), color)) }
    }

    #[inline]
    pub fn get_fill_style_gradient(&self) -> Result<DynamicGradient> {
        unsafe {
            let mut gradient: LinearGradient = mem::zeroed();
            errcode_to_result(ffi::blContextGetFillStyle(
                self.core(),
                gradient.core_mut() as *mut _ as *mut _,
            ))
            .map(|_| match gradient.impl_().gradientType as u32 {
                Linear::BL_TYPE => gradient.with_type::<Linear>().into(),
                Radial::BL_TYPE => gradient.with_type::<Radial>().into(),
                Conical::BL_TYPE => gradient.with_type::<Conical>().into(),
                _ => unreachable!(),
            })
        }
    }

    #[inline]
    pub fn get_fill_style_rgba32(&self) -> Result<u32> {
        unsafe {
            let mut out = 0;
            errcode_to_result(ffi::blContextGetFillStyleRgba32(self.core(), &mut out)).map(|_| out)
        }
    }

    #[inline]
    pub fn get_fill_style_rgba64(&self) -> Result<u64> {
        unsafe {
            let mut out = 0;
            errcode_to_result(ffi::blContextGetFillStyleRgba64(self.core(), &mut out)).map(|_| out)
        }
    }

    #[inline]
    pub fn stroke_width(&self) -> f64 {
        self.state().strokeOptions.width
    }

    #[inline]
    pub fn stroke_miter_limit(&self) -> f64 {
        self.state().strokeOptions.miterLimit
    }

    #[inline]
    pub fn stroke_dast_offset(&self) -> f64 {
        self.state().strokeOptions.miterLimit
    }

    #[inline]
    pub fn stroke_options(&self) -> &StrokeOptions {
        //FIXME? Make a Ref/RefMut wrapper struct that doesnt call the destructor,
        // instead of doing this nasty ref casting
        unsafe { &*(&self.state().strokeOptions as *const _ as *const _) }
    }

    #[inline]
    pub fn set_stroke_miter_limit(&mut self, limit: f64) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextSetStrokeMiterLimit(self.core_mut(), limit)) }
    }

    #[inline]
    pub fn set_stroke_width(&mut self, width: f64) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextSetStrokeWidth(self.core_mut(), width)) }
    }

    #[inline]
    pub fn set_stroke_join(&mut self, join: StrokeJoin) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextSetStrokeJoin(self.core_mut(), join.into())) }
    }

    #[inline]
    pub fn set_stroke_cap(&mut self, pos: StrokeCapPosition, cap: StrokeCap) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blContextSetStrokeCap(
                self.core_mut(),
                pos.into(),
                cap.into(),
            ))
        }
    }

    #[inline]
    pub fn set_stroke_start_cap(&mut self, cap: StrokeCap) -> Result<()> {
        self.set_stroke_cap(StrokeCapPosition::Start, cap)
    }

    #[inline]
    pub fn set_stroke_end_cap(&mut self, cap: StrokeCap) -> Result<()> {
        self.set_stroke_cap(StrokeCapPosition::End, cap)
    }

    #[inline]
    pub fn set_stroke_caps(&mut self, cap: u32) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextSetStrokeCaps(self.core_mut(), cap)) }
    }

    #[inline]
    pub fn set_stroke_dash_offset(&mut self, offset: f64) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextSetStrokeDashOffset(self.core_mut(), offset)) }
    }

    #[inline]
    pub fn set_stroke_dash_array(&mut self, dash_array: &Array<f64>) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blContextSetStrokeDashArray(
                self.core_mut(),
                dash_array.core(),
            ))
        }
    }

    #[inline]
    pub fn set_stroke_options(&mut self, opts: &StrokeOptions) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextSetStrokeOptions(self.core_mut(), &opts.core)) }
    }

    #[inline]
    pub fn stroke_alpha(&self) -> f64 {
        unsafe { self.state().__bindgen_anon_3.__bindgen_anon_1.strokeAlpha }
    }

    #[inline]
    pub fn set_stroke_alpha(&mut self, alpha: f64) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextSetStrokeAlpha(self.core_mut(), alpha)) }
    }

    #[inline]
    pub fn stroke_style_type(&self) -> StyleType {
        unsafe {
            (self
                .state()
                .__bindgen_anon_2
                .__bindgen_anon_1
                .strokeStyleType as u32)
                .into()
        }
    }

    #[inline]
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn set_stroke_style(&mut self, core: &ffi::BLVariantCore) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blContextSetStrokeStyle(
                self.core_mut(),
                core as *const _ as *const _,
            ))
        }
    }

    #[inline]
    pub fn set_stroke_style_gradient<T: GradientType>(
        &mut self,
        gradient: &Gradient<T>,
    ) -> Result<()> {
        self.set_stroke_style(gradient.core().as_variant_core())
    }

    #[inline]
    pub fn set_stroke_style_image(&mut self, image: &Image) -> Result<()> {
        self.set_stroke_style(image.core().as_variant_core())
    }

    #[inline]
    pub fn set_stroke_style_rgba32(&mut self, color: u32) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextSetStrokeStyleRgba32(self.core_mut(), color)) }
    }

    #[inline]
    pub fn set_stroke_style_rgba64(&mut self, color: u64) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextSetStrokeStyleRgba64(self.core_mut(), color)) }
    }

    #[inline]
    pub fn get_stroke_style_gradient(&self) -> Result<DynamicGradient> {
        unsafe {
            let mut gradient: LinearGradient = mem::zeroed();
            errcode_to_result(ffi::blContextGetStrokeStyle(
                self.core(),
                gradient.core_mut() as *mut _ as *mut _,
            ))
            .map(|_| match gradient.impl_().gradientType as u32 {
                Linear::BL_TYPE => gradient.with_type::<Linear>().into(),
                Radial::BL_TYPE => gradient.with_type::<Radial>().into(),
                Conical::BL_TYPE => gradient.with_type::<Conical>().into(),
                _ => unreachable!(),
            })
        }
    }

    #[inline]
    pub fn get_stroke_style_rgba32(&self) -> Result<u32> {
        unsafe {
            let mut out = 0;
            errcode_to_result(ffi::blContextGetStrokeStyleRgba32(self.core(), &mut out))
                .map(|_| out)
        }
    }

    #[inline]
    pub fn get_stroke_style_rgba64(&self) -> Result<u64> {
        unsafe {
            let mut out = 0;
            errcode_to_result(ffi::blContextGetStrokeStyleRgba64(self.core(), &mut out))
                .map(|_| out)
        }
    }

    #[inline]
    fn virt_op_style(&self) -> ffi::BLContextVirt__bindgen_ty_1__bindgen_ty_2 {
        unsafe { self.impl_().virt().__bindgen_anon_1.__bindgen_anon_2 }
    }

    #[inline]
    pub fn set_op_style_rgba32(&mut self, op: ContextOpType, val: u32) -> Result<()> {
        unsafe {
            errcode_to_result((self.virt_op_style().setOpStyleRgba32
                [u32::from(op) as usize]
                .unwrap())(self.impl_mut(), val))
        }
    }

    #[inline]
    pub fn set_op_style_rgba64(&mut self, op: ContextOpType, val: u64) -> Result<()> {
        unsafe {
            errcode_to_result((self.virt_op_style().setOpStyleRgba64
                [u32::from(op) as usize]
                .unwrap())(self.impl_mut(), val))
        }
    }

    #[inline]
    pub fn get_op_style_rgba32(&self, op: ContextOpType) -> Result<u32> {
        unsafe {
            let mut out = 0;
            errcode_to_result((self.virt_op_style().getOpStyleRgba32
                [u32::from(op) as usize]
                .unwrap())(self.impl_mut(), &mut out))
            .map(|_| out)
        }
    }

    #[inline]
    pub fn get_op_style_rgba64(&self, op: ContextOpType) -> Result<u64> {
        unsafe {
            let mut out = 0;
            errcode_to_result((self.virt_op_style().getOpStyleRgba64[op as u32 as usize]
                .unwrap())(self.impl_mut(), &mut out))
            .map(|_| out)
        }
    }

    #[inline]
    pub fn op_alpha(&self, op: ContextOpType) -> f64 {
        unsafe { self.state().__bindgen_anon_3.opAlpha[op as u32 as usize] }
    }

    #[inline]
    pub fn set_op_alpha(&mut self, op: ContextOpType, alpha: f64) -> Result<()> {
        unsafe {
            errcode_to_result((self.virt_op_style().setOpAlpha[op as u32 as usize]
                .unwrap())(self.impl_mut(), alpha))
        }
    }
}

/// Clip Operations
impl Context {
    #[inline]
    pub fn restore_clipping(&mut self) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextRestoreClipping(self.core_mut())) }
    }

    #[inline]
    pub fn clip_to_rect<R: Rect>(&mut self, rect: &R) -> Result<()> {
        unsafe {
            errcode_to_result((R::CLIP_TO_RECT)(
                self.core_mut(),
                rect as *const _ as *const _,
            ))
        }
    }

    #[inline]
    pub fn clip_to(&mut self, x: f64, y: f64, w: f64, h: f64) -> Result<()> {
        self.clip_to_rect(&RectD { x, y, w, h })
    }
}

/// Clear Operations
impl Context {
    #[inline]
    pub fn clear_all(&mut self) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextClearAll(self.core_mut())) }
    }

    #[inline]
    pub fn clear_rect<R: Rect>(&mut self, rect: &R) -> Result<()> {
        unsafe {
            errcode_to_result((R::CLEAR_RECT)(
                self.core_mut(),
                rect as *const _ as *const _,
            ))
        }
    }

    #[inline]
    pub fn clear(&mut self, x: f64, y: f64, w: f64, h: f64) -> Result<()> {
        self.clear_rect(&RectD { x, y, w, h })
    }

    #[inline]
    pub fn blit_image<P: Point>(
        &mut self,
        dst: &P,
        src: &Image,
        src_area: Option<&RectI>,
    ) -> Result<()> {
        unsafe {
            errcode_to_result(P::BLIT_IMAGE(
                self.core_mut(),
                dst as *const _ as *const _,
                src.core(),
                src_area.map_or(ptr::null(), |r| r as *const _ as *const _),
            ))
        }
    }

    #[inline]
    pub fn blit_scaled_image<R: Rect>(
        &mut self,
        dst: &R,
        src: &Image,
        src_area: Option<&RectI>,
    ) -> Result<()> {
        unsafe {
            errcode_to_result(R::BLIT_SCALED_IMAGE(
                self.core_mut(),
                dst as *const _ as *const _,
                src.core(),
                src_area.map_or(ptr::null(), |r| r as *const _ as *const _),
            ))
        }
    }
}

/// Fill Operations
impl Context {
    #[inline]
    pub fn fill_geometry<T: Geometry + ?Sized>(&mut self, geo: &T) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blContextFillGeometry(
                self.core_mut(),
                T::GEO_TYPE,
                geo as *const _ as *const _,
            ))
        }
    }

    #[inline]
    pub fn fill_all(&mut self) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextFillAll(self.core_mut())) }
    }

    #[inline]
    pub fn fill_box(&mut self, x0: f64, y0: f64, x1: f64, y1: f64) -> Result<()> {
        self.fill_geometry(&BoxD { x0, y0, x1, y1 })
    }

    #[inline]
    pub fn fill_rect(&mut self, x: f64, y: f64, w: f64, h: f64) -> Result<()> {
        self.fill_geometry(&RectD { x, y, w, h })
    }

    #[inline]
    pub fn fill_circle(&mut self, cx: f64, cy: f64, r: f64) -> Result<()> {
        self.fill_geometry(&Circle { cx, cy, radius: r })
    }

    #[inline]
    pub fn fill_ellipse(&mut self, cx: f64, cy: f64, rx: f64, ry: f64) -> Result<()> {
        self.fill_geometry(&Ellipse { cx, cy, rx, ry })
    }

    #[inline]
    #[rustfmt::skip]
    pub fn fill_round_rect(&mut self, x: f64, y: f64, w: f64, h: f64, rx: f64, ry: f64) -> Result<()> {
        self.fill_geometry(&RoundRect { x, y, w, h, rx, ry })
    }

    #[inline]
    #[rustfmt::skip]
    pub fn fill_arc(&mut self, cx: f64, cy: f64, rx: f64, ry: f64, start: f64, sweep: f64) -> Result<()> {
        self.fill_geometry(&Arc { cx, cy, rx, ry, start, sweep })
    }

    #[inline]
    #[rustfmt::skip]
    pub fn fill_chord(&mut self, cx: f64, cy: f64, rx: f64, ry: f64, start: f64, sweep: f64) -> Result<()> {
        self.fill_geometry(&Chord { cx, cy, rx, ry, start, sweep })
    }

    #[inline]
    #[rustfmt::skip]
    pub fn fill_pie(&mut self, cx: f64, cy: f64, rx: f64, ry: f64, start: f64, sweep: f64) -> Result<()> {
        self.fill_geometry(&Pie { cx, cy, rx, ry, start, sweep })
    }

    #[inline]
    #[rustfmt::skip]
    pub fn fill_triangle(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> Result<()> {
        self.fill_geometry(&Triangle { x0, y0, x1, y1, x2, y2 })
    }

    #[inline]
    pub fn fill_path(&mut self, p: &Path) -> Result<()> {
        self.fill_geometry(p)
    }

    #[inline]
    pub fn fill_polygon<R, P>(&mut self, poly: R) -> Result<()>
    where
        [P]: Geometry,
        R: AsRef<[P]>,
        P: Point,
    {
        self.fill_geometry(poly.as_ref())
    }

    #[inline]
    pub fn fill_slice<R, P>(&mut self, slice: R) -> Result<()>
    where
        [P]: Geometry,
        R: AsRef<[P]>,
        P: GeoViewArray,
    {
        self.fill_geometry(slice.as_ref())
    }
}

/// Stroke Operations
impl Context {
    #[inline]
    pub fn stroke_geometry<T: Geometry + ?Sized>(&mut self, geo: &T) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blContextStrokeGeometry(
                self.core_mut(),
                T::GEO_TYPE,
                geo as *const _ as *const _,
            ))
        }
    }

    #[inline]
    pub fn stroke_box(&mut self, x0: f64, y0: f64, x1: f64, y1: f64) -> Result<()> {
        self.stroke_geometry(&BoxD { x0, y0, x1, y1 })
    }

    #[inline]
    pub fn stroke_rect(&mut self, x: f64, y: f64, w: f64, h: f64) -> Result<()> {
        self.stroke_geometry(&RectD { x, y, w, h })
    }

    #[inline]
    pub fn stroke_line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64) -> Result<()> {
        self.stroke_geometry(&Line { x0, y0, x1, y1 })
    }

    #[inline]
    pub fn stroke_circle(&mut self, cx: f64, cy: f64, r: f64) -> Result<()> {
        self.stroke_geometry(&Circle { cx, cy, radius: r })
    }

    #[inline]
    pub fn stroke_ellipse(&mut self, cx: f64, cy: f64, rx: f64, ry: f64) -> Result<()> {
        self.stroke_geometry(&Ellipse { cx, cy, rx, ry })
    }

    #[inline]
    #[rustfmt::skip]
    pub fn stroke_round_rect(&mut self, x: f64, y: f64, w: f64, h: f64, rx: f64, ry: f64) -> Result<()> {
        self.stroke_geometry(&RoundRect { x, y, w, h, rx, ry })
    }

    #[inline]
    #[rustfmt::skip]
    pub fn stroke_arc(&mut self, cx: f64, cy: f64, rx: f64, ry: f64, start: f64, sweep: f64) -> Result<()> {
        self.stroke_geometry(&Arc { cx, cy, rx, ry, start, sweep })
    }

    #[inline]
    #[rustfmt::skip]
    pub fn stroke_chord(&mut self, cx: f64, cy: f64, rx: f64, ry: f64, start: f64, sweep: f64) -> Result<()> {
        self.stroke_geometry(&Chord { cx, cy, rx, ry, start, sweep })
    }

    #[inline]
    #[rustfmt::skip]
    pub fn stroke_pie(&mut self, cx: f64, cy: f64, rx: f64, ry: f64, start: f64, sweep: f64) -> Result<()> {
        self.stroke_geometry(&Pie { cx, cy, rx, ry, start, sweep })
    }

    #[inline]
    #[rustfmt::skip]
    pub fn stroke_triangle(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> Result<()> {
        self.stroke_geometry(&Triangle { x0, y0, x1, y1, x2, y2 })
    }

    #[inline]
    pub fn stroke_path(&mut self, p: &Path) -> Result<()> {
        self.stroke_geometry(p)
    }

    #[inline]
    pub fn stroke_polygon<R, P>(&mut self, poly: R) -> Result<()>
    where
        [P]: Geometry,
        R: AsRef<[P]>,
        P: Point,
    {
        self.fill_geometry(poly.as_ref())
    }

    #[inline]
    pub fn stroke_polyline<R, P>(&mut self, poly: R) -> Result<()>
    where
        [P]: Geometry,
        R: AsRef<[P]>,
        P: Point,
    {
        unsafe {
            errcode_to_result(ffi::blContextStrokeGeometry(
                self.core_mut(),
                P::POLYLINE_TYPE,
                poly.as_ref().as_ptr() as *const _,
            ))
        }
    }

    #[inline]
    pub fn stroke_slice<R, P>(&mut self, slice: R) -> Result<()>
    where
        [P]: Geometry,
        R: AsRef<[P]>,
        P: GeoViewArray,
    {
        self.stroke_geometry(slice.as_ref())
    }
}

impl MatrixTransform for Context {
    #[inline]
    #[doc(hidden)]
    fn apply_matrix_op(&mut self, op: Matrix2DOp, data: &[f64]) -> Result<()> {
        unsafe {
            errcode_to_result((self.impl_().virt().matrixOp.unwrap())(
                self.impl_mut(),
                op as u32,
                data.as_ptr() as *const _,
            ))
        }
    }
}

impl PartialEq for Context {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.impl_() as *const _ == other.impl_() as *const _
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { ffi::blContextReset(&mut self.core) };
    }
}
