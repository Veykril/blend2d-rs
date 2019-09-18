use core::{borrow::Borrow, fmt, slice};

use crate::{
    error::{errcode_to_result, expect_mem_err, OutOfMemory},
    geometry::{BoxI, HitTest, PointI, RectI},
    variant::WrappedBlCore,
    BooleanOp,
};

use ffi::BLRegionType::*;
bl_enum! {
    pub enum RegionType {
        Empty   = BL_REGION_TYPE_EMPTY,
        Rect    = BL_REGION_TYPE_RECT,
        Complex = BL_REGION_TYPE_COMPLEX,
    }
    Default => Empty
}

#[repr(transparent)]
pub struct Region {
    core: ffi::BLRegionCore,
}

unsafe impl WrappedBlCore for Region {
    type Core = ffi::BLRegionCore;
    const IMPL_TYPE_INDEX: usize = crate::variant::ImplType::Region as usize;

    #[inline]
    fn from_core(core: Self::Core) -> Self {
        Region { core }
    }
}

impl Region {
    #[inline]
    pub fn new() -> Self {
        Region::from_core(*Self::none())
    }

    #[inline]
    pub fn region_type(&self) -> RegionType {
        unsafe { ffi::blRegionGetType(self.core()).into() }
    }

    /// Returns true if this region is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.region_type() == RegionType::Empty
    }

    /// Returns true if this region is a rectangle.
    #[inline]
    pub fn is_rect(&self) -> bool {
        self.region_type() == RegionType::Rect
    }

    /// Returns true if this region is of complex shape.
    #[inline]
    pub fn is_complex(&self) -> bool {
        self.region_type() == RegionType::Complex
    }

    /// The region's boxes.
    #[inline]
    pub fn data(&self) -> &[BoxI] {
        unsafe { slice::from_raw_parts(ffi::blRegionGetData(self.core()) as *const _, self.len()) }
    }

    /// The number of [`BoxI`] this region contains.
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { ffi::blRegionGetSize(self.core()) }
    }

    /// Returns the currently allocated capacity of the region.
    #[inline]
    pub fn capacity(&self) -> usize {
        unsafe { ffi::blRegionGetCapacity(self.core()) }
    }

    /// A bounding box representing this region.
    #[inline]
    pub fn bounding_box(&self) -> &BoxI {
        unsafe { &*(&self.impl_().boundingBox as *const _ as *const _) }
    }

    /// Clears the region.
    #[inline]
    pub fn clear(&mut self) {
        unsafe { expect_mem_err(ffi::blRegionClear(self.core_mut())) };
    }

    /// Reserves capacity for at least n boxes.
    ///
    /// # Panics
    ///
    /// Panics if blend2d returns an
    /// [`OutOfMemory`](../error/enum.Error.html#variant.OutOfMemory) error
    #[inline]
    pub fn reserve(&mut self, n: usize) {
        self.try_reserve(n).expect("memory allocation failed")
    }

    /// Reserves capacity for at least n boxes.
    #[inline]
    pub fn try_reserve(&mut self, n: usize) -> std::result::Result<(), OutOfMemory> {
        unsafe { OutOfMemory::from_errcode(ffi::blRegionReserve(self.core_mut(), n)) }
    }

    /// Shrinks the region's allocated capacity down to its current length.
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        unsafe { expect_mem_err(ffi::blRegionShrink(self.core_mut())) };
    }

    #[inline]
    pub fn combine(&mut self, other: &Self, op: BooleanOp) {
        unsafe {
            expect_mem_err(ffi::blRegionCombine(
                self.core_mut(),
                self.core(),
                other.core(),
                op.into(),
            ));
        }
    }

    #[inline]
    pub fn combine_rb(&mut self, b: &BoxI, op: BooleanOp) {
        unsafe {
            expect_mem_err(ffi::blRegionCombineRB(
                self.core_mut(),
                self.core(),
                &b as *const _ as *const _,
                op.into(),
            ))
        };
    }

    #[inline]
    pub fn combine_br(&mut self, b: &BoxI, op: BooleanOp) {
        unsafe {
            expect_mem_err(ffi::blRegionCombineBR(
                self.core_mut(),
                &b as *const _ as *const _,
                self.core(),
                op.into(),
            ))
        };
    }

    #[inline]
    pub fn combine_bb(&mut self, b: &BoxI, b2: &BoxI, op: BooleanOp) {
        unsafe {
            expect_mem_err(ffi::blRegionCombineBB(
                self.core_mut(),
                &b as *const _ as *const _,
                &b2 as *const _ as *const _,
                op.into(),
            ))
        };
    }

    /// Translates the region by the given [`PointI`].
    /// Possible overflow will be handled by clipping to a maximum region
    /// boundary, so the final region could be smaller than the region before
    /// translation.
    #[inline]
    pub fn translate(&mut self, p: PointI) {
        unsafe {
            expect_mem_err(ffi::blRegionTranslate(
                self.core_mut(),
                self.core(),
                &p as *const _ as *const _,
            ))
        };
    }

    /// Translates the region by the given [`PointI`] and clips it to the given
    /// `[BoxI]`.
    #[inline]
    pub fn translate_and_clip(&mut self, p: PointI, clip: &BoxI) {
        unsafe {
            expect_mem_err(ffi::blRegionTranslateAndClip(
                self.core_mut(),
                self.core(),
                &p as *const _ as *const _,
                clip as *const _ as *const _,
            ))
        };
    }
    /// Translates the region by the given region and clips it to the given
    /// `[BoxI]`.
    #[inline]
    pub fn intersect_and_clip(&mut self, region: &Region, clip: &BoxI) {
        unsafe {
            expect_mem_err(ffi::blRegionIntersectAndClip(
                self.core_mut(),
                self.core(),
                region.core(),
                clip as *const _ as *const _,
            ))
        };
    }

    /// Tests if a given [`PointI`] is in the region.
    #[inline]
    pub fn hit_test(&self, p: PointI) -> HitTest {
        unsafe { ffi::blRegionHitTest(self.core(), &p as *const _ as *const _).into() }
    }

    /// Tests if a given [`BoxI`] is in the region.
    #[inline]
    pub fn hit_test_box(&self, b: &BoxI) -> HitTest {
        unsafe { ffi::blRegionHitTestBoxI(self.core(), b as *const _ as *const _).into() }
    }
}

