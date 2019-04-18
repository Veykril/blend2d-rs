use core::{fmt, marker::PhantomData, mem, ops, ptr, slice};

use ffi::BLGradientValue::*;

use crate::{
    bl_range,
    error::{errcode_to_result, Result},
    matrix::{Matrix2D, Matrix2DOp, MatrixTransform},
    variant::WrappedBlCore,
    ExtendMode,
};
use std::{borrow::Borrow, ops::RangeBounds};

mod private {
    pub trait Sealed {}
    impl Sealed for super::Linear {}
    impl Sealed for super::Radial {}
    impl Sealed for super::Conical {}
}

pub trait GradientType: private::Sealed {
    type ValuesType;
    #[doc(hidden)]
    const BL_TYPE: u32;
}

#[derive(Debug)]
pub enum Linear {}
impl GradientType for Linear {
    type ValuesType = LinearGradientValues;
    #[doc(hidden)]
    const BL_TYPE: u32 = ffi::BLGradientType::BL_GRADIENT_TYPE_LINEAR as u32;
}
#[derive(Debug)]
pub enum Radial {}
impl GradientType for Radial {
    type ValuesType = RadialGradientValues;
    #[doc(hidden)]
    const BL_TYPE: u32 = ffi::BLGradientType::BL_GRADIENT_TYPE_RADIAL as u32;
}
#[derive(Debug)]
pub enum Conical {}
impl GradientType for Conical {
    type ValuesType = ConicalGradientValues;
    #[doc(hidden)]
    const BL_TYPE: u32 = ffi::BLGradientType::BL_GRADIENT_TYPE_CONICAL as u32;
}

/// An offset with an associated color for a gradient.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct GradientStop {
    pub offset: f64,
    pub rgba: u64,
}

/// The values that make up a [`LinearGradient`].
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct LinearGradientValues {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

/// The values that make up a [`RadialGradient`].
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct RadialGradientValues {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    pub r0: f64,
}

/// The values that make up a [`ConicalGradient`].
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct ConicalGradientValues {
    pub x0: f64,
    pub y0: f64,
    pub angle: f64,
}

/// A Dynamic Gradient
#[derive(Debug)]
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

/// A linear gradient.
pub type LinearGradient = Gradient<Linear>;
/// A radial gradient.
pub type RadialGradient = Gradient<Radial>;
/// A conical gradient.
pub type ConicalGradient = Gradient<Conical>;

/// A color gradient. It is generic over its kind, see [`Linear`], [`Radial`]
/// and [`Conical`].
#[repr(transparent)]
pub struct Gradient<T: GradientType> {
    core: ffi::BLGradientCore,
    _pd: PhantomData<*const T>,
}

unsafe impl<T: GradientType> WrappedBlCore for Gradient<T> {
    type Core = ffi::BLGradientCore;
    const IMPL_TYPE_INDEX: usize = crate::variant::ImplType::Gradient as usize;
}

impl<T: GradientType> Gradient<T> {
    /// Creates a new gradient with optional initial stops and an optional
    /// transformation [`Matrix2D`].
    pub fn new(
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

    /// Creates a new gradient from an iterator of [`GradientStop`]s and an
    /// optional transformation [`Matrix2D`]..
    pub fn new_from_iter<I: Iterator<Item = GradientStop>>(
        values: &T::ValuesType,
        extend_mode: ExtendMode,
        stops: I,
        m: Option<&Matrix2D>,
    ) -> Self {
        let stops = stops.collect::<Vec<_>>();
        Self::new(values, extend_mode, &stops, m)
    }

    #[inline]
    pub(in crate) unsafe fn with_type<U: GradientType>(self) -> Gradient<U> {
        Gradient {
            core: self.core,
            _pd: PhantomData,
        }
    }

    /// The [`ExtendMode`] of this gradient.
    #[inline]
    pub fn extend_mode(&self) -> ExtendMode {
        (self.impl_().extendMode as u32).into()
    }

    /// Sets the gradient's [`ExtendMode`].
    #[inline]
    pub fn set_extend_mode(&mut self, mode: ExtendMode) {
        unsafe { ffi::blGradientSetExtendMode(self.core_mut(), mode as u32) };
    }

    #[inline]
    fn value(&self, index: usize) -> f64 {
        unsafe { self.impl_().__bindgen_anon_2.values[index] }
    }

    #[inline]
    fn set_value(&mut self, index: usize, value: f64) {
        assert!(index < ffi::BLGradientValue::BL_GRADIENT_VALUE_COUNT as usize);
        unsafe { ffi::blGradientSetValue(self.core_mut(), index, value) };
    }

    /// The value struct of this gradient.
    #[inline]
    pub fn values(&self) -> &T::ValuesType {
        unsafe { &*(self.impl_().__bindgen_anon_2.values.as_ptr() as *const _ as *const _) }
    }

    /// Sets the value struct of this gradient.
    #[inline]
    pub fn set_values(&mut self, values: &T::ValuesType) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blGradientSetValues(
                self.core_mut(),
                0,
                values as *const _ as *const _,
                mem::size_of::<T::ValuesType>() / mem::size_of::<f64>(),
            ))
        }
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

    /// Returns the transformation matrix.
    #[inline]
    pub fn matrix(&self) -> &Matrix2D {
        unsafe { &*(&self.impl_().matrix as *const _ as *const _) }
    }
}

