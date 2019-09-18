//! Linear, Radial and Conical Gradients.

use std::borrow::Borrow;
use std::marker::PhantomData;
use std::ops::{self, RangeBounds};
use std::{fmt, mem, ptr, slice};

use ffi::BLGradientValue::*;

use crate::error::{expect_mem_err, OutOfMemory};
use crate::matrix::{Matrix2D, Matrix2DOp, MatrixTransform};
use crate::util::range_to_tuple;
use crate::variant::WrappedBlCore;
use crate::ExtendMode;

mod private {
    pub trait Sealed {}
    impl Sealed for super::Linear {}
    impl Sealed for super::Radial {}
    impl Sealed for super::Conical {}
}

/// The type of the gradient.
pub trait GradientType: private::Sealed {
    type ValuesType;
    #[doc(hidden)]
    const BL_TYPE: u32;
}

/// Template type that marks a gradient as being a linear one.
#[derive(Debug)]
pub enum Linear {}
impl GradientType for Linear {
    type ValuesType = LinearGradientValues;
    #[doc(hidden)]
    const BL_TYPE: u32 = ffi::BLGradientType::BL_GRADIENT_TYPE_LINEAR as u32;
}
/// Template type that marks a gradient as being a radial one.
#[derive(Debug)]
pub enum Radial {}
impl GradientType for Radial {
    type ValuesType = RadialGradientValues;
    #[doc(hidden)]
    const BL_TYPE: u32 = ffi::BLGradientType::BL_GRADIENT_TYPE_RADIAL as u32;
}

/// Template type that marks a gradient as being a conical one.
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

    #[inline]
    fn from_core(core: Self::Core) -> Self {
        Gradient {
            core,
            _pd: PhantomData,
        }
    }
}

impl<T: GradientType> Gradient<T> {
    /// Creates a new gradient with optional initial stops and an optional
    /// transformation [`Matrix2D`].
    #[inline]
    pub fn new<'m, R, M>(values: &T::ValuesType, extend_mode: ExtendMode, stops: R, m: M) -> Self
    where
        R: AsRef<[GradientStop]>,
        M: Into<Option<&'m Matrix2D>>,
    {
        let mut this = Gradient::from_core(*Self::none());
        unsafe {
            ffi::blGradientInitAs(
                this.core_mut(),
                T::BL_TYPE,
                values as *const _ as *const _,
                extend_mode as u32,
                stops.as_ref().as_ptr() as *const _ as *const _,
                stops.as_ref().len(),
                m.into()
                    .map_or(ptr::null_mut(), |m| m as *const _ as *const _),
            )
        };
        this
    }

