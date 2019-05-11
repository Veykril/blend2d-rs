use core::{fmt, ptr, str};
use std::ffi::CStr;

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
    pub fn find_by_name<'a>(codecs: &'a Array<ImageCodec>, name: &str) -> Option<&'a ImageCodec> {
        codecs.iter().find(|codec| codec.name() == name)
    }

    /// Searches for an image codec in the array by the given data.
    pub fn find_by_data<R: AsRef<[u8]>>(
        codecs: &Array<ImageCodec>,
        data: R,
    ) -> Option<&ImageCodec> {
        let mut best_score = 0;
        let mut candidate = None;
        let data = data.as_ref();

        for codec in codecs {
            let score = unsafe {
                ffi::blImageCodecInspectData(codec.core(), data.as_ptr() as *const _, data.len())
            };
            if score > best_score {
                best_score = score;
                candidate = Some(codec)
            }
        }
        candidate
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
    pub fn built_in_codecs() -> Array<ImageCodec> {
        let mut core = ffi::BLArrayCore {
            impl_: ptr::null_mut(),
        };
        unsafe { ffi::blImageCodecArrayInitBuiltInCodecs(&mut core) };
        WrappedBlCore::from_core(core)
    }

    /// The codec's name.
    #[inline]
    pub fn name(&self) -> &str {
        unsafe { str::from_utf8_unchecked(CStr::from_ptr(self.impl_().name).to_bytes()) }
    }

    /// The codec's vendor.
    #[inline]
    pub fn vendor(&self) -> &str {
        unsafe { str::from_utf8_unchecked(CStr::from_ptr(self.impl_().vendor).to_bytes()) }
    }

    /// The codec's mime-type.
    #[inline]
    pub fn mime_type(&self) -> &str {
        unsafe { str::from_utf8_unchecked(CStr::from_ptr(self.impl_().mimeType).to_bytes()) }
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
        let codecs = ImageCodec::built_in_codecs();
        let codec = codecs.first().unwrap();
        let encoder = codec.create_encoder().unwrap();
        assert_eq!(codec, encoder.codec());
    }

    #[test]
    fn test_decoder_creation() {
        let codecs = ImageCodec::built_in_codecs();
        let codec = codecs.first().unwrap();
        let decoder = codec.create_decoder().unwrap();
        assert_eq!(codec, decoder.codec());
    }
}
