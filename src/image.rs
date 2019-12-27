//! Image loading and handling.
use bitflags::bitflags;

use std::ffi::CString;
use std::path::Path;
use std::{fmt, mem, ops, ptr, slice};

use ffi::{self, BLImageCore};

use crate::array::Array;
use crate::codec::ImageCodec;
use crate::error::{errcode_to_result, expect_mem_err, Result};
use crate::geometry::{SizeD, SizeI};
use crate::variant::WrappedBlCore;

const IMAGE_SCALE_OPTIONS_ZEROED: ffi::BLImageScaleOptions = ffi::BLImageScaleOptions {
    userFunc: None,
    userData: ptr::null_mut(),
    radius: 0.0,
    __bindgen_anon_1: ffi::BLImageScaleOptions__bindgen_ty_1 {
        data: [0.0, 0.0, 0.0],
    },
};

use ffi::BLFormat::*;
bl_enum! {
    /// Pixel format.
    pub enum ImageFormat {
        /// 32-bit premultiplied ARGB pixel format (8-bit components).
        PRgb32 = BL_FORMAT_PRGB32,
        /// 32-bit (X)RGB pixel format (8-bit components, alpha ignored).
        XRgb32 = BL_FORMAT_XRGB32,
        /// 8-bit alpha-only pixel format.
        A8     = BL_FORMAT_A8,
    }
    Default => PRgb32
}

use ffi::BLFormatFlags;
bitflags! {
    /// Pixel format flags.
    pub struct FormatFlags: u32 {
        /// Pixel format provides RGB components.
        const RGB           = BLFormatFlags::BL_FORMAT_FLAG_RGB as u32;
        /// Pixel format provides only alpha component.
        const ALPHA         = BLFormatFlags::BL_FORMAT_FLAG_ALPHA as u32;
        /// A combination of RGB | ALPHA.
        const RGBA          = BLFormatFlags::BL_FORMAT_FLAG_RGBA as u32;
        /// Pixel format provides LUM component (and not RGB components).
        const LUM           = BLFormatFlags::BL_FORMAT_FLAG_LUM as u32;
        /// A combination of BL_FORMAT_FLAG_LUM | BL_FORMAT_FLAG_ALPHA.
        const LUMA          = BLFormatFlags::BL_FORMAT_FLAG_LUMA as u32;
        /// Indexed pixel format the requres a palette (I/O only).
        const INDEXED       = BLFormatFlags::BL_FORMAT_FLAG_INDEXED as u32;
        /// RGB components are premultiplied by alpha component.
        const PREMULTIPLIED = BLFormatFlags::BL_FORMAT_FLAG_PREMULTIPLIED as u32;
        /// Pixel format doesn't use native byte-order (I/O only).
        const BYTE_SWAP     = BLFormatFlags::BL_FORMAT_FLAG_BYTE_SWAP as u32;
        /// Pixel components are byte aligned (all 8bpp).
        const BYTE_ALIGNED  = BLFormatFlags::BL_FORMAT_FLAG_BYTE_ALIGNED as u32;

        /// Pixel has some undefined bits that represent no information.
        ///
        /// For example a 32-bit XRGB pixel has 8 undefined bits that are usually set
        /// to all ones so the format can be interpreted as premultiplied RGB as well.
        /// There are other formats like 16_0555 where the bit has no information and
        /// is usually set to zero. Blend2D doesn't rely on the content of such bits.
        const UNDEFINED_BITS = BLFormatFlags::BL_FORMAT_FLAG_UNDEFINED_BITS as u32;
        /// Convenience flag that contains either zero or `BYTE_SWAP`
        /// depending on host byte order. Little endian hosts have this flag set to
        /// zero and big endian hosts to `BYTE_SWAP`.
        ///
        /// This is not a real flag that you can test, it's only provided for
        /// convenience to define little endian pixel formats.
        const FLAG_LE = BLFormatFlags::BL_FORMAT_FLAG_LE as u32;

        /// Convenience flag that contains either zero or `BYTE_SWAP`
        /// depending on host byte order. Big endian hosts have this flag set to
        /// zero and little endian hosts to `BYTE_SWAP`.
        ///
        /// This is not a real flag that you can test, it's only provided for
        /// convenience to define big endian pixel formats.
        const FLAG_BE = BLFormatFlags::BL_FORMAT_FLAG_BE as u32;
    }
}

