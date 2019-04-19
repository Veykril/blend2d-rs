use core::{fmt, marker::PhantomData, ptr};

use crate::{
    error::{errcode_to_result, Result},
    geometry::RectI,
    image::Image,
    matrix::{Matrix2D, Matrix2DOp, MatrixTransform},
    variant::WrappedBlCore,
    ExtendMode,
};

#[repr(transparent)]
pub struct Pattern<'a> {
    core: ffi::BLPatternCore,
    _pd: PhantomData<&'a Image<'a>>,
}

unsafe impl WrappedBlCore for Pattern<'_> {
    type Core = ffi::BLPatternCore;
    const IMPL_TYPE_INDEX: usize = crate::variant::ImplType::Pattern as usize;

    fn from_core(core: Self::Core) -> Self {
        Pattern {
            core,
            _pd: PhantomData,
        }
    }
}

impl Pattern<'_> {
    #[inline]
    pub fn new(
        image: &Image<'_>,
        area: Option<&RectI>,
        extend_mode: ExtendMode,
        matrix: Option<&Matrix2D>,
    ) -> Self {
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

    #[inline]
    pub fn image<'a>(&'a self) -> &Image<'a> {
        unsafe { &*(&self.impl_().image as *const _ as *const _) }
    }

    #[inline]
    pub fn set_image(&mut self, image: &Image<'_>) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blPatternSetImage(
                self.core_mut(),
                image.core(),
                ptr::null(),
            ))
        }
    }

    #[inline]
    pub fn set_image_clipped(&mut self, image: &Image<'_>, area: &RectI) -> Result<()> {
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
    pub fn matrix(&self) -> &Matrix2D {
        unsafe { &*(&self.impl_().matrix as *const _ as *const _) }
    }
}

impl MatrixTransform for Pattern<'_> {
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

impl PartialEq for Pattern<'_> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::blPatternEquals(self.core(), other.core()) }
    }
}

impl fmt::Debug for Pattern<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Pattern")
            .field("image", self.image())
            .field("area", self.area())
            .field("extend_mode", &self.extend_mode())
            .field("matrix", self.matrix())
            .finish()
    }
}

impl Clone for Pattern<'_> {
    fn clone(&self) -> Self {
        let mut new = Pattern::from_core(*Self::none());
        unsafe { ffi::blPatternAssignDeep(new.core_mut(), self.core()) };
        new
    }
}

impl Drop for Pattern<'_> {
    fn drop(&mut self) {
        unsafe { ffi::blPatternReset(&mut self.core) };
    }
}
