pub trait VTable {}

impl VTable for ffi::BLContextVirt {}
impl VTable for ffi::BLFontDataVirt {}
impl VTable for ffi::BLFontLoaderVirt {}
