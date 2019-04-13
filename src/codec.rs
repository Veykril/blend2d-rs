use std::ffi::{CStr, CString};

use crate::{
    error::{errcode_to_result, Result},
    variant::WrappedBlCore,
};
use core::ptr;
use std::borrow::Cow;

use crate::array::Array;
use ffi::BLImageCodecFeatures::*;
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
    pub(in crate) core: ffi::BLImageCodecCore,
}

unsafe impl WrappedBlCore for ImageCodec {
    type Core = ffi::BLImageCodecCore;
}

impl ImageCodec {
    #[inline]
    pub fn new() -> Self {
        ImageCodec {
            core: unsafe {
                *crate::variant::none(ffi::BLImplType::BL_IMPL_TYPE_IMAGE_CODEC as usize)
            },
        }
    }

    pub fn new_by_name(name: &str) -> Result<Self> {
        unsafe {
            let mut this = Self::new();
            let name = CString::new(name).expect("Failed to create CString");
            let codecs = ffi::blImageCodecBuiltInCodecs();
            errcode_to_result(ffi::blImageCodecFindByName(
                this.core_mut(),
                codecs,
                name.as_ptr(),
            ))
            .map(|_| this)
        }
    }

    pub fn new_by_data(data: &[u8]) -> Result<Self> {
        unsafe {
            let mut this = Self::new();
            let codecs = ffi::blImageCodecBuiltInCodecs();
            errcode_to_result(ffi::blImageCodecFindByData(
                this.core_mut(),
                codecs,
                data.as_ptr() as *const _,
                data.len(),
            ))
            .map(|_| this)
        }
    }

    pub fn create_decoder(&mut self) -> Result<ImageDecoder> {
        unsafe {
            let mut decoder = ImageDecoder::new();
            errcode_to_result(ffi::blImageCodecCreateDecoder(
                self.core_mut(),
                decoder.core_mut(),
            ))
            .map(|_| decoder)
        }
    }

    pub fn create_encoder(&mut self) -> Result<ImageEncoder> {
        unsafe {
            let mut encoder = ImageEncoder::new();
            errcode_to_result(ffi::blImageCodecCreateEncoder(
                self.core_mut(),
                encoder.core_mut(),
            ))
            .map(|_| encoder)
        }
    }

    pub fn built_in_codecs() -> &'static Array<ImageCodec> {
        unsafe { &*(ffi::blImageCodecBuiltInCodecs() as *const _ as *const _) }
    }

    #[inline]
    pub fn name(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(self.impl_().name).to_string_lossy() }
    }

    #[inline]
    pub fn vendor(&self) -> Cow<str> {
        unsafe { CStr::from_ptr(self.impl_().vendor).to_string_lossy() }
    }

    #[inline]
    pub fn mime_type(&self) -> Cow<str> {
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

impl Default for ImageCodec {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for ImageCodec {
    fn eq(&self, other: &Self) -> bool {
        self.impl_() as *const _ == other.impl_() as *const _
    }
}

impl Clone for ImageCodec {
    fn clone(&self) -> Self {
        let mut core = ffi::BLImageCodecCore {
            impl_: ptr::null_mut(),
        };
        unsafe {
            ffi::blVariantInitWeak(
                &mut core as *mut _ as *mut _,
                &self.core as *const _ as *const _,
            )
        };
        ImageCodec { core }
    }
}

impl Drop for ImageCodec {
    fn drop(&mut self) {
        unsafe { ffi::blImageCodecReset(&mut self.core) };
    }
}

#[repr(transparent)]
pub struct ImageEncoder {
    pub(in crate) core: ffi::BLImageEncoderCore,
}

unsafe impl WrappedBlCore for ImageEncoder {
    type Core = ffi::BLImageEncoderCore;
}

impl ImageEncoder {
    #[inline]
    pub fn new() -> Self {
        ImageEncoder {
            core: unsafe {
                *crate::variant::none(ffi::BLImplType::BL_IMPL_TYPE_IMAGE_ENCODER as usize)
            },
        }
    }

    pub fn restart(&mut self) -> Result<()> {
        unsafe { errcode_to_result(ffi::blImageEncoderRestart(self.core_mut())) }
    }
}

impl Default for ImageEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for ImageEncoder {
    fn eq(&self, other: &Self) -> bool {
        self.impl_() as *const _ == other.impl_() as *const _
    }
}

impl Clone for ImageEncoder {
    fn clone(&self) -> Self {
        let mut core = ffi::BLImageEncoderCore {
            impl_: ptr::null_mut(),
        };
        unsafe {
            ffi::blVariantInitWeak(
                &mut core as *mut _ as *mut _,
                &self.core as *const _ as *const _,
            )
        };
        ImageEncoder { core }
    }
}

impl Drop for ImageEncoder {
    fn drop(&mut self) {
        unsafe { ffi::blImageEncoderReset(&mut self.core) };
    }
}

#[repr(transparent)]
pub struct ImageDecoder {
    pub(in crate) core: ffi::BLImageDecoderCore,
}

unsafe impl WrappedBlCore for ImageDecoder {
    type Core = ffi::BLImageDecoderCore;
}

impl ImageDecoder {
    #[inline]
    pub fn new() -> Self {
        ImageDecoder {
            core: unsafe {
                *crate::variant::none(ffi::BLImplType::BL_IMPL_TYPE_IMAGE_DECODER as usize)
            },
        }
    }

    pub fn restart(&mut self) -> Result<()> {
        unsafe { errcode_to_result(ffi::blImageDecoderRestart(self.core_mut())) }
    }
}

impl Default for ImageDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for ImageDecoder {
    fn eq(&self, other: &Self) -> bool {
        self.impl_() as *const _ == other.impl_() as *const _
    }
}

impl Clone for ImageDecoder {
    fn clone(&self) -> Self {
        let mut core = ffi::BLImageDecoderCore {
            impl_: ptr::null_mut(),
        };
        unsafe {
            ffi::blVariantInitWeak(
                &mut core as *mut _ as *mut _,
                &self.core as *const _ as *const _,
            )
        };
        ImageDecoder { core }
    }
}

impl Drop for ImageDecoder {
    fn drop(&mut self) {
        unsafe { ffi::blImageDecoderReset(&mut self.core) };
    }
}