    /// Creates a new gradient from an iterator of [`GradientStop`]s and an
    /// optional transformation [`Matrix2D`].
    pub fn new_from_iter<'m, I, M>(
        values: &T::ValuesType,
        extend_mode: ExtendMode,
        stops: I,
        m: M,
    ) -> Self
    where
        I: IntoIterator<Item = GradientStop>,
        M: Into<Option<&'m Matrix2D>>,
    {
        let mut this = Self::new(values, extend_mode, &[], m);
        let stops = stops.into_iter();
        let len = stops.size_hint().1.unwrap_or(stops.size_hint().0);
        this.try_reserve(len).unwrap();
        this.extend(stops);
        this
    }

    /// The [`ExtendMode`] of this gradient.
    #[inline]
    pub fn extend_mode(&self) -> ExtendMode {
        u32::from(self.impl_().extendMode).into()
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
    pub fn set_values(&mut self, values: &T::ValuesType) {
        unsafe {
            expect_mem_err(ffi::blGradientSetValues(
                self.core_mut(),
                0,
                values as *const _ as *const _,
                mem::size_of::<T::ValuesType>() / mem::size_of::<f64>(),
            ))
        };
    }

    /// Returns the x0 value of this gradient.
    #[inline]
    pub fn x0(&self) -> f64 {
        self.value(BL_GRADIENT_VALUE_COMMON_X0 as usize)
    }

    /// Returns the y0 value of this gradient.
    #[inline]
    pub fn y0(&self) -> f64 {
        self.value(BL_GRADIENT_VALUE_COMMON_Y0 as usize)
    }

    /// Sets the x0 value of this gradient.
    #[inline]
    pub fn set_x0(&mut self, val: f64) {
        self.set_value(BL_GRADIENT_VALUE_COMMON_X0 as usize, val)
    }

    /// Sets the y0 value of this gradient.
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
        self.try_reserve(n).expect("memory allocation failed");
    }

    /// Reserves the capacity of gradient stops for at least `n` stops.
    #[inline]
    pub fn try_reserve(&mut self, n: usize) -> std::result::Result<(), OutOfMemory> {
        unsafe { OutOfMemory::from_errcode(ffi::blGradientReserve(self.core_mut(), n)) }
    }

    /// Shrinks the capacity of gradient stops to fit the current usage.
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        unsafe { expect_mem_err(ffi::blGradientShrink(self.core_mut())) };
    }

    /// Returns the number of stops in this gradient.
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { ffi::blGradientGetSize(self.core()) }
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
    pub fn remove_stop(&mut self, index: usize) {
        unsafe { expect_mem_err(ffi::blGradientRemoveStop(self.core_mut(), index)) }
    }

    /// Removes multiple stops indexed by the given range.
    #[inline]
    pub fn remove_stops<R: RangeBounds<usize>>(&mut self, range: R) {
        let (start, end) = range_to_tuple(range, || self.len());
        unsafe { expect_mem_err(ffi::blGradientRemoveStops(self.core_mut(), start, end)) };
    }

    /// Removes the first stop that matches the given offset.
    #[inline]
    pub fn remove_stop_by_offset(&mut self, offset: f64) {
        unsafe {
            expect_mem_err(ffi::blGradientRemoveStopByOffset(
                self.core_mut(),
                offset,
                false as _,
            ))
        };
    }

    /// Removes a range of stops that lie inside of the given range.
    ///
    /// The range specified will always include its upper bound.
    #[inline]
    pub fn remove_stops_in_range<R: ops::RangeBounds<f64>>(&mut self, range: R) {
        unsafe {
            expect_mem_err(ffi::blGradientRemoveStopsFromTo(
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
        };
    }

    /// Removes all stops matching the given offset.
    #[inline]
    pub fn remove_all_stops_by_offset(&mut self, offset: f64) {
        unsafe {
            expect_mem_err(ffi::blGradientRemoveStopByOffset(
                self.core_mut(),
                offset,
                true as _,
            ))
        };
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
    pub fn reset_stops(&mut self) {
        unsafe { expect_mem_err(ffi::blGradientResetStops(self.core_mut())) };
    }

    /// Adds a gradient stop to the buffer.
    #[inline]
    pub fn add_stop(&mut self, stop: GradientStop) {
        unsafe {
            expect_mem_err(ffi::blGradientAddStopRgba64(
                self.core_mut(),
                stop.offset,
                stop.rgba,
            ))
        };
    }

    /// Adds a gradient stop to the buffer.
    #[inline]
    pub fn add_stop32(&mut self, offset: f64, rgba: u32) {
        unsafe { expect_mem_err(ffi::blGradientAddStopRgba32(self.core_mut(), offset, rgba)) };
    }

    /// Adds a gradient stop to the buffer.
    #[inline]
    pub fn add_stop64(&mut self, offset: f64, rgba: u64) {
        unsafe { expect_mem_err(ffi::blGradientAddStopRgba64(self.core_mut(), offset, rgba)) };
    }
}

impl Gradient<Linear> {
    #[inline]
    pub fn new_linear<'m, M>(
        values: &LinearGradientValues,
        extend_mode: ExtendMode,
        stops: &[GradientStop],
        m: M,
    ) -> Self
    where
        M: Into<Option<&'m Matrix2D>>,
    {
        Self::new(values, extend_mode, stops, m)
    }

    /// Returns the x1 value of this gradient.
    #[inline]
    pub fn x1(&self) -> f64 {
        self.value(BL_GRADIENT_VALUE_COMMON_X1 as usize)
    }

    /// Returns the y1 value of this gradient.
    #[inline]
    pub fn y1(&self) -> f64 {
        self.value(BL_GRADIENT_VALUE_COMMON_Y1 as usize)
    }

    /// Sets the x1 value of this gradient.
    #[inline]
    pub fn set_x1(&mut self, val: f64) {
        self.set_value(BL_GRADIENT_VALUE_COMMON_X1 as usize, val)
    }

    /// Sets the y1 value of this gradient.
    #[inline]
    pub fn set_y1(&mut self, val: f64) {
        self.set_value(BL_GRADIENT_VALUE_COMMON_Y1 as usize, val)
    }
}

impl Gradient<Radial> {
    #[inline]
    pub fn new_radial<'m, M>(
        values: &RadialGradientValues,
        extend_mode: ExtendMode,
        stops: &[GradientStop],
        m: M,
    ) -> Self
    where
        M: Into<Option<&'m Matrix2D>>,
    {
        Self::new(values, extend_mode, stops, m)
    }

    /// Returns the x1 value of this gradient.
    #[inline]
    pub fn x1(&self) -> f64 {
        self.value(BL_GRADIENT_VALUE_COMMON_X1 as usize)
    }

    /// Returns the y1 value of this gradient.
    #[inline]
    pub fn y1(&self) -> f64 {
        self.value(BL_GRADIENT_VALUE_COMMON_Y1 as usize)
    }

    /// Returns the r0 value of this gradient.
    #[inline]
    pub fn r0(&self) -> f64 {
        self.value(BL_GRADIENT_VALUE_RADIAL_R0 as usize)
    }

    /// Sets the x1 value of this gradient.
    #[inline]
    pub fn set_x1(&mut self, val: f64) {
        self.set_value(BL_GRADIENT_VALUE_COMMON_X1 as usize, val)
    }

    /// Sets the y1 value of this gradient.
    #[inline]
    pub fn set_y1(&mut self, val: f64) {
        self.set_value(BL_GRADIENT_VALUE_COMMON_Y1 as usize, val)
    }

    /// Sets the r0 value of this gradient.
    #[inline]
    pub fn set_r0(&mut self, val: f64) {
        self.set_value(BL_GRADIENT_VALUE_RADIAL_R0 as usize, val)
    }
}

