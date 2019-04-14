use core::{fmt, marker::PhantomData, mem, ptr};

use ffi::BLGradientValue::*;

use crate::{
    error::{errcode_to_result, Result},
    variant::WrappedBlCore,
    ExtendMode, Matrix2D,
};

mod private {
    pub trait Sealed {}
    impl Sealed for super::Linear {}
    impl Sealed for super::Radial {}
    impl Sealed for super::Conical {}
}

pub trait GradientType: private::Sealed {
    #[doc(hidden)]
    type ValuesType;
    #[doc(hidden)]
    const BL_TYPE: u32;
}

pub enum Linear {}
impl GradientType for Linear {
    #[doc(hidden)]
    type ValuesType = LinearGradientValues;
    #[doc(hidden)]
    const BL_TYPE: u32 = ffi::BLGradientType::BL_GRADIENT_TYPE_LINEAR as u32;
}
pub enum Radial {}
impl GradientType for Radial {
    #[doc(hidden)]
    type ValuesType = RadialGradientValues;
    #[doc(hidden)]
    const BL_TYPE: u32 = ffi::BLGradientType::BL_GRADIENT_TYPE_RADIAL as u32;
}
pub enum Conical {}
impl GradientType for Conical {
    #[doc(hidden)]
    type ValuesType = ConicalGradientValues;
    #[doc(hidden)]
    const BL_TYPE: u32 = ffi::BLGradientType::BL_GRADIENT_TYPE_CONICAL as u32;
}

#[derive(Copy, Clone, Default, PartialEq)]
pub struct GradientStop {
    pub offset: f64,
    pub rgba: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq)]
pub struct LinearGradientValues {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq)]
pub struct RadialGradientValues {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    pub r0: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq)]
pub struct ConicalGradientValues {
    pub x0: f64,
    pub y0: f64,
    pub angle: f64,
}

pub enum DynamicGradient {
    Linear(LinearGradient),
    Radial(RadialGradient),
    Conical(ConicalGradient),
}

impl From<LinearGradient> for DynamicGradient {
    fn from(g: LinearGradient) -> Self {
        DynamicGradient::Linear(g)
    }
}

impl From<ConicalGradient> for DynamicGradient {
    fn from(g: ConicalGradient) -> Self {
        DynamicGradient::Conical(g)
    }
}

impl From<RadialGradient> for DynamicGradient {
    fn from(g: RadialGradient) -> Self {
        DynamicGradient::Radial(g)
    }
}

pub type LinearGradient = Gradient<Linear>;
pub type RadialGradient = Gradient<Radial>;
pub type ConicalGradient = Gradient<Conical>;

#[repr(transparent)]
pub struct Gradient<T: GradientType> {
    core: ffi::BLGradientCore,
    _pd: PhantomData<*const T>,
}

unsafe impl<T: GradientType> WrappedBlCore for Gradient<T> {
    type Core = ffi::BLGradientCore;
}

impl<T: GradientType> Gradient<T> {
    #[inline]
    pub fn new() -> Self {
        Gradient {
            core: *Self::none(ffi::BLImplType::BL_IMPL_TYPE_GRADIENT as usize),
            _pd: PhantomData,
        }
    }

    pub fn new_with(
        values: &T::ValuesType,
        extend_mode: ExtendMode,
        stops: &[GradientStop],
        m: Option<&Matrix2D>,
    ) -> Self {
        let mut core = ffi::BLGradientCore {
            impl_: ptr::null_mut(),
        };
        unsafe {
            ffi::blGradientInitAs(
                &mut core,
                T::BL_TYPE,
                values as *const _ as *const _,
                extend_mode as u32,
                stops.as_ptr() as *const _ as *const _,
                stops.len(),
                m.map_or(ptr::null_mut(), |m| m as *const _ as *const _),
            )
        };
        Gradient {
            core,
            _pd: PhantomData,
        }
    }

    pub fn create<U: GradientType>(
        mut self,
        values: &U::ValuesType,
        extend_mode: ExtendMode,
        stops: &[GradientStop],
        m: Option<&Matrix2D>,
    ) -> Gradient<U> {
        unsafe {
            ffi::blGradientCreate(
                self.core_mut(),
                U::BL_TYPE,
                values as *const _ as *const _,
                extend_mode as u32,
                stops.as_ptr() as *const _ as *const _,
                stops.len(),
                m.map_or(ptr::null_mut(), |m| m as *const _ as *const _),
            )
        };
        Gradient {
            core: self.core,
            _pd: PhantomData,
        }
    }

    #[inline]
    pub(in crate) unsafe fn with_type<U: GradientType>(self) -> Gradient<U> {
        Gradient {
            core: self.core,
            _pd: PhantomData,
        }
    }

    #[inline]
    pub fn extend_mode(&self) -> ExtendMode {
        (self.impl_().extendMode as u32).into()
    }

    #[inline]
    pub fn set_extend_mode(&mut self, mode: ExtendMode) {
        unsafe { ffi::blGradientSetExtendMode(self.core_mut(), mode as u32) };
    }

