use core::ptr;

use ffi::BLContextCore;

use crate::{
    error::{errcode_to_result, Result},
    image::Image,
    path::Path,
    ImplType,
};

#[repr(i32)]
pub enum CompOp {
    // Source-over [default].
    SrcOver = ffi::BLCompOp::BL_COMP_OP_SRC_OVER,
    // Source-copy.
    SrcCopy = ffi::BLCompOp::BL_COMP_OP_SRC_COPY,
    // Source-in.
    SrcIn = ffi::BLCompOp::BL_COMP_OP_SRC_IN,
    // Source-out.
    SrcOut = ffi::BLCompOp::BL_COMP_OP_SRC_OUT,
    // Source-atop.
    SrcAtop = ffi::BLCompOp::BL_COMP_OP_SRC_ATOP,
    // Destination-over.
    DstOver = ffi::BLCompOp::BL_COMP_OP_DST_OVER,
    // Destination-copy [nop].
    DstCopy = ffi::BLCompOp::BL_COMP_OP_DST_COPY,
    // Destination-in.
    DstIn = ffi::BLCompOp::BL_COMP_OP_DST_IN,
    // Destination-out.
    DstOut = ffi::BLCompOp::BL_COMP_OP_DST_OUT,
    // Destination-atop.
    DstAtop = ffi::BLCompOp::BL_COMP_OP_DST_ATOP,
    // Xor.
    Xor = ffi::BLCompOp::BL_COMP_OP_XOR,
    // Clear.
    Clear = ffi::BLCompOp::BL_COMP_OP_CLEAR,
    // Plus.
    Plus = ffi::BLCompOp::BL_COMP_OP_PLUS,
    // Minus.
    Minus = ffi::BLCompOp::BL_COMP_OP_MINUS,
    // Multiply.
    Multiply = ffi::BLCompOp::BL_COMP_OP_MULTIPLY,
    // Screen.
    Screen = ffi::BLCompOp::BL_COMP_OP_SCREEN,
    // Overlay.
    Overlay = ffi::BLCompOp::BL_COMP_OP_OVERLAY,
    // Darken.
    Darken = ffi::BLCompOp::BL_COMP_OP_DARKEN,
    // Lighten.
    Lighten = ffi::BLCompOp::BL_COMP_OP_LIGHTEN,
    // Color dodge.
    ColorDodge = ffi::BLCompOp::BL_COMP_OP_COLOR_DODGE,
    // Color burn.
    ColorBurn = ffi::BLCompOp::BL_COMP_OP_COLOR_BURN,
    // Linear burn.
    LinearBurn = ffi::BLCompOp::BL_COMP_OP_LINEAR_BURN,
    // Linear light.
    LinearLight = ffi::BLCompOp::BL_COMP_OP_LINEAR_LIGHT,
    // Pin light.
    PinLight = ffi::BLCompOp::BL_COMP_OP_PIN_LIGHT,
    // Hard-light.
    HardLight = ffi::BLCompOp::BL_COMP_OP_HARD_LIGHT,
    // Soft-light.
    SoftLight = ffi::BLCompOp::BL_COMP_OP_SOFT_LIGHT,
    // Difference.
    Difference = ffi::BLCompOp::BL_COMP_OP_DIFFERENCE,
    // Exclusion.
    Exclusion = ffi::BLCompOp::BL_COMP_OP_EXCLUSION,
}

impl From<u32> for CompOp {
    fn from(val: u32) -> CompOp {
        match val as ffi::BLCompOp::Type {
            ffi::BLCompOp::BL_COMP_OP_SRC_COPY => CompOp::SrcCopy,
            ffi::BLCompOp::BL_COMP_OP_SRC_IN => CompOp::SrcIn,
            ffi::BLCompOp::BL_COMP_OP_SRC_OUT => CompOp::SrcOut,
            ffi::BLCompOp::BL_COMP_OP_SRC_ATOP => CompOp::SrcAtop,
            ffi::BLCompOp::BL_COMP_OP_DST_OVER => CompOp::DstOver,
            ffi::BLCompOp::BL_COMP_OP_DST_COPY => CompOp::DstCopy,
            ffi::BLCompOp::BL_COMP_OP_DST_IN => CompOp::DstIn,
            ffi::BLCompOp::BL_COMP_OP_DST_OUT => CompOp::DstOut,
            ffi::BLCompOp::BL_COMP_OP_DST_ATOP => CompOp::DstAtop,
            ffi::BLCompOp::BL_COMP_OP_XOR => CompOp::Xor,
            ffi::BLCompOp::BL_COMP_OP_CLEAR => CompOp::Clear,
            ffi::BLCompOp::BL_COMP_OP_PLUS => CompOp::Plus,
            ffi::BLCompOp::BL_COMP_OP_MINUS => CompOp::Minus,
            ffi::BLCompOp::BL_COMP_OP_MULTIPLY => CompOp::Multiply,
            ffi::BLCompOp::BL_COMP_OP_SCREEN => CompOp::Screen,
            ffi::BLCompOp::BL_COMP_OP_OVERLAY => CompOp::Overlay,
            ffi::BLCompOp::BL_COMP_OP_DARKEN => CompOp::Darken,
            ffi::BLCompOp::BL_COMP_OP_LIGHTEN => CompOp::Lighten,
            ffi::BLCompOp::BL_COMP_OP_COLOR_DODGE => CompOp::ColorDodge,
            ffi::BLCompOp::BL_COMP_OP_COLOR_BURN => CompOp::ColorBurn,
            ffi::BLCompOp::BL_COMP_OP_LINEAR_BURN => CompOp::LinearBurn,
            ffi::BLCompOp::BL_COMP_OP_LINEAR_LIGHT => CompOp::LinearLight,
            ffi::BLCompOp::BL_COMP_OP_PIN_LIGHT => CompOp::PinLight,
            ffi::BLCompOp::BL_COMP_OP_HARD_LIGHT => CompOp::HardLight,
            ffi::BLCompOp::BL_COMP_OP_SOFT_LIGHT => CompOp::SoftLight,
            ffi::BLCompOp::BL_COMP_OP_DIFFERENCE => CompOp::Difference,
            ffi::BLCompOp::BL_COMP_OP_EXCLUSION => CompOp::Exclusion,
            _ => CompOp::SrcOver,
        }
    }
}
impl From<CompOp> for u32 {
    fn from(val: CompOp) -> u32 {
        val as u32
    }
}

impl Default for CompOp {
    fn default() -> Self {
        CompOp::SrcOver
    }
}

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

    pub fn new_from_image(target: &mut Image) -> Result<Context> {
        unsafe {
            let mut core = std::mem::uninitialized();

            errcode_to_result(ffi::blContextInitAs(
                &mut core,
                &mut target.core,
                ptr::null(),
            ))
            .map(|_| Context { core })
        }
    }

    pub fn new_from_image_with_options(
        target: &mut Image,
        options: Option<ffi::BLContextCreateOptions>,
    ) -> Result<Context> {
        unsafe {
            let mut core = std::mem::uninitialized();

            errcode_to_result(ffi::blContextInitAs(
                &mut core,
                &mut target.core,
                options.as_ref().map_or(ptr::null(), |ptr| ptr as *const _),
            ))
            .map(move |_| Context { core })
        }
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
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl ImplType for Context {
    type Type = ffi::BLContextCore;
    const IDX: usize = ffi::BLImplType::BL_IMPL_TYPE_CONTEXT as usize;
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            ffi::blContextEnd(&mut self.core);
            ffi::blContextReset(&mut self.core);
        }
    }
}
