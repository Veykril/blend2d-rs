use std::ffi::CString;

use crate::{
    error::{errcode_to_result, Result},
    ImplType,
};

pub struct ImageCodec {
    pub(in crate) core: ffi::BLImageCodecCore,
}

impl ImageCodec {
    pub fn new() -> Self {
        ImageCodec {
            core: ffi::BLImageCodecCore {
                impl_: Self::none().impl_,
            },
        }
    }

    pub fn by_name(name: &str) -> Result<Self> {
        unsafe {
            let mut this = Self::new();
            let name = CString::new(name).expect("Failed to create CString");
            let codecs = ffi::blImageCodecBuiltInCodecs();
            errcode_to_result(ffi::blImageCodecFindByName(
                &mut this.core,
                codecs,
                name.as_ptr(),
            ))
            .map(|_| this)
        }
    }
}

impl Default for ImageCodec {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ImageCodec {
    fn drop(&mut self) {
        unsafe { ffi::blImageCodecReset(&mut self.core) };
    }
}

impl ImplType for ImageCodec {
    type CoreType = ffi::BLImageCodecCore;
    const IMPL_TYPE_ID: usize = ffi::BLImplType::BL_IMPL_TYPE_IMAGE_CODEC as usize;
}
