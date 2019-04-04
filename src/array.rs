use crate::{
    error::{errcode_to_result, Result},
    ImplType,
};
use core::{ops, ptr, slice};
use std::marker::PhantomData;

#[repr(transparent)]
pub struct Array<T: ArrayType> {
    pub(in crate) core: ffi::BLArrayCore,
    _pd: PhantomData<T>,
}

impl<T: ArrayType> Array<T> {
    pub fn new() -> Self {
        Array {
            core: ffi::BLArrayCore {
                impl_: Self::none().impl_,
            },
            _pd: PhantomData,
        }
    }

    pub fn clear(&mut self) {
        unsafe { ffi::blArrayClear(&mut self.core) };
    }

    pub fn shrink_to_fit(&mut self) {
        unsafe { ffi::blArrayShrink(&mut self.core) };
    }

    #[inline]
    pub fn reserve(&mut self, n: usize) {
        self.try_reserve(n).unwrap();
    }

    pub fn try_reserve(&mut self, n: usize) -> Result<()> {
        unsafe { errcode_to_result(ffi::blArrayReserve(&mut self.core, n)) }
    }

    pub fn truncate(&mut self, n: usize) {
        unsafe { ffi::blArrayResize(&mut self.core, n.min(self.len()), ptr::null_mut()) };
    }

    /*TODO resize
    pub fn resize(&mut self, n: usize, fill: &T) -> Result<()> {
        unsafe { errcode_to_result(ffi::blArrayResize(&mut self.core, n, &fill.core)) }
    }
    */

    pub fn remove(&mut self, index: usize) -> Result<()> {
        unsafe { errcode_to_result(ffi::blArrayRemoveIndex(&mut self.core, index)) }
    }

    pub fn as_slice(&self) -> &[T] {
        self
    }

    pub fn len(&self) -> usize {
        unsafe { self.impl_().__bindgen_anon_1.__bindgen_anon_1.size as usize }
    }

    pub fn capacity(&self) -> usize {
        self.impl_().capacity as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn reset(&mut self) {
        unsafe { ffi::blArrayReset(&mut self.core) };
    }

    #[inline]
    fn impl_(&self) -> &ffi::BLArrayImpl {
        unsafe { &*self.core.impl_ }
    }

    #[inline]
    fn data_ptr(&self) -> *const T {
        unsafe { self.impl_().__bindgen_anon_1.__bindgen_anon_1.data as *const _ }
    }
}

impl<T: ArrayType> AsRef<[T]> for Array<T> {
    fn as_ref(&self) -> &[T] {
        self
    }
}

impl<T: ArrayType> ops::Deref for Array<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.data_ptr(), self.len()) }
    }
}

impl<T, I> ops::Index<I> for Array<T>
where
    T: ArrayType,
    I: slice::SliceIndex<[T]>,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        ops::Index::index(&**self, index)
    }
}

impl<T: ArrayType> Default for Array<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ArrayType> PartialEq for Array<T> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::blArrayEquals(&self.core, &other.core) }
    }
}

impl<T: ArrayType> Clone for Array<T> {
    fn clone(&self) -> Self {
        let mut core = ffi::BLArrayCore {
            impl_: ptr::null_mut(),
        };
        unsafe {
            ffi::blVariantInitWeak(
                &mut core as *mut _ as *mut _,
                &self.core as *const _ as *const _,
            )
        };
        Array {
            core,
            _pd: PhantomData,
        }
    }
}

impl<T: ArrayType> Drop for Array<T> {
    fn drop(&mut self) {
        self.reset()
    }
}

impl<T: ArrayType> ImplType for Array<T> {
    type CoreType = ffi::BLArrayCore;
    const IMPL_TYPE_ID: usize = T::IMPL_IDX;
}

pub trait ArrayType: Sized {
    const IMPL_IDX: usize;
}

macro_rules! impl_array_type {
    ($(impl $($ty:ty),* = $idx:expr);*$(;)*) => {
        $(
            $(
                impl ArrayType for $ty {
                    const IMPL_IDX: usize = $idx as usize;
                }
            )*
        )*
    }
}

use ffi::BLImplType::*;
#[cfg(target_arch = "32")]
impl_array_type!(impl *const T, *mut T, &T, &mut T = BL_IMPL_TYPE_ARRAY_U32);
#[cfg(target_arch = "64")]
impl_array_type!(impl *const T, *mut T, &T, &mut T = BL_IMPL_TYPE_ARRAY_U64);
impl_array_type! {
    impl i8 = BL_IMPL_TYPE_ARRAY_I8;
    impl u8 = BL_IMPL_TYPE_ARRAY_U8;
    impl i16 = BL_IMPL_TYPE_ARRAY_I16;
    impl u16 = BL_IMPL_TYPE_ARRAY_U16;
    impl i32 = BL_IMPL_TYPE_ARRAY_I32;
    impl u32 = BL_IMPL_TYPE_ARRAY_U32;
    impl i64 = BL_IMPL_TYPE_ARRAY_I64;
    impl u64 = BL_IMPL_TYPE_ARRAY_U64;
    impl f32 = BL_IMPL_TYPE_ARRAY_F32;
    impl f64 = BL_IMPL_TYPE_ARRAY_F64;

}
/*
//
#[repr(transparent)]
struct Struct<T: Sized>(T);

impl<T> ArrayType for Struct<T> {
    const IMPL_IDX: usize = match mem::size_of::<T>() {
        _ => unimplemented!(),
    };
}
*/
