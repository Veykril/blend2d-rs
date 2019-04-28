use bitflags::bitflags;

use core::{fmt, ptr, slice};
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

use ffi::BLImageInfoFlags::*;
bitflags! {
    pub struct ImageInfoFlags: u32 {
        const PROGRESSIVE = BL_IMAGE_INFO_FLAG_PROGRESSIVE as u32;
    }
}

use ffi::BLImageScaleFilter::*;
bl_enum! {
    pub enum ImageScaleFilter {
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
    Default => Nearest
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImageScaleOptions {
    user_func: ffi::BLImageScaleUserFunc,
    user_data: *mut std::os::raw::c_void,
    pub radius: f64,
    pub b: f64,
    pub c: f64,
    _data: f64,
}

impl Default for ImageScaleOptions {
    fn default() -> Self {
        ImageScaleOptions {
            user_func: None,
            user_data: ptr::null_mut(),
            radius: 2.0,
            b: 1.0 / 3.0,
            c: 1.0 / 3.0,
            _data: 0.0,
        }
    }
}

#[repr(transparent)]
pub struct Image {
    core: BLImageCore,
}

unsafe impl WrappedBlCore for Image {
    type Core = ffi::BLImageCore;
    const IMPL_TYPE_INDEX: usize = crate::variant::ImplType::Image as usize;

    #[inline]
    fn from_core(core: Self::Core) -> Image {
        Image { core }
    }
}

impl Image {
    #[inline]
    pub fn new(width: i32, height: i32, format: ImageFormat) -> Result<Image> {
        unsafe {
            let mut this = Image::from_core(*Self::none());
            errcode_to_result(ffi::blImageCreate(
                this.core_mut(),
                width,
                height,
                format.into(),
            ))
            .map(|_| this)
        }
    }

    /* FIXME figure out a solution for the lifetime issue
    #[inline]
    pub fn new_external(
        width: i32,
        height: i32,
        format: ImageFormat,
        data: &'a mut [u8],
        stride: isize,
    ) -> Result<Image<'a>> {
        unsafe {
            let mut this = Image::from_core(*Self::none());
            errcode_to_result(ffi::blImageCreateFromData(
                this.core_mut(),
                width,
                height,
                format.into(),
                data.as_mut_ptr() as *mut _,
                stride,
                None,
                ptr::null_mut(),
            ))
            .map(|_| this)
        }
    }*/

    pub fn from_data<R: AsRef<[u8]>>(
        width: i32,
        height: i32,
        format: ImageFormat,
        data: &R,
        codecs: &Array<ImageCodec>,
    ) -> Result<Image> {
        let mut this = Self::new(width, height, format)?;
        unsafe {
            errcode_to_result(ffi::blImageReadFromData(
                this.core_mut(),
                data.as_ref().as_ptr() as *const _,
                data.as_ref().len(),
                codecs.core(),
            ))
            .map(|_| this)
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P, codecs: &Array<ImageCodec>) -> Result<Image> {
        unsafe {
            let mut this = Image::from_core(*Self::none());
            let path =
                CString::new(path.as_ref().to_string_lossy().as_bytes()).expect("Invalid Path");
            errcode_to_result(ffi::blImageReadFromFile(
                this.core_mut(),
                path.as_ptr(),
                codecs.core(),
            ))
            .map(|_| this)
        }
    }

    #[inline]
    pub fn format(&self) -> ImageFormat {
        (self.impl_().format as u32).into()
    }

    #[inline]
    pub fn size(&self) -> SizeI {
        let ffi::BLSizeI { w, h } = self.impl_().size;
        SizeI { w, h }
    }

    #[inline]
    pub fn width(&self) -> i32 {
        self.size().w
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.size().h
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
                    flags: ImageInfoFlags::from_bits_truncate(data.flags),
                }
            })
        }
    }

    #[inline]
    pub fn scale(
        &mut self,
        size: SizeI,
        filter: ImageScaleFilter,
        options: Option<&ImageScaleOptions>,
    ) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blImageScale(
                self.core_mut(),
                self.core(),
                &size as *const _ as *const _,
                filter as u32,
                options.map_or(ptr::null(), |opt| opt as *const _ as *const _),
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

    #[inline]
    pub fn write_to_data(&self, dst: &mut Array<u8>, codec: &ImageCodec) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blImageWriteToData(
                self.core(),
                dst.core_mut(),
                codec.core(),
            ))
        }
    }
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Image")
            .field("size", &self.size())
            .field("format", &self.format())
            .finish()
    }
}

impl PartialEq for Image {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::blImageEquals(self.core(), other.core()) }
    }
}

impl Clone for Image {
    #[inline]
    fn clone(&self) -> Image {
        let mut new = Image::from_core(*Self::none());
        unsafe { ffi::blImageAssignDeep(new.core_mut(), self.core()) };
        new
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { ffi::blImageReset(&mut self.core) };
    }
}

#[derive(Debug)]
pub struct ImageData<'a> {
    pub data: &'a [u8],
    pub stride: isize,
    pub size: (i32, i32),
    pub format: ImageFormat,
    pub flags: ImageInfoFlags,
}

#[cfg(test)]
mod test_codec {
    use crate::{geometry::SizeI, image::Image};

    #[test]
    fn test_image_err_on_zero_size() {
        assert!(Image::new(0, 100, Default::default()).is_err());
        assert!(Image::new(100, 0, Default::default()).is_err());
        assert!(Image::new(0, 0, Default::default()).is_err());
    }

    #[test]
    fn test_image_scale() {
        let new_size = SizeI { w: 100, h: 100 };
        let mut image = Image::new(50, 50, Default::default()).unwrap();
        image.scale(new_size, Default::default(), None).unwrap();
        assert_eq!(image.size(), new_size);
    }
}