impl From<BoxI> for Region {
    #[inline]
    fn from(b: BoxI) -> Self {
        let mut this = Self::new();
        unsafe {
            errcode_to_result(ffi::blRegionAssignBoxI(
                this.core_mut(),
                &b as *const _ as *const _,
            ))
            .unwrap()
        };
        this
    }
}

impl<'a> From<&'a [BoxI]> for Region {
    #[inline]
    fn from(b: &'a [BoxI]) -> Self {
        let mut this = Self::new();
        unsafe {
            errcode_to_result(ffi::blRegionAssignBoxIArray(
                this.core_mut(),
                b.as_ptr() as *const _,
                b.len(),
            ))
            .unwrap()
        };
        this
    }
}

impl From<RectI> for Region {
    #[inline]
    fn from(r: RectI) -> Self {
        let mut this = Self::new();
        unsafe {
            errcode_to_result(ffi::blRegionAssignRectI(
                this.core_mut(),
                &r as *const _ as *const _,
            ))
            .unwrap()
        };
        this
    }
}

impl<'a> From<&'a [RectI]> for Region {
    #[inline]
    fn from(r: &'a [RectI]) -> Self {
        let mut this = Self::new();
        unsafe {
            errcode_to_result(ffi::blRegionAssignRectIArray(
                this.core_mut(),
                r.as_ptr() as *const _,
                r.len(),
            ))
            .unwrap()
        };
        this
    }
}

impl AsRef<[BoxI]> for Region {
    #[inline]
    fn as_ref(&self) -> &[BoxI] {
        self.data()
    }
}

impl Borrow<[BoxI]> for Region {
    #[inline]
    fn borrow(&self) -> &[BoxI] {
        self.data()
    }
}

impl Default for Region {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Region {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::blRegionEquals(self.core(), other.core()) }
    }
}

impl fmt::Debug for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Region")
            .field("region_type", &self.region_type())
            .field("data", &self.data())
            .finish()
    }
}

impl Clone for Region {
    fn clone(&self) -> Self {
        Self::from_core(self.init_weak())
    }
}

impl Drop for Region {
    fn drop(&mut self) {
        unsafe { ffi::blRegionReset(&mut self.core) };
    }
}
