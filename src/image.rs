use core::slice;
use std::{ffi::CString, path::Path};

use ffi::{self, BLImageCore};

use crate::{
    codec::ImageCodec,
    error::{errcode_to_result, Result},
    format::ImageFormat,
    variant::WrappedBlCore,
};

#[repr(transparent)]
pub struct Image {
    pub(in crate) core: BLImageCore,
}

unsafe impl WrappedBlCore for Image {
    type Core = ffi::BLImageCore;
}

impl Image {
    pub fn new(width: i32, height: i32, format: ImageFormat) -> Result<Self> {
        unsafe {
            let mut core = std::mem::uninitialized();

            errcode_to_result(ffi::blImageInitAs(&mut core, width, height, format.into()))
                .map(|_| Image { core })
        }
    }

    pub fn size(&self) -> (i32, i32) {
        let ffi::BLSizeI { w, h } = unsafe { (*self.core.impl_).size };
        (w, h)
    }

    pub fn create(&mut self, width: i32, height: i32, format: ImageFormat) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blImageCreate(
                &mut self.core,
                width,
                height,
                format.into(),
            ))
        }
    }

    pub fn data(&self) -> Result<ImageData> {
        unsafe {
            let mut data = std::mem::uninitialized();
            errcode_to_result(ffi::blImageGetData(&self.core, &mut data)).map(|_| {
                let ffi::BLSizeI { w, h } = data.size;
                ImageData {
                    data: slice::from_raw_parts(data.pixelData as *mut _, (w * h) as usize),
                    stride: data.stride,
                    size: (w, h),
                    format: data.format.into(),
                    flags: data.flags,
                }
            })
        }
    }

    pub fn write_to_file<P: AsRef<Path>>(&mut self, path: P, codec: &ImageCodec) -> Result<()> {
        unsafe {
            let path =
                CString::new(path.as_ref().to_string_lossy().as_bytes()).expect("Invalid Path");
            errcode_to_result(ffi::blImageWriteToFile(
                &mut self.core,
                path.as_ptr(),
                &codec.core,
            ))
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { ffi::blImageReset(&mut self.core) };
    }
}

#[derive(Debug)]
pub struct ImageData<'a> {
    data: &'a [u8],
    stride: isize,
    size: (i32, i32),
    format: ImageFormat,
    flags: u32,
}
