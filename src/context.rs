use bitflags::bitflags;

use core::ptr;

use ffi::BLContextCore;

use crate::{
    error::{errcode_to_result, Result},
    geometry::Path,
    image::Image,
    ImplType,
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

use ffi::BLRenderingQuality::*;
bl_enum! {
    pub enum RenderingQuality {
        AntiAliasing = BL_RENDERING_QUALITY_ANTIALIAS,
    }
    Default => AntiAliasing
}

#[derive(PartialOrd, PartialEq)]
pub struct ContextCookie(u128);

#[repr(transparent)]
pub struct Context {
    core: BLContextCore,
}

impl Context {
    pub fn new() -> Self {
        Context {
            core: ffi::BLContextCore {
                impl_: Self::none().impl_,
            },
        }
    }

    #[inline]
    pub fn from_image(target: &mut Image) -> Result<Context> {
        Self::from_image_with_options(target, None)
    }

    pub fn from_image_with_options(
        target: &mut Image,
        options: Option<ffi::BLContextCreateOptions>,
    ) -> Result<Context> {
        unsafe {
            let mut core = ffi::BLContextCore {
                impl_: ptr::null_mut(),
            };

            errcode_to_result(ffi::blContextInitAs(
                &mut core,
                &mut target.core,
                options.as_ref().map_or(ptr::null(), |ptr| ptr as *const _),
            ))
            .map(move |_| Context { core })
        }
    }

    #[inline]
    pub fn target_size(&self) -> (f64, f64) {
        let ffi::BLSize { w, h } = self.impl_().targetSize;
        (w, h)
    }

    #[inline]
    pub fn target_width(&self) -> f64 {
        self.target_size().0
    }

    #[inline]
    pub fn target_height(&self) -> f64 {
        self.target_size().1
    }

    #[inline]
    pub fn context_type(&self) -> ContextType {
        self.impl_().contextType.into()
    }

    #[inline]
    pub fn reset(&mut self) {
        unsafe { ffi::blContextReset(&mut self.core) };
    }

    #[inline]
    pub fn end(&mut self) {
        unsafe { ffi::blContextEnd(&mut self.core) };
    }

    pub fn flush(&mut self, flags: ContextFlushFlags) {
        unsafe { ffi::blContextFlush(&mut self.core, flags.bits()) };
    }

    pub fn set_comp_op(&mut self, comp_op: CompOp) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextSetCompOp(&mut self.core, comp_op.into())) }
    }

    pub fn set_fill_style_rgba32(&mut self, color: u32) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextSetFillStyleRgba32(&mut self.core, color)) }
    }

    pub fn fill_all(&mut self) -> Result<()> {
        unsafe { errcode_to_result(ffi::blContextFillAll(&mut self.core)) }
    }

    pub fn fill_path(&mut self, path: &Path) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blContextFillGeometry(
                &mut self.core,
                ffi::BLGeometryType::BL_GEOMETRY_TYPE_PATH as u32,
                &path.core as *const _ as *const _,
            ))
        }
    }

    fn impl_(&self) -> &ffi::BLContextImpl {
        unsafe { &*self.core.impl_ }
    }
}

impl PartialEq for Context {
    fn eq(&self, other: &Self) -> bool {
        self.impl_() as *const _ == other.impl_() as *const _
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        let mut core = ffi::BLContextCore {
            impl_: ptr::null_mut(),
        };
        unsafe {
            ffi::blVariantInitWeak(
                &mut core as *mut _ as *mut _,
                &self.core as *const _ as *const _,
            )
        };
        Context { core }
    }
}

impl ImplType for Context {
    type CoreType = ffi::BLContextCore;
    const IMPL_TYPE_ID: usize = ffi::BLImplType::BL_IMPL_TYPE_CONTEXT as usize;
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            ffi::blContextReset(&mut self.core);
        }
    }
}
