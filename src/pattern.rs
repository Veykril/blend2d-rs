use core::ptr;

use crate::{
    error::{errcode_to_result, Result},
    geometry::RectI,
    image::Image,
    variant::WrappedBlCore,
    ExtendMode, Matrix2D,
};

#[repr(transparent)]
pub struct Pattern {
    core: ffi::BLPatternCore,
}

unsafe impl WrappedBlCore for Pattern {
    type Core = ffi::BLPatternCore;
}

impl Pattern {
    #[inline]
    pub fn new() -> Self {
        Pattern {
            core: *Self::none(ffi::BLImplType::BL_IMPL_TYPE_PATTERN as usize),
        }
    }

    #[inline]
    pub fn new_with(
        image: &Image,
        area: Option<&RectI>,
        extend_mode: ExtendMode,
        matrix: Option<&Matrix2D>,
    ) -> Self {
        let mut this = Self::new();
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
    pub fn create(
        &mut self,
        image: &Image,
        area: Option<&RectI>,
        extend_mode: ExtendMode,
        matrix: Option<&Matrix2D>,
    ) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPatternCreate(
                self.core_mut(),
                image.core(),
                area.map_or(ptr::null(), |a| a as *const _ as *const _),
                extend_mode.into(),
                matrix.map_or(ptr::null(), |a| a as *const _ as *const _),
            ))
        }
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
        self.set_image(&Image::new())
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
}

impl Pattern {
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

impl Default for Pattern {
    #[inline]
    fn default() -> Self {
        Self::new()
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
        Pattern {
            core: self.init_weak(),
        }
    }
}

impl Drop for Pattern {
    fn drop(&mut self) {
        unsafe { ffi::blPatternReset(&mut self.core) };
    }
}
