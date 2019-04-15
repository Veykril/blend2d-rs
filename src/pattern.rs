use core::ptr;

use crate::{
    error::{errcode_to_result, Result},
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
    const IMPL_TYPE_INDEX: usize = ffi::BLImplType::BL_IMPL_TYPE_PATTERN as usize;
}

impl Pattern {
    #[inline]
    pub fn new(
        image: &Image,
        area: Option<&RectI>,
        extend_mode: ExtendMode,
        matrix: Option<&Matrix2D>,
    ) -> Self {
        let mut this = Pattern {
            core: *Self::none(),
        };
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

    #[inline]
    pub fn image(&self) -> &Image {
        unsafe { &*(&self.impl_().image as *const _ as *const _) }
    }

    #[inline]
    pub fn set_image(&mut self, image: &Image) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPatternSetImage(
                self.core_mut(),
                image.core(),
                ptr::null(),
            ))
        }
    }

    #[inline]
    pub fn set_image_clipped(&mut self, image: &Image, area: &RectI) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPatternSetImage(
                self.core_mut(),
                image.core(),
                area as *const _ as *const _,
            ))
        }
    }

    #[inline]
    pub fn reset_image(&mut self) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPatternSetImage(
                self.core_mut(),
                Image::none(),
                ptr::null(),
            ))
        }
    }

    #[inline]
    pub fn area(&self) -> &RectI {
        unsafe { &*(&self.impl_().area as *const _ as *const _) }
    }

    #[inline]
    pub fn set_area(&mut self, area: &RectI) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPatternSetArea(
                self.core_mut(),
                area as *const _ as *const _,
            ))
        }
    }

    #[inline]
    pub fn reset_area(&mut self) -> Result<()> {
        self.set_area(&RectI::default())
    }

    #[inline]
    pub fn extend_mode(&self) -> ExtendMode {
        (self.impl_().extendMode as u32).into()
    }

    #[inline]
    pub fn set_extend_mode(&mut self, mode: ExtendMode) -> Result<()> {
        unsafe { errcode_to_result(ffi::blPatternSetExtendMode(self.core_mut(), mode.into())) }
    }

    #[inline]
    pub fn reset_extend_mode(&mut self) -> Result<()> {
        self.set_extend_mode(Default::default())
    }

    #[inline]
    pub fn has_matrix(&self) -> bool {
        self.impl_().matrixType as i32 != ffi::BLMatrix2DType::BL_MATRIX2D_TYPE_IDENTITY
    }

    #[inline]
    pub fn matrix(&self) -> Option<&Matrix2D> {
        if self.has_matrix() {
            unsafe { Some(&*(&self.impl_().matrix as *const _ as *const _)) }
        } else {
            None
        }
    }
}

impl MatrixTransform for Pattern {
    #[inline]
    #[doc(hidden)]
    fn apply_matrix_op(&mut self, op: Matrix2DOp, data: &[f64]) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPatternApplyMatrixOp(
                self.core_mut(),
                op as u32,
                data.as_ptr() as *const _,
            ))
        }
    }
}

impl PartialEq for Pattern {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::blPatternEquals(self.core(), other.core()) }
    }
}

impl Clone for Pattern {
    fn clone(&self) -> Self {
        let mut new = Pattern {
            core: *Self::none(),
        };
        unsafe { ffi::blPatternAssignDeep(new.core_mut(), self.core()) };
        new
    }
}

impl Drop for Pattern {
    fn drop(&mut self) {
        unsafe { ffi::blPatternReset(&mut self.core) };
    }
}