use ffi::BLImageInfoFlags::*;
bitflags! {
    /// Flags used by [`ImageInfo`].
    pub struct ImageInfoFlags: u32 {
        /// progressive mode.
        const PROGRESSIVE = BL_IMAGE_INFO_FLAG_PROGRESSIVE as u32;
    }
}

/// Filter type used by [`Image::scale`].
///
/// [`Image::scale`]: struct.Image.html#method.scale
#[derive(Copy, Clone, Debug)]
pub enum ImageScaleFilter {
    /// Nearest neighbor filter (radius 1.0).
    Nearest,
    /// Bilinear filter (radius 1.0).
    Bilinear,
    /// Bicubic filter (radius 2.0).
    Bicubic,
    /// Bell filter (radius 1.5).
    Bell,
    /// Gauss filter (radius 2.0).
    Gauss,
    /// Hermite filter (radius 1.0).
    Hermite,
    /// Hanning filter (radius 1.0).
    Hanning,
    /// Catrom filter (radius 2.0).
    Catrom,
    /// Bessel filter (radius 3.2383).
    Bessel,
    /// Sinc filter (default radius 2.0).
    Sinc { radius: f64 },
    /// Lanczos filter (default radius 2.0).
    Lanczos { radius: f64 },
    /// Blackman filter (default radius 2.0).
    Blackman { radius: f64 },
    /// Mitchell filter (radius 2.0).
    Mitchell { b: f64, c: f64 },
}

impl ImageScaleFilter {
    #[inline]
    fn filter(&self) -> u32 {
        use ffi::BLImageScaleFilter::*;
        (match self {
            ImageScaleFilter::Nearest => BL_IMAGE_SCALE_FILTER_NEAREST,
            ImageScaleFilter::Bilinear => BL_IMAGE_SCALE_FILTER_BILINEAR,
            ImageScaleFilter::Bicubic => BL_IMAGE_SCALE_FILTER_BICUBIC,
            ImageScaleFilter::Bell => BL_IMAGE_SCALE_FILTER_BELL,
            ImageScaleFilter::Gauss => BL_IMAGE_SCALE_FILTER_GAUSS,
            ImageScaleFilter::Hermite => BL_IMAGE_SCALE_FILTER_HERMITE,
            ImageScaleFilter::Hanning => BL_IMAGE_SCALE_FILTER_HANNING,
            ImageScaleFilter::Catrom => BL_IMAGE_SCALE_FILTER_CATROM,
            ImageScaleFilter::Bessel => BL_IMAGE_SCALE_FILTER_BESSEL,
            ImageScaleFilter::Sinc { .. } => BL_IMAGE_SCALE_FILTER_SINC,
            ImageScaleFilter::Lanczos { .. } => BL_IMAGE_SCALE_FILTER_LANCZOS,
            ImageScaleFilter::Blackman { .. } => BL_IMAGE_SCALE_FILTER_BLACKMAN,
            ImageScaleFilter::Mitchell { .. } => BL_IMAGE_SCALE_FILTER_MITCHELL,
        }) as u32
    }

