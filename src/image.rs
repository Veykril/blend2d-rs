use core::{ptr, slice};
use std::{ffi::CString, path::Path};

use ffi::{self, BLImageCore};

use crate::{
    array::Array,
    codec::ImageCodec,
    error::{errcode_to_result, Result},
    format::ImageFormat,
    geometry::SizeI,
    variant::WrappedBlCore,
};

use bitflags::bitflags;

use ffi::BLImageInfoFlags::*;
bitflags! {
    pub struct ImageInfoFlags: u32 {
        const PROGRESSIVE = BL_IMAGE_INFO_FLAG_PROGRESSIVE as u32;
    }
}

use ffi::BLImageScaleFilter::*;
bl_enum! {
    pub enum ImageScaleFilter {
        None     = BL_IMAGE_SCALE_FILTER_NONE,
        Nearest  = BL_IMAGE_SCALE_FILTER_NEAREST,
        Bilinear = BL_IMAGE_SCALE_FILTER_BILINEAR,
        Bicubic  = BL_IMAGE_SCALE_FILTER_BICUBIC,
        Bell     = BL_IMAGE_SCALE_FILTER_BELL,
        Gauss    = BL_IMAGE_SCALE_FILTER_GAUSS,
        Hermite  = BL_IMAGE_SCALE_FILTER_HERMITE,
        Hanning  = BL_IMAGE_SCALE_FILTER_HANNING,
        Catrom   = BL_IMAGE_SCALE_FILTER_CATROM,
        Bessel   = BL_IMAGE_SCALE_FILTER_BESSEL,
        Sinc     = BL_IMAGE_SCALE_FILTER_SINC,
        Lanczos  = BL_IMAGE_SCALE_FILTER_LANCZOS,
        Blackman = BL_IMAGE_SCALE_FILTER_BLACKMAN,
        Mitchell = BL_IMAGE_SCALE_FILTER_MITCHELL,
        User     = BL_IMAGE_SCALE_FILTER_USER,
    }
    Default => None
}

#[repr(transparent)]
pub struct Image {
    core: BLImageCore,
}

unsafe impl WrappedBlCore for Image {
    type Core = ffi::BLImageCore;
}

impl Image {
    pub fn new() -> Self {
        Image {
            core: *Self::none(ffi::BLImplType::BL_IMPL_TYPE_IMAGE as usize),
        }
    }

    pub fn new_with(width: i32, height: i32, format: ImageFormat) -> Result<Self> {
        unsafe {
            let mut this = Self::new();
            errcode_to_result(ffi::blImageInitAs(
                this.core_mut(),
                width,
                height,
                format.into(),
            ))
            .map(|_| this)
        }
    }

    pub fn size(&self) -> SizeI {
        let ffi::BLSizeI { w, h } = self.impl_().size;
        SizeI { w, h }
    }

    pub fn width(&self) -> i32 {
        self.size().w
    }

    pub fn height(&self) -> i32 {
        self.size().h
    }

    pub fn create(&mut self, width: i32, height: i32, format: ImageFormat) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blImageCreate(
                self.core_mut(),
                width,
                height,
                format.into(),
            ))
        }
    }

    // FIXME lifetime of data must outlive the image
    fn create_from_data(
        &mut self,
        width: i32,
        height: i32,
        format: ImageFormat,
        data: &mut [u8],
        stride: isize,
    ) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blImageCreateFromData(
                self.core_mut(),
                width,
                height,
                format.into(),
                data.as_mut_ptr() as *mut _,
                stride,
                None,
                ptr::null_mut(),
            ))
        }
    }

    pub fn data(&self) -> Result<ImageData<'_>> {
        unsafe {
            let mut data = std::mem::zeroed();
            errcode_to_result(ffi::blImageGetData(self.core(), &mut data)).map(|_| {
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

    pub fn read_from_file<P: AsRef<Path>>(
        &mut self,
        path: P,
        codecs: &Array<ImageCodec>,
    ) -> Result<()> {
        unsafe {
            let path =
                CString::new(path.as_ref().to_string_lossy().as_bytes()).expect("Invalid Path");
            errcode_to_result(ffi::blImageReadFromFile(
                self.core_mut(),
                path.as_ptr(),
                codecs.core(),
            ))
        }
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, path: P, codec: &ImageCodec) -> Result<()> {
        unsafe {
            let path =
                CString::new(path.as_ref().to_string_lossy().as_bytes()).expect("Invalid Path");
            errcode_to_result(ffi::blImageWriteToFile(
                self.core(),
                path.as_ptr(),
                codec.core(),
            ))
        }
    }
}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        self.impl_() as *const _ == other.impl_() as *const _
    }
}

impl Default for Image {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Image {
    fn clone(&self) -> Self {
        Image {
            core: self.init_weak(),
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
