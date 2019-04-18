use core::fmt;
use std::{
    borrow::Cow,
    ffi::{CStr, CString},
};

use ffi::BLImageCodecFeatures::*;

use crate::{
    array::Array,
    error::{errcode_to_result, Result},
    variant::WrappedBlCore,
};

bl_enum! {
    pub enum ImageCodecFeatures {
        Read       = BL_IMAGE_CODEC_FEATURE_READ,
        Write      = BL_IMAGE_CODEC_FEATURE_WRITE,
        Lossless   = BL_IMAGE_CODEC_FEATURE_LOSSLESS,
        Lossy      = BL_IMAGE_CODEC_FEATURE_LOSSY,
        MultiFrame = BL_IMAGE_CODEC_FEATURE_MULTI_FRAME,
        Iptc       = BL_IMAGE_CODEC_FEATURE_IPTC,
        Exif       = BL_IMAGE_CODEC_FEATURE_EXIF,
        Xmp        = BL_IMAGE_CODEC_FEATURE_XMP,
    }
    Default => Read
}

#[repr(transparent)]
pub struct ImageCodec {
    core: ffi::BLImageCodecCore,
}

unsafe impl WrappedBlCore for ImageCodec {
    type Core = ffi::BLImageCodecCore;
    const IMPL_TYPE_INDEX: usize = ffi::BLImplType::BL_IMPL_TYPE_IMAGE_CODEC as usize;
}

impl ImageCodec {
    pub fn find_by_name(codecs: &Array<ImageCodec>, name: &str) -> Result<Self> {
        unsafe {
            let mut this = ImageCodec {
                core: *Self::none(),
            };
            let name = CString::new(name).expect("Failed to create CString");
            errcode_to_result(ffi::blImageCodecFindByName(
                this.core_mut(),
                codecs.core(),
                name.as_ptr(),
            ))
            .map(|_| this)
        }
    }

    #[inline]
    pub fn find_by_data<R: AsRef<[u8]>>(codecs: &Array<ImageCodec>, data: R) -> Result<Self> {
        unsafe {
            let mut this = ImageCodec {
                core: *Self::none(),
            };
            errcode_to_result(ffi::blImageCodecFindByData(
                this.core_mut(),
                codecs.core(),
                data.as_ref().as_ptr() as *const _,
                data.as_ref().len(),
            ))
            .map(|_| this)
        }
    }

    #[inline]
    pub fn create_decoder(&mut self) -> Result<ImageDecoder> {
        unsafe {
            let mut decoder = ImageDecoder {
                core: *ImageDecoder::none(),
            };
            errcode_to_result(ffi::blImageCodecCreateDecoder(
                self.core_mut(),
                decoder.core_mut(),
            ))
            .map(|_| decoder)
        }
    }

    #[inline]
    pub fn create_encoder(&mut self) -> Result<ImageEncoder> {
        unsafe {
            let mut encoder = ImageEncoder {
                core: *ImageEncoder::none(),
            };
            errcode_to_result(ffi::blImageCodecCreateEncoder(
                self.core_mut(),
                encoder.core_mut(),
            ))
            .map(|_| encoder)
        }
    }

    #[inline]
    pub fn inspect_data<R: AsRef<[u8]>>(&self, data: R) -> u32 {
        unsafe {
            ffi::blImageCodecInspectData(
                self.core(),
                data.as_ref().as_ptr() as *const _,
                data.as_ref().len(),
            )
        }
    }

    #[inline]
    pub fn built_in_codecs() -> &'static Array<ImageCodec> {
        unsafe { &*(ffi::blImageCodecBuiltInCodecs() as *const _ as *const _) }
    }

    #[inline]
    pub fn name(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.impl_().name).to_string_lossy() }
    }

    #[inline]
    pub fn vendor(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.impl_().vendor).to_string_lossy() }
    }

    #[inline]
    pub fn mime_type(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.impl_().mimeType).to_string_lossy() }
    }

    pub fn extensions(&self) -> impl Iterator<Item = &str> {
        unsafe {
            CStr::from_ptr(self.impl_().extensions)
                .to_str()
                .unwrap_or_default()
                .split('|')
        }
    }

    #[inline]
    pub fn features(&self) -> ImageCodecFeatures {
        (self.impl_().features as u32).into()
    }
}

impl fmt::Debug for ImageCodec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImageCodec")
            .field("name", &self.name())
            .field("vendor", &self.vendor())
            .field("mime_type", &self.mime_type())
            .finish()
    }
}

impl PartialEq for ImageCodec {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.impl_() as *const _ == other.impl_() as *const _
    }
}

impl Drop for ImageCodec {
    fn drop(&mut self) {
        unsafe { ffi::blImageCodecReset(&mut self.core) };
    }
}

#[repr(transparent)]
pub struct ImageEncoder {
    core: ffi::BLImageEncoderCore,
}

unsafe impl WrappedBlCore for ImageEncoder {
    type Core = ffi::BLImageEncoderCore;
    const IMPL_TYPE_INDEX: usize = ffi::BLImplType::BL_IMPL_TYPE_IMAGE_ENCODER as usize;
}

impl ImageEncoder {
    #[inline]
    pub fn restart(&mut self) -> Result<()> {
        unsafe { errcode_to_result(ffi::blImageEncoderRestart(self.core_mut())) }
    }

    #[inline]
    pub fn last_result(&self) -> Result<()> {
        errcode_to_result(self.impl_().lastResult)
    }
}

impl PartialEq for ImageEncoder {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.impl_() as *const _ == other.impl_() as *const _
    }
}

impl Drop for ImageEncoder {
    fn drop(&mut self) {
        unsafe { ffi::blImageEncoderReset(&mut self.core) };
    }
}

#[repr(transparent)]
pub struct ImageDecoder {
    core: ffi::BLImageDecoderCore,
}

unsafe impl WrappedBlCore for ImageDecoder {
    type Core = ffi::BLImageDecoderCore;
    const IMPL_TYPE_INDEX: usize = ffi::BLImplType::BL_IMPL_TYPE_IMAGE_DECODER as usize;
}

impl ImageDecoder {
    #[inline]
    pub fn restart(&mut self) -> Result<()> {
        unsafe { errcode_to_result(ffi::blImageDecoderRestart(self.core_mut())) }
    }

    #[inline]
    pub fn last_result(&self) -> Result<()> {
        errcode_to_result(self.impl_().lastResult)
    }
}

impl PartialEq for ImageDecoder {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.impl_() as *const _ == other.impl_() as *const _
    }
}

impl Drop for ImageDecoder {
    fn drop(&mut self) {
        unsafe { ffi::blImageDecoderReset(&mut self.core) };
    }
}
