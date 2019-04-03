use crate::{
    error::{errcode_to_result, Result},
    ImplType,
};

pub struct Path {
    pub(in crate) core: ffi::BLPathCore,
}

impl Path {
    pub fn new() -> Self {
        Path {
            core: ffi::BLPathCore {
                impl_: Self::none().impl_,
            },
        }
    }

    pub fn move_to(&mut self, x: f64, y: f64) -> Result<()> {
        unsafe { errcode_to_result(ffi::blPathMoveTo(&mut self.core, x, y)) }
    }

    pub fn cubic_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> Result<()> {
        unsafe { errcode_to_result(ffi::blPathCubicTo(&mut self.core, x1, y1, x2, y2, x3, y3)) }
    }
}

impl ImplType for Path {
    type Type = ffi::BLPathCore;
    const IDX: usize = ffi::BLImplType::BL_IMPL_TYPE_PATH2D as usize;
}
