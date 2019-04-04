#[macro_use]
mod macros;

pub mod array;
pub mod codec;
pub mod context;
pub mod error;
pub mod format;
pub mod image;
pub mod path;

pub(in crate) trait ImplType: Sized {
    type CoreType;
    const IMPL_TYPE_ID: usize;

    #[inline]
    fn none() -> &'static Self::CoreType {
        debug_assert!(Self::IMPL_TYPE_ID < ffi::BLImplType::BL_IMPL_TYPE_COUNT as usize);
        unsafe { &*(&ffi::blNone[Self::IMPL_TYPE_ID] as *const _ as *const _) }
    }
}
