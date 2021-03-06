#![deny(rust_2018_idioms, missing_debug_implementations)]
#![warn(clippy::correctness, clippy::complexity)]

#[macro_use]
mod macros;

pub(in crate) mod util;
pub(in crate) mod variant;

pub use self::variant::DeepClone;

pub mod array;
pub mod codec;
pub mod context;
pub mod error;
pub mod font;
pub mod font_defs;
pub mod geometry;
pub mod glyph_buffer;
pub mod gradient;
pub mod image;
pub mod matrix;
pub mod path;
pub mod pattern;
pub mod prelude;
pub mod region;
pub mod runtime;

use bitflags::bitflags;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tag(u32);

use ffi::BLBooleanOp::*;
bl_enum! {
    pub enum BooleanOp {
        /// Result = B
        Copy = BL_BOOLEAN_OP_COPY,
        /// Result = A & B
        And  = BL_BOOLEAN_OP_AND,
        /// Result = A | B
        Or   = BL_BOOLEAN_OP_OR,
        /// Result = A ^ B
        Xor  = BL_BOOLEAN_OP_XOR,
        /// Result = A & ~B
        Sub  = BL_BOOLEAN_OP_SUB,
    }
    Default => Copy
}

use ffi::BLExtendMode::*;
bl_enum! {
    pub enum ExtendMode {
        PadXPadY =         BL_EXTEND_MODE_PAD_X_PAD_Y,
        RepeatXRepeatY =   BL_EXTEND_MODE_REPEAT_X_REPEAT_Y,
        ReflectXReflectY = BL_EXTEND_MODE_REFLECT_X_REFLECT_Y,
        PadXRepeatY =      BL_EXTEND_MODE_PAD_X_REPEAT_Y,
        PadXReflectY =     BL_EXTEND_MODE_PAD_X_REFLECT_Y,
        RepeatXPadY =      BL_EXTEND_MODE_REPEAT_X_PAD_Y,
        RepeatXReflectY =  BL_EXTEND_MODE_REPEAT_X_REFLECT_Y,
        ReflectXPadY =     BL_EXTEND_MODE_REFLECT_X_PAD_Y,
        ReflectXRepeatY =  BL_EXTEND_MODE_REFLECT_X_REPEAT_Y,
    }
    Default => PadXPadY
}

use ffi::BLStyleType::*;
bl_enum! {
    pub enum StyleType {
        None =     BL_STYLE_TYPE_NONE,
        Solid =    BL_STYLE_TYPE_SOLID,
        Pattern =  BL_STYLE_TYPE_PATTERN,
        Gradient = BL_STYLE_TYPE_GRADIENT,
    }
    Default => None
}

use ffi::BLDataAccessFlags::*;
bitflags! {
    pub struct DataAccessFlags: u32 {
        const READ       = BL_DATA_ACCESS_READ as u32;
        const WRITE      = BL_DATA_ACCESS_READ as u32;
        const READ_WRITE = BL_DATA_ACCESS_READ as u32;
    }
}