impl<T: GradientType> Gradient<T> {
    /// Reserves the capacity of gradient stops for at least `n` stops.
    ///
    /// # Panics
    ///
    /// Panics if blend2d returns an [`OutOfMemory`] error.
    ///
    /// [`OutOfMemory`]: ../error/enum.Error.html#variant.OutOfMemory
    #[inline]
    pub fn reserve(&mut self, n: usize) {
        self.try_reserve(n).unwrap();
    }

    /// Reserves the capacity of gradient stops for at least `n` stops.
    #[inline]
    pub fn try_reserve(&mut self, n: usize) -> Result<()> {
        unsafe { errcode_to_result(ffi::blGradientReserve(self.core_mut(), n)) }
    }

    /// Shrinks the capacity of gradient stops to fit the current usage.
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        unsafe { errcode_to_result(ffi::blGradientShrink(self.core_mut())).unwrap() };
    }

    /// Returns the number of stops in this gradient.
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { self.impl_().__bindgen_anon_1.__bindgen_anon_1.size }
    }

    /// Returns the number of stops in this gradient.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.impl_().capacity as usize
    }

    /// Returns true if this gradient is empty, in other words if it has no
    /// stops.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the gradient stops as an immutable slice.
    #[inline]
    pub fn stops(&self) -> &[GradientStop] {
        unsafe {
            slice::from_raw_parts(
                self.impl_().__bindgen_anon_1.__bindgen_anon_1.stops as *const _,
                self.len(),
            )
        }
    }

    /// Removes the stop at the specified index.
    #[inline]
    pub fn remove_stop(&mut self, index: usize) -> Result<()> {
        unsafe { errcode_to_result(ffi::blGradientRemoveStop(self.core_mut(), index)) }
    }

    /// Removes multiple stops indexed by the given range.
    #[inline]
    pub fn remove_stops<R: RangeBounds<usize>>(&mut self, range: R) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blGradientRemoveStops(
                self.core_mut(),
                &bl_range(range),
            ))
        }
    }

    /// Removes the first stop that matches the given offset.
    #[inline]
    pub fn remove_stop_by_offset(&mut self, offset: f64) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blGradientRemoveStopByOffset(
                self.core_mut(),
                offset,
                false as _,
            ))
        }
    }

    /// Removes a range of stops that lie inside of the given range.
    ///
    /// The range specified will always include its upper bound.
    #[inline]
    pub fn remove_stops_in_range<R: ops::RangeBounds<f64>>(&mut self, range: R) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blGradientRemoveStopsFromTo(
                self.core_mut(),
                match range.start_bound() {
                    ops::Bound::Included(n) | ops::Bound::Excluded(n) => *n,
                    ops::Bound::Unbounded => 0.0,
                },
                match range.end_bound() {
                    ops::Bound::Included(n) | ops::Bound::Excluded(n) => *n,
                    ops::Bound::Unbounded => 1.0,
                },
            ))
        }
    }

    /// Removes all stops matching the given offset.
    #[inline]
    pub fn remove_all_stops_by_offset(&mut self, offset: f64) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blGradientRemoveStopByOffset(
                self.core_mut(),
                offset,
                true as _,
            ))
        }
    }

    /// Returns the index of of the stop with with the given offset, or None if
    /// none exist.
    #[inline]
    pub fn index_of_stop(&self, offset: f64) -> Option<usize> {
        unsafe {
            let idx = ffi::blGradientIndexOfStop(self.core(), offset);
            if idx == !0 {
                None
            } else {
                Some(idx)
            }
        }
    }

    /// Clears the stops buffer.
    #[inline]
    pub fn reset_stops(&mut self) -> Result<()> {
        unsafe { errcode_to_result(ffi::blGradientResetStops(self.core_mut())) }
    }

    /// Adds a gradient stop to the buffer.
    #[inline]
    pub fn add_stop(&mut self, stop: GradientStop) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blGradientAddStopRgba64(
                self.core_mut(),
                stop.offset,
                stop.rgba,
            ))
        }
    }

    /// Adds a gradient stop to the buffer.
    #[inline]
    pub fn add_stop32(&mut self, offset: f64, rgba: u32) -> Result<()> {
        unsafe { errcode_to_result(ffi::blGradientAddStopRgba32(self.core_mut(), offset, rgba)) }
    }

    /// Adds a gradient stop to the buffer.
    #[inline]
    pub fn add_stop64(&mut self, offset: f64, rgba: u64) -> Result<()> {
        unsafe { errcode_to_result(ffi::blGradientAddStopRgba64(self.core_mut(), offset, rgba)) }
    }
}

