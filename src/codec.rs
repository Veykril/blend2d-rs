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
    const IMPL_TYPE_INDEX: usize = crate::variant::ImplType::ImageCodec as usize;

    #[inline]
    fn from_core(core: Self::Core) -> Self {
        ImageCodec { core }
    }
}

impl ImageCodec {
    /// Searches for an image codec in the array by the given name.
    pub fn find_by_name(codecs: &Array<ImageCodec>, name: &str) -> Result<Self> {
        unsafe {
            let mut this = ImageCodec::from_core(*Self::none());
            let name = CString::new(name).expect("Failed to create CString");
            errcode_to_result(ffi::blImageCodecFindByName(
                this.core_mut(),
                codecs.core(),
                name.as_ptr(),
            ))
            .map(|_| this)
        }
    }

    /// Searches for an image codec in the array by the given data.
    #[inline]
    pub fn find_by_data<R: AsRef<[u8]>>(codecs: &Array<ImageCodec>, data: R) -> Result<Self> {
        unsafe {
            let mut this = ImageCodec::from_core(*Self::none());
            errcode_to_result(ffi::blImageCodecFindByData(
                this.core_mut(),
                codecs.core(),
                data.as_ref().as_ptr() as *const _,
                data.as_ref().len(),
            ))
            .map(|_| this)
        }
    }

    /// Creates an [`ImageDecoder`] for this codec.
    #[inline]
    pub fn create_decoder(&self) -> Result<ImageDecoder> {
        unsafe {
            let mut decoder = ImageDecoder {
                core: *ImageDecoder::none(),
            };
            errcode_to_result(ffi::blImageCodecCreateDecoder(
                self.core(),
                decoder.core_mut(),
            ))
            .map(|_| decoder)
        }
    }

    /// Creates an [`ImageEncoder`] for this codec.
    #[inline]
    pub fn create_encoder(&self) -> Result<ImageEncoder> {
        unsafe {
            let mut encoder = ImageEncoder {
                core: *ImageEncoder::none(),
            };
            errcode_to_result(ffi::blImageCodecCreateEncoder(
                self.core(),
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

    /// Returns a static reference of the blend2d builtin codecs.
    #[inline]
    pub fn built_in_codecs() -> &'static Array<ImageCodec> {
        unsafe { &*(ffi::blImageCodecBuiltInCodecs() as *const _ as *const _) }
    }

    /// The codec's name.
    #[inline]
    pub fn name(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.impl_().name).to_string_lossy() }
    }

    /// The codec's vendor.
    #[inline]
    pub fn vendor(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.impl_().vendor).to_string_lossy() }
    }

    /// The codec's mime-type.
    #[inline]
    pub fn mime_type(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.impl_().mimeType).to_string_lossy() }
    }

    /// The codec's file extensions.
    pub fn extensions(&self) -> impl Iterator<Item = &str> {
        unsafe {
            CStr::from_ptr(self.impl_().extensions)
                .to_str()
                .unwrap_or_default()
                .split('|')
        }
    }

    /// The codec's features.
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
            .field("features", &self.features())
            .finish()
    }
}

impl PartialEq for ImageCodec {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.impl_equals(other)
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
    const IMPL_TYPE_INDEX: usize = crate::variant::ImplType::ImageEncoder as usize;

    #[inline]
    fn from_core(core: Self::Core) -> Self {
        ImageEncoder { core }
    }
}

impl ImageEncoder {
    /// The codec this encoder belongs to.
    #[inline]
    pub fn codec(&self) -> &ImageCodec {
        unsafe { &*(&self.impl_().codec as *const _ as *const _) }
    }

    #[inline]
    pub fn restart(&mut self) -> Result<()> {
        unsafe { errcode_to_result(ffi::blImageEncoderRestart(self.core_mut())) }
    }

    /// The last encoding result.
    #[inline]
    pub fn last_result(&self) -> Result<()> {
        errcode_to_result(self.impl_().lastResult)
    }
}

impl PartialEq for ImageEncoder {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.impl_equals(other)
    }
}

impl fmt::Debug for ImageEncoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImageEncoder")
            .field("codec", &self.codec())
            .finish()
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
    const IMPL_TYPE_INDEX: usize = crate::variant::ImplType::ImageDecoder as usize;

    #[inline]
    fn from_core(core: Self::Core) -> Self {
        ImageDecoder { core }
    }
}

impl ImageDecoder {
    /// The codec this decoder belongs to.
    #[inline]
    pub fn codec(&self) -> &ImageCodec {
        unsafe { &*(&self.impl_().codec as *const _ as *const _) }
    }

    #[inline]
    pub fn restart(&mut self) -> Result<()> {
        unsafe { errcode_to_result(ffi::blImageDecoderRestart(self.core_mut())) }
    }

    /// The last decoding result.
    #[inline]
    pub fn last_result(&self) -> Result<()> {
        errcode_to_result(self.impl_().lastResult)
    }
}

impl PartialEq for ImageDecoder {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.impl_equals(other)
    }
}

impl fmt::Debug for ImageDecoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImageDecoder")
            .field("codec", &self.codec())
            .finish()
    }
}

impl Drop for ImageDecoder {
    fn drop(&mut self) {
        unsafe { ffi::blImageDecoderReset(&mut self.core) };
    }
}

#[cfg(test)]
mod test_codec {
    use crate::codec::ImageCodec;

    #[test]
    fn test_built_in_codecs() {
        assert_ne!(ImageCodec::built_in_codecs().len(), 0)
    }

    #[test]
    fn test_encoder_creation() {
        let codec = ImageCodec::built_in_codecs().first().unwrap();
        let encoder = codec.create_encoder().unwrap();
        assert_eq!(codec, encoder.codec());
    }

    #[test]
    fn test_decoder_creation() {
        let codec = ImageCodec::built_in_codecs().first().unwrap();
        let encoder = codec.create_decoder().unwrap();
        assert_eq!(codec, encoder.codec());
    }
}
