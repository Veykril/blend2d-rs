#[macro_use]
mod macros;

pub mod array;
pub mod codec;
pub mod context;
pub mod error;
pub mod format;
pub mod geometry;
pub mod image;

// Row-Major
#[derive(Default, Copy, Clone)]
pub struct Matrix2D(pub [f64; ffi::BLMatrix2DValue::BL_MATRIX2D_VALUE_COUNT as usize]);

pub(in crate) trait ImplType: Sized {
    type CoreType;
    const IMPL_TYPE_ID: usize;

    #[inline]
    fn none() -> &'static Self::CoreType {
        debug_assert!(Self::IMPL_TYPE_ID < ffi::BLImplType::BL_IMPL_TYPE_COUNT as usize);
        unsafe { &*(&ffi::blNone[Self::IMPL_TYPE_ID] as *const _ as *const _) }
    }
}

use core::ops;
pub(in crate) fn bl_range<R: ops::RangeBounds<usize>>(range: R) -> ffi::BLRange {
    ffi::BLRange {
        start: match range.start_bound() {
            ops::Bound::Included(n) => *n,
            ops::Bound::Excluded(n) => *n + 1,
            ops::Bound::Unbounded => 0,
        },
        end: match range.end_bound() {
            ops::Bound::Included(n) => *n,
            ops::Bound::Excluded(n) => *n - 1,
            ops::Bound::Unbounded => 0,
        },
    }
}
