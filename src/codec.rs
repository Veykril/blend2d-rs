use std::ffi::CString;

use crate::{
    bl_impl::WrappedBlCore,
    error::{errcode_to_result, Result},
};

#[repr(transparent)]
pub struct ImageCodec {
    pub(in crate) core: ffi::BLImageCodecCore,
}

unsafe impl WrappedBlCore for ImageCodec {
    type Core = ffi::BLImageCore;
}

impl ImageCodec {
    pub fn new() -> Self {
        ImageCodec {
            core: unsafe {
                *crate::bl_impl::none(ffi::BLImplType::BL_IMPL_TYPE_IMAGE_CODEC as usize)
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
