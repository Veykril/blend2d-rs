use bitflags::bitflags;

use ffi::BLFormat::*;
bl_enum! {
    pub enum ImageFormat {
        PRgb32 = BL_FORMAT_PRGB32,
        XRgb32 = BL_FORMAT_XRGB32,
        A8     = BL_FORMAT_A8,
    }
    Default => PRgb32
}

use ffi::BLFormatFlags;
bitflags! {
    pub struct FormatFlags: u32 {
        const RGB = BLFormatFlags::BL_FORMAT_FLAG_RGB as u32;
        const ALPHA = BLFormatFlags::BL_FORMAT_FLAG_ALPHA as u32;
        const RGBA = BLFormatFlags::BL_FORMAT_FLAG_RGBA as u32;
        const LUM = BLFormatFlags::BL_FORMAT_FLAG_LUM as u32;
        const LUMA = BLFormatFlags::BL_FORMAT_FLAG_LUMA as u32;
        const INDEXED = BLFormatFlags::BL_FORMAT_FLAG_INDEXED as u32;
        const PREMULTIPLIED = BLFormatFlags::BL_FORMAT_FLAG_PREMULTIPLIED as u32;
        const BYTE_SWAP = BLFormatFlags::BL_FORMAT_FLAG_BYTE_SWAP as u32;
    }
}