    #[inline]
    fn into_options(self) -> Option<ffi::BLImageScaleOptions> {
        match self {
            ImageScaleFilter::Nearest
            | ImageScaleFilter::Bilinear
            | ImageScaleFilter::Bicubic
            | ImageScaleFilter::Bell
            | ImageScaleFilter::Gauss
            | ImageScaleFilter::Hermite
            | ImageScaleFilter::Hanning
            | ImageScaleFilter::Catrom
            | ImageScaleFilter::Bessel => None,
            ImageScaleFilter::Sinc { radius }
            | ImageScaleFilter::Lanczos { radius }
            | ImageScaleFilter::Blackman { radius } => Some(ffi::BLImageScaleOptions {
                radius,
                ..IMAGE_SCALE_OPTIONS_ZEROED
            }),
            ImageScaleFilter::Mitchell { b, c } => Some(ffi::BLImageScaleOptions {
                __bindgen_anon_1: ffi::BLImageScaleOptions__bindgen_ty_1 { data: [b, c, 0.0] },
                ..IMAGE_SCALE_OPTIONS_ZEROED
            }),
        }
    }
}

/// A 2D raster image.
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
    /// Creates a new empty image with the specified dimensions and image
    /// format. Note: The pixel data of the newly created image is
    /// uninitialized.
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

    /// Attempts to create a new image with the specified dimensions and image
    /// format by decoding the data with the given codec.
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
        let mut this = Image::from_core(*Self::none());
        let path = CString::new(path.as_ref().to_string_lossy().into_owned().into_bytes()).unwrap();
        unsafe {
            errcode_to_result(ffi::blImageReadFromFile(
                this.core_mut(),
                path.as_ptr(),
                codecs.core(),
            ))
            .map(|_| this)
        }
    }

    /// This image's format.
    #[inline]
    pub fn format(&self) -> ImageFormat {
        u32::from(self.impl_().format).into()
    }

    /// This image's dimensions.
    #[inline]
    pub fn size(&self) -> SizeI {
        let ffi::BLSizeI { w, h } = self.impl_().size;
        SizeI { w, h }
    }

    /// This image's width.
    #[inline]
    pub fn width(&self) -> i32 {
        self.size().w
    }

    /// This image's height.
    #[inline]
    pub fn height(&self) -> i32 {
        self.size().h
    }

    /// Returns an [`ImageData`] instance containing most of this image's
    /// information.
    pub fn data(&self) -> ImageData<'_> {
        unsafe {
            let mut data = std::mem::zeroed();
            ffi::blImageGetData(self.core(), &mut data);
            let ffi::BLSizeI { w, h } = data.size;
            ImageData {
                data: slice::from_raw_parts(
                    data.pixelData as *mut _,
                    (h as isize * data.stride) as usize,
                ),
                stride: data.stride as isize / w as isize,
                size: (w, h),
                format: data.format.into(),
                flags: ImageInfoFlags::from_bits_truncate(data.flags),
            }
        }
    }

    pub fn convert(&mut self, format: ImageFormat) -> Result<()> {
        unsafe { errcode_to_result(ffi::blImageConvert(self.core_mut(), format.into())) }
    }

    pub fn scale(&mut self, size: SizeI, filter: ImageScaleFilter) -> Result<()> {
        unsafe {
            let opts = filter.into_options();
            errcode_to_result(ffi::blImageScale(
                self.core_mut(),
                self.core(),
                &size as *const _ as *const _,
                filter.filter(),
                opts.as_ref().map_or(ptr::null(), |opt| opt as *const _),
            ))
        }
    }

    // FIXME: Allow the closure to return an error
    #[inline]
    pub fn scale_user<F>(&mut self, size: SizeI, radius: f64, mut filter: F) -> Result<()>
    where
        F: for<'a> Fn(&'a mut [f64], &'a [f64]),
    {
        unsafe extern "C" fn user_func_callback<F>(
            dst: *mut f64,
            t_array: *const f64,
            n: usize,
            func: *const F,
        ) -> ffi::BLResult
        where
            F: for<'a> Fn(&'a mut [f64], &'a [f64]),
        {
            (&*func)(
                slice::from_raw_parts_mut(dst, n),
                slice::from_raw_parts(t_array, n),
            );
            0
        }
        unsafe {
            errcode_to_result(ffi::blImageScale(
                self.core_mut(),
                self.core(),
                &size as *const _ as *const _,
                ffi::BLImageScaleFilter::BL_IMAGE_SCALE_FILTER_USER as u32,
                &ffi::BLImageScaleOptions {
                    radius,
                    userFunc: Some(mem::transmute::<*const (), _>(
                        user_func_callback::<F> as *const (),
                    )),
                    userData: &mut filter as *mut _ as *mut _,
                    ..IMAGE_SCALE_OPTIONS_ZEROED
                },
            ))
        }
    }

    /// Writes the image to the file at the given path.
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

    /// Writes the image to the given [`Array`].
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