impl Gradient<Conical> {
    #[inline]
    pub fn new_conical<'m, M>(
        values: &ConicalGradientValues,
        extend_mode: ExtendMode,
        stops: &[GradientStop],
        m: M,
    ) -> Self
    where
        M: Into<Option<&'m Matrix2D>>,
    {
        Self::new(values, extend_mode, stops, m)
    }

    /// Returns the angle of this gradient.
    #[inline]
    pub fn angle(&self) -> f64 {
        self.value(BL_GRADIENT_VALUE_CONICAL_ANGLE as usize)
    }

    /// Sets the angle of this gradient.
    #[inline]
    pub fn set_angle(&mut self, val: f64) {
        self.set_value(BL_GRADIENT_VALUE_CONICAL_ANGLE as usize, val)
    }
}

impl<'a, T: GradientType> From<&'a T::ValuesType> for Gradient<T> {
    #[inline]
    fn from(v: &T::ValuesType) -> Self {
        Self::new(v, Default::default(), &[], None)
    }
}

impl<T: GradientType> MatrixTransform for Gradient<T> {
    #[inline]
    #[doc(hidden)]
    fn apply_matrix_op(&mut self, op: Matrix2DOp, data: &[f64]) {
        unsafe {
            expect_mem_err(ffi::blGradientApplyMatrixOp(
                self.core_mut(),
                op as u32,
                data.as_ptr() as *const _,
            ))
        };
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
    #[inline]
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

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.stops()
    }
}

impl<T, I> ops::Index<I> for Gradient<T>
where
    T: GradientType,
    I: slice::SliceIndex<[GradientStop]>,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        ops::Index::index(&**self, index)
    }
}

impl<'a, T: GradientType> IntoIterator for &'a Gradient<T> {
    type Item = &'a GradientStop;
    type IntoIter = slice::Iter<'a, GradientStop>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: GradientType> Extend<GradientStop> for Gradient<T> {
    #[inline]
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = GradientStop>,
    {
        for stop in iter {
            self.add_stop(stop)
        }
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

impl<T: GradientType> Clone for Gradient<T> {
    fn clone(&self) -> Self {
        Self::from_core(self.init_weak())
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
            rgba: 0xFF_12_34_56,
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
            rgba: 0xFF_12_34_56,
        }];
        let mat = Matrix2D::scaling(1.0, 2.0);

        let gradient = Gradient::<Linear>::new(&values, ExtendMode::PadXPadY, &stops, Some(&mat));
        let mut default = Gradient::<Linear>::default();
        default.set_values(&values);
        default.add_stop(stops[0]);
        default.set_matrix(&mat);

        assert_eq!(gradient, default);
    }
}
