#[repr(i32)]
#[derive(Debug)]
pub enum ImageFormat {
    None = ffi::BLFormat::BL_FORMAT_NONE,
    PRgb32 = ffi::BLFormat::BL_FORMAT_PRGB32,
    XRgb32 = ffi::BLFormat::BL_FORMAT_XRGB32,
    A8 = ffi::BLFormat::BL_FORMAT_A8,
}

impl From<u32> for ImageFormat {
    fn from(val: u32) -> Self {
        match val as ffi::BLFormat::Type {
            ffi::BLFormat::BL_FORMAT_PRGB32 => ImageFormat::PRgb32,
            ffi::BLFormat::BL_FORMAT_XRGB32 => ImageFormat::XRgb32,
            ffi::BLFormat::BL_FORMAT_A8 => ImageFormat::A8,
            _ => ImageFormat::None,
        }
    }
}

impl From<ImageFormat> for u32 {
    fn from(format: ImageFormat) -> u32 {
        format as u32
    }
}
