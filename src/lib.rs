pub mod error;

use error::{errcode_to_result, Result};

use core::ptr;
use ffi::BLContextCore;
use ffi::BLImageCore;

pub struct Context {
    context_core: BLContextCore,
}

impl Context {
    pub fn new(target: &mut Image) -> Result<Self> {
        unsafe {
            let mut core = std::mem::uninitialized();

            errcode_to_result(ffi::blContextInitAs(
                &mut core,
                &mut target.core,
                ptr::null(),
            ))
            .map(|_| Context { context_core: core })
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { ffi::blContextReset(&mut self.context_core) };
    }
}

pub enum ImageFormat {
    None,
    PRgb32,
    XRgb32,
    A8,
}

impl From<ImageFormat> for u32 {
    fn from(format: ImageFormat) -> u32 {
        (match format {
            ImageFormat::None => ffi::BLFormat::BL_FORMAT_NONE,
            ImageFormat::PRgb32 => ffi::BLFormat::BL_FORMAT_PRGB32,
            ImageFormat::XRgb32 => ffi::BLFormat::BL_FORMAT_XRGB32,
            ImageFormat::A8 => ffi::BLFormat::BL_FORMAT_A8,
        }) as u32
    }
}

pub struct Image {
    core: BLImageCore,
}

impl Image {
    pub fn new(width: i32, height: i32, format: ImageFormat) -> Result<Self> {
        unsafe {
            let mut core = std::mem::uninitialized();

            errcode_to_result(ffi::blImageInitAs(&mut core, width, height, format.into()))
                .map(|_| Image { core })
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { ffi::blImageReset(&mut self.core) };
    }
}
