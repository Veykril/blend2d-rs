#[macro_use]
mod macros;

pub mod codec;
pub mod context;
pub mod error;
pub mod format;
pub mod image;
pub mod path;

pub(in crate) trait ImplType {
    type Type;
    const IDX: usize;

    fn none() -> &'static Self::Type {
        debug_assert!(Self::IDX < ffi::BLImplType::BL_IMPL_TYPE_COUNT as usize);
        unsafe { &*(&ffi::blNone[Self::IDX] as *const _ as *const _) }
    }
}
