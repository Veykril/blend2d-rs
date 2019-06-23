use core::{fmt, ptr};

use crate::{
    error::{errcode_to_result, expect_mem_err, Result},
    geometry::RectI,
    image::Image,
    matrix::{Matrix2D, Matrix2DOp, MatrixTransform},
    variant::WrappedBlCore,
    ExtendMode,
};

#[repr(transparent)]
pub struct Pattern {
    core: ffi::BLPatternCore,
}

unsafe impl WrappedBlCore for Pattern {
    type Core = ffi::BLPatternCore;
    const IMPL_TYPE_INDEX: usize = crate::variant::ImplType::Pattern as usize;

    #[inline]
    fn from_core(core: Self::Core) -> Self {
        Pattern { core }
    }
}

impl Pattern {
    /// Creates a new pattern that borrows the given [`Image`] immutably for its
    /// lifetime.
    #[inline]
    pub fn new(
        image: &Image,
        area: Option<&RectI>,
        extend_mode: ExtendMode,
        matrix: Option<&Matrix2D>,
    ) -> Pattern {
        let mut this = Pattern::from_core(*Self::none());
        unsafe {
            ffi::blPatternInitAs(
                this.core_mut(),
                image.core(),
                area.map_or(ptr::null(), |a| a as *const _ as *const _),
                extend_mode.into(),
                matrix.map_or(ptr::null(), |a| a as *const _ as *const _),
            );
        }
        this
    }

    /// The pattern's [`Image`].
    #[inline]
    pub fn image(&self) -> &Image {
        unsafe { &*(&self.impl_().image as *const _ as *const _) }
    }

    /// Returns this pattern with a new [`Image`].
    ///
    /// There is no set function because such a function would not be able to
    /// change the pattern's lifetime.
    #[inline]
    pub fn with_new_image(mut self, image: &Image) -> Result<Pattern> {
        unsafe {
            errcode_to_result(ffi::blPatternSetImage(
                self.core_mut(),
                image.core(),
                ptr::null(),
            ))
            .map(|_| Pattern { core: self.core })
        }
    }

    /// Returns this pattern with a new clipped [`Image`].
    #[inline]
    pub fn with_new_image_clipped(mut self, image: &Image, area: &RectI) -> Result<Pattern> {
        unsafe {
            errcode_to_result(ffi::blPatternSetImage(
                self.core_mut(),
                image.core(),
                area as *const _ as *const _,
            ))
            .map(|_| Pattern { core: self.core })
        }
    }

    /// The clipping area.
    #[inline]
    pub fn area(&self) -> &RectI {
        unsafe { &*(&self.impl_().area as *const _ as *const _) }
    }

    /// Sets the clipping area.
    #[inline]
    pub fn set_area(&mut self, area: &RectI) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPatternSetArea(
                self.core_mut(),
                area as *const _ as *const _,
            ))
        }
    }

    /// Resets the clipping area to zero.
    #[inline]
    pub fn reset_area(&mut self) {
        let _ = self.set_area(&RectI::default());
    }

    /// The pattern's [`ExtendMode`].
    #[inline]
    pub fn extend_mode(&self) -> ExtendMode {
        (self.impl_().extendMode as u32).into()
    }

    /// Sets the pattern's [`ExtendMode`].
    #[inline]
    pub fn set_extend_mode(&mut self, mode: ExtendMode) {
        unsafe { expect_mem_err(ffi::blPatternSetExtendMode(self.core_mut(), mode.into())) };
    }

    /// Resets the pattern's [`ExtendMode`] to the default.
    #[inline]
    pub fn reset_extend_mode(&mut self) {
        self.set_extend_mode(Default::default());
    }

    /// SThe pattern's [`Matrix2D`]
    #[inline]
    pub fn matrix(&self) -> &Matrix2D {
        unsafe { &*(&self.impl_().matrix as *const _ as *const _) }
    }
}

impl MatrixTransform for Pattern {
    #[inline]
    #[doc(hidden)]
    fn apply_matrix_op(&mut self, op: Matrix2DOp, data: &[f64]) {
        unsafe {
            expect_mem_err(ffi::blPatternApplyMatrixOp(
                self.core_mut(),
                op as u32,
                data.as_ptr() as *const _,
            ))
        };
    }
}

impl From<&Image> for Pattern {
    #[inline]
    fn from(image: &Image) -> Self {
        Self::new(image, None, Default::default(), None)
    }
}

impl PartialEq for Pattern {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::blPatternEquals(self.core(), other.core()) }
    }
}

impl fmt::Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Pattern")
            .field("image", self.image())
            .field("area", self.area())
            .field("extend_mode", &self.extend_mode())
            .field("matrix", self.matrix())
            .finish()
    }
}

impl Clone for Pattern {
    fn clone(&self) -> Self {
        Self::from_core(self.init_weak())
    }
}

impl Drop for Pattern {
    fn drop(&mut self) {
        unsafe { ffi::blPatternReset(&mut self.core) };
    }
}