impl ops::Deref for Image {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let mut data = std::mem::zeroed();
            expect_mem_err(ffi::blImageGetData(self.core(), &mut data));
            slice::from_raw_parts(
                data.pixelData as *const _,
                (data.size.h as isize * data.stride) as usize,
            )
        }
    }
}

impl ops::DerefMut for Image {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let mut data = std::mem::zeroed();
            expect_mem_err(ffi::blImageMakeMutable(self.core_mut(), &mut data));
            slice::from_raw_parts_mut(
                data.pixelData as *mut _,
                (data.size.h as isize * data.stride) as usize,
            )
        }
    }
}

impl AsRef<[u8]> for Image {
    fn as_ref(&self) -> &[u8] {
        self
    }
}

impl AsMut<[u8]> for Image {
    fn as_mut(&mut self) -> &mut [u8] {
        self
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
        Self::from_core(self.init_weak())
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { ffi::blImageReset(&mut self.core) };
    }
}

/// A struct containing information about an image.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ImageData<'a> {
    pub data: &'a [u8],
    pub stride: isize,
    pub size: (i32, i32),
    pub format: ImageFormat,
    pub flags: ImageInfoFlags,
}

/// Image information provided by image codecs.
#[derive(Debug)]
pub struct ImageInfo {
    /// Image size.
    pub size: SizeI,
    /// Pixel density per one meter, can contain fractions.
    pub density: SizeD,
    /// Image flags.
    pub flags: ImageInfoFlags,
    /// Image depth.
    pub depth: u16,
    /// Number of planes.
    pub plane_count: u16,
    /// Number of frames (0 = unknown/unspecified).
    pub frame_count: u64,
    /// Image format (as understood by codec).
    format: [u8; 16],
    /// Image compression (as understood by codec).
    compression: [u8; 16],
}

#[cfg(test)]
mod test_codec {
    use crate::image::ImageScaleFilter;
    use crate::{geometry::SizeI, image::Image, DeepClone};

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
        image
            .scale(new_size, ImageScaleFilter::Blackman { radius: 2.0 })
            .unwrap();
        assert_eq!(image.size(), new_size);
    }

    #[test]
    fn test_image_scale_user_func() {
        let new_size = SizeI { w: 100, h: 100 };
        let mut image = Image::new(50, 50, Default::default()).unwrap();
        let mut image2 = image.clone_deep();
        // nearest filter, but in RUST
        image
            .scale_user(new_size, 1.0, |dst, t_array| {
                for (dst, t) in dst.iter_mut().zip(t_array.iter().copied()) {
                    *dst = if t <= 0.5 { 1.0 } else { 0.0 };
                }
            })
            .unwrap();
        image2.scale(new_size, ImageScaleFilter::Nearest).unwrap();
        assert_eq!(image, image2);
    }

    #[test]
    fn test_image_data() {
        let image = Image::new(50, 50, Default::default()).unwrap();
        let image_data = image.data();
        assert_eq!(image_data.stride, 4);
        assert_eq!(
            image_data.data.to_vec().len() as isize,
            50 * 50 * image_data.stride
        );
    }
}