    #[inline]
    pub fn value(&self, index: usize) -> f64 {
        unsafe { self.impl_().__bindgen_anon_2.values[index] }
    }

    #[inline]
    pub fn set_value(&mut self, index: usize, value: f64) {
        assert!(index < ffi::BLGradientValue::BL_GRADIENT_VALUE_COUNT as usize);
        unsafe { ffi::blGradientSetValue(self.core_mut(), index, value) };
    }

    #[inline]
    pub fn values(&self) -> &T::ValuesType {
        unsafe { &*(self.impl_().__bindgen_anon_2.values.as_ptr() as *const _ as *const _) }
    }

    #[inline]
    pub fn set_values(&mut self, values: &T::ValuesType) {
        unsafe {
            ffi::blGradientSetValues(
                self.core_mut(),
                0,
                values as *const _ as *const _,
                mem::size_of::<T::ValuesType>() / mem::size_of::<f64>(),
            )
        };
    }

    #[inline]
    pub fn set_values_from_slice(&mut self, index: usize, values: &[f64]) {
        unsafe { ffi::blGradientSetValues(self.core_mut(), index, values.as_ptr(), values.len()) };
    }

    #[inline]
    pub fn x0(&self) -> f64 {
        self.value(BL_GRADIENT_VALUE_COMMON_X0 as usize)
    }

    #[inline]
    pub fn y0(&self) -> f64 {
        self.value(BL_GRADIENT_VALUE_COMMON_Y0 as usize)
    }

    #[inline]
    pub fn set_x0(&mut self, val: f64) {
        self.set_value(BL_GRADIENT_VALUE_COMMON_X0 as usize, val)
    }

    #[inline]
    pub fn set_y0(&mut self, val: f64) {
        self.set_value(BL_GRADIENT_VALUE_COMMON_Y0 as usize, val)
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

impl<T: GradientType> Gradient<T> {
    #[inline]
    pub fn add_stop32(&mut self, offset: f64, rgba: u32) -> Result<()> {
        unsafe { errcode_to_result(ffi::blGradientAddStopRgba32(self.core_mut(), offset, rgba)) }
    }

    #[inline]
    pub fn add_stop64(&mut self, offset: f64, rgba: u64) -> Result<()> {
        unsafe { errcode_to_result(ffi::blGradientAddStopRgba64(self.core_mut(), offset, rgba)) }
    }
}

impl Gradient<Linear> {
    #[inline]
    pub fn x1(&self) -> f64 {
        self.value(BL_GRADIENT_VALUE_COMMON_X1 as usize)
    }

    #[inline]
    pub fn y1(&self) -> f64 {
        self.value(BL_GRADIENT_VALUE_COMMON_Y1 as usize)
    }

    #[inline]
    pub fn set_x1(&mut self, val: f64) {
        self.set_value(BL_GRADIENT_VALUE_COMMON_X1 as usize, val)
    }

    #[inline]
    pub fn set_y1(&mut self, val: f64) {
        self.set_value(BL_GRADIENT_VALUE_COMMON_Y1 as usize, val)
    }
}

impl Gradient<Radial> {
    #[inline]
    pub fn x1(&self) -> f64 {
        self.value(BL_GRADIENT_VALUE_COMMON_X1 as usize)
    }

    #[inline]
    pub fn y1(&self) -> f64 {
        self.value(BL_GRADIENT_VALUE_COMMON_Y1 as usize)
    }

    #[inline]
    pub fn r0(&self) -> f64 {
        self.value(BL_GRADIENT_VALUE_RADIAL_R0 as usize)
    }

    #[inline]
    pub fn set_x1(&mut self, val: f64) {
        self.set_value(BL_GRADIENT_VALUE_COMMON_X1 as usize, val)
    }

    #[inline]
    pub fn set_y1(&mut self, val: f64) {
        self.set_value(BL_GRADIENT_VALUE_COMMON_Y1 as usize, val)
    }

    #[inline]
    pub fn set_r0(&mut self, val: f64) {
        self.set_value(BL_GRADIENT_VALUE_RADIAL_R0 as usize, val)
    }
}

impl Gradient<Conical> {
    #[inline]
    pub fn angle(&self) -> f64 {
        self.value(BL_GRADIENT_VALUE_CONICAL_ANGLE as usize)
    }

    #[inline]
    pub fn set_angle(&mut self, val: f64) {
        self.set_value(BL_GRADIENT_VALUE_CONICAL_ANGLE as usize, val)
    }
}

impl<T: GradientType> PartialEq for Gradient<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.impl_() as *const _ == other.impl_() as *const _
    }
}

impl<T: GradientType> Drop for Gradient<T> {
    fn drop(&mut self) {
        unsafe { ffi::blGradientReset(&mut self.core) };
    }
}

impl<T: GradientType> Default for Gradient<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: GradientType> Clone for Gradient<T> {
    fn clone(&self) -> Self {
        Gradient {
            core: self.init_weak(),
            _pd: PhantomData,
        }
    }
}
