#![allow(clippy::cast_lossless)]
#[macro_use]
mod macros;

pub(in crate) mod variant;

pub mod array;
pub mod codec;
pub mod context;
pub mod error;
pub mod format;
pub mod geometry;
pub mod gradient;
pub mod image;

#[repr(transparent)]
pub struct Tag(u32);

use ffi::BLExtendMode::*;
bl_enum! {
    pub enum ExtendMode {
        PadXPadY = BL_EXTEND_MODE_PAD_X_PAD_Y,
        RepeatXRepeatY = BL_EXTEND_MODE_REPEAT_X_REPEAT_Y,
        ReflectXReflectY = BL_EXTEND_MODE_REFLECT_X_REFLECT_Y,
        PadXRepeatY = BL_EXTEND_MODE_PAD_X_REPEAT_Y,
        PadXReflectY = BL_EXTEND_MODE_PAD_X_REFLECT_Y,
        RepeatXPadY = BL_EXTEND_MODE_REPEAT_X_PAD_Y,
        RepeatXReflectY = BL_EXTEND_MODE_REPEAT_X_REFLECT_Y,
        ReflectXPadY = BL_EXTEND_MODE_REFLECT_X_PAD_Y,
        ReflectXRepeatY = BL_EXTEND_MODE_REFLECT_X_REPEAT_Y,
    }
    Default => PadXPadY
}

use ffi::BLStyleType::*;
bl_enum! {
    pub enum StyleType {
        None = BL_STYLE_TYPE_NONE,
        Solid = BL_STYLE_TYPE_SOLID,
        Pattern = BL_STYLE_TYPE_PATTERN,
        Gradient = BL_STYLE_TYPE_GRADIENT,
    }
    Default => None
}

use ffi::BLMatrix2DType::*;
bl_enum! {
    pub enum Matrix2DType {
        Identity = BL_MATRIX2D_TYPE_IDENTITY,
        Translate = BL_MATRIX2D_TYPE_TRANSLATE,
        Scale = BL_MATRIX2D_TYPE_SCALE,
        Swap = BL_MATRIX2D_TYPE_SWAP,
        Affine = BL_MATRIX2D_TYPE_AFFINE,
        Invalid = BL_MATRIX2D_TYPE_INVALID,
    }
    Default => Identity
}

// Row-Major
#[derive(Default, Copy, Clone)]
pub struct Matrix2D(pub [f64; ffi::BLMatrix2DValue::BL_MATRIX2D_VALUE_COUNT as usize]);

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