impl Gradient<Linear> {
    #[inline]
    pub fn new_linear(
        values: &LinearGradientValues,
        extend_mode: ExtendMode,
        stops: &[GradientStop],
        m: Option<&Matrix2D>,
    ) -> Self {
        Self::new(values, extend_mode, stops, m)
    }

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
    pub fn new_radial(
        values: &RadialGradientValues,
        extend_mode: ExtendMode,
        stops: &[GradientStop],
        m: Option<&Matrix2D>,
    ) -> Self {
        Self::new(values, extend_mode, stops, m)
    }

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
    pub fn new_conical(
        values: &ConicalGradientValues,
        extend_mode: ExtendMode,
        stops: &[GradientStop],
        m: Option<&Matrix2D>,
    ) -> Self {
        Self::new(values, extend_mode, stops, m)
    }

    #[inline]
    pub fn angle(&self) -> f64 {
        self.value(BL_GRADIENT_VALUE_CONICAL_ANGLE as usize)
    }

    #[inline]
    pub fn set_angle(&mut self, val: f64) {
        self.set_value(BL_GRADIENT_VALUE_CONICAL_ANGLE as usize, val)
    }
}

impl<T: GradientType> MatrixTransform for Gradient<T> {
    #[inline]
    #[doc(hidden)]
    fn apply_matrix_op(&mut self, op: Matrix2DOp, data: &[f64]) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blGradientApplyMatrixOp(
                self.core_mut(),
                op as u32,
                data.as_ptr() as *const _,
            ))
        }
    }
}

impl<T> Default for Gradient<T>
where
    T: GradientType,
    T::ValuesType: Default,
{
    #[inline]
    fn default() -> Self {
        Self::new(&Default::default(), Default::default(), &[], None)
    }
}

impl<T: GradientType> Borrow<[GradientStop]> for Gradient<T> {
    fn borrow(&self) -> &[GradientStop] {
        self
    }
}

impl<T: GradientType> AsRef<[GradientStop]> for Gradient<T> {
    #[inline]
    fn as_ref(&self) -> &[GradientStop] {
        self
    }
}

impl<T: GradientType> ops::Deref for Gradient<T> {
    type Target = [GradientStop];

    fn deref(&self) -> &Self::Target {
        self.stops()
    }
}

impl<T> fmt::Debug for Gradient<T>
where
    T: GradientType,
    T::ValuesType: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Gradient")
            .field("values", self.values())
            .field("extend_mode", &self.extend_mode())
            .field("stops", &self.stops())
            .field("matrix", self.matrix())
            .finish()
    }
}

impl<T: GradientType> PartialEq for Gradient<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::blGradientEquals(self.core(), other.core()) }
    }
}

impl<T: GradientType> Drop for Gradient<T> {
    fn drop(&mut self) {
        unsafe { ffi::blGradientReset(&mut self.core) };
    }
}

#[cfg(test)]
mod test_gradient {
    use crate::{
        gradient::{Conical, Gradient, GradientStop, Linear, LinearGradientValues},
        matrix::{Matrix2D, MatrixTransform},
        ExtendMode,
    };

    #[test]
    fn test_gradient_default() {
        let gradient = Gradient::<Conical>::default();

        assert_eq!(gradient.values(), &Default::default());
        assert_eq!(gradient.stops(), &[]);
        assert_eq!(gradient.matrix(), &Matrix2D::identity());
    }

    #[test]
    fn test_gradient_new() {
        let values = LinearGradientValues {
            x0: 0.0,
            y0: 0.0,
            x1: 100.0,
            y1: 100.0,
        };
        let stops = [GradientStop {
            offset: 0.5,
            rgba: 0xFF123456,
        }];
        let mat = Matrix2D::scaling(1.0, 2.0);

        let gradient = Gradient::<Linear>::new(&values, ExtendMode::PadXPadY, &stops, Some(&mat));

        assert_eq!(gradient.values(), &values);
        assert_eq!(gradient.stops(), &stops);
        assert_eq!(gradient.matrix(), &mat);
    }

    #[test]
    fn test_gradient_default_eq_late_init() {
        let values = LinearGradientValues {
            x0: 0.0,
            y0: 0.0,
            x1: 100.0,
            y1: 100.0,
        };
        let stops = [GradientStop {
            offset: 0.5,
            rgba: 0xFF123456,
        }];
        let mat = Matrix2D::scaling(1.0, 2.0);

        let gradient = Gradient::<Linear>::new(&values, ExtendMode::PadXPadY, &stops, Some(&mat));
        let mut default = Gradient::<Linear>::default();
        default.set_values(&values).unwrap();
        default.add_stop(stops[0]).unwrap();
        default.set_matrix(&mat).unwrap();

        assert_eq!(gradient, default);
    }
}
