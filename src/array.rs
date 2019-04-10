use crate::{
    bl_range,
    error::{errcode_to_result, Result},
    variant::WrappedBlCore,
};
use core::{fmt, marker::PhantomData, ops, ptr, slice};

#[repr(transparent)]
pub struct Array<T: ArrayType> {
    pub(in crate) core: ffi::BLArrayCore,
    _pd: PhantomData<T>,
}

unsafe impl<T: ArrayType> WrappedBlCore for Array<T> {
    type Core = ffi::BLArrayCore;
}

impl<T: ArrayType> Array<T> {
    pub fn new() -> Self {
        Array {
            core: unsafe { *crate::variant::none(T::IMPL_IDX) },
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
        unsafe { ffi::blArrayResize(&mut self.core, n.min(self.len()), ptr::null()) };
    }

    pub fn resize(&mut self, fill: &[T]) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blArrayResize(
                &mut self.core,
                fill.len(),
                fill.as_ptr() as *const _ as *const _,
            ))
        }
    }

    pub fn remove(&mut self, index: usize) -> Result<()> {
        unsafe { errcode_to_result(ffi::blArrayRemoveIndex(&mut self.core, index)) }
    }

    pub fn remove_range<R: ops::RangeBounds<usize>>(&mut self, range: R) -> Result<()> {
        unsafe { errcode_to_result(ffi::blArrayRemoveRange(&mut self.core, &bl_range(range))) }
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

    pub fn extend_from_slice(&mut self, data: &[T]) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blArrayAppendView(
                self.core_mut(),
                data.as_ptr() as *const _,
                data.len(),
            ))
        }
    }

    pub fn insert_from_slice(&mut self, index: usize, data: &[T]) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blArrayInsertView(
                self.core_mut(),
                index,
                data.as_ptr() as *const _,
                data.len(),
            ))
        }
    }

    pub fn replace_from_slice<R: ops::RangeBounds<usize>>(
        &mut self,
        range: R,
        data: &[T],
    ) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blArrayReplaceView(
                self.core_mut(),
                &bl_range(range),
                data.as_ptr() as *const _,
                data.len(),
            ))
        }
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

impl<T: ArrayType> Borrow<[T]> for Array<T> {
    fn borrow(&self) -> &[T] {
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

impl<T: ArrayType> fmt::Debug for Array<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &*self)
    }
}

impl<T: ArrayType> Drop for Array<T> {
    fn drop(&mut self) {
        self.reset()
    }
}

impl<T> Array<T>
where
    T: ArrayType + WrappedBlCore,
{
    pub fn push(&mut self, item: &T) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blArrayAppendItem(
                self.core_mut(),
                item.core() as *const _ as *const _,
            ))
        }
    }
    pub fn insert(&mut self, index: usize, item: &T) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blArrayInsertItem(
                self.core_mut(),
                index,
                item.core() as *const _ as *const _,
            ))
        }
    }
    pub fn repalce(&mut self, index: usize, item: &T) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blArrayReplaceItem(
                self.core_mut(),
                index,
                item.core() as *const _ as *const _,
            ))
        }
    }
}

// Macro-zone ahead, you have been warned

macro_rules! impl_array_val_ops {
    ($( $append:ident, $insert:ident, $replace:ident for $($ty:ty),*);*$(;)*) => {
        $(
            $(
                impl Array<$ty> {
                    pub fn push(&mut self, item: $ty) -> Result<()> {
                        unsafe {
                            errcode_to_result(ffi::$append(
                                self.core_mut(),
                                item as _,
                            ))
                        }
                    }

                    pub fn insert(&mut self, index: usize, item: $ty) -> Result<()> {
                        unsafe {
                            errcode_to_result(ffi::$insert(
                                self.core_mut(),
                                index,
                                item as _,
                            ))
                        }
                    }

                    pub fn replace(&mut self, index: usize, item: $ty) -> Result<()> {
                        unsafe {
                            errcode_to_result(ffi::$replace(
                                self.core_mut(),
                                index,
                                item as _,
                            ))
                        }
                    }
                }
            )+
        )*
    }
}

impl_array_val_ops! {
    blArrayAppendU8,  blArrayInsertU8,  blArrayInsertU8   for i8,  u8;
    blArrayAppendU16, blArrayInsertU16, blArrayInsertU16  for i16, u16;
    blArrayAppendU32, blArrayInsertU32, blArrayInsertU32  for i32, u32;
    blArrayAppendU64, blArrayInsertU64, blArrayInsertU64  for i64, u64;
    blArrayAppendF32, blArrayInsertF32, blArrayInsertF32  for f32;
    blArrayAppendF64, blArrayInsertF64, blArrayInsertF64  for f64;
}

#[cfg(target_pointer_width = "32")]
impl_array_val_ops!(blArrayAppendU32, blArrayInsertU32, blArrayInsertU32 for isize, usize);
#[cfg(target_pointer_width = "64")]
impl_array_val_ops!(blArrayAppendU64, blArrayInsertU64, blArrayInsertU64 for isize, usize);

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
            )+
        )*
    }
}

use ffi::BLImplType::*;
#[cfg(target_pointer_width = "32")]
const BL_IMPL_TYPE_ARRAY_USIZE: usize = BL_IMPL_TYPE_ARRAY_U32 as usize;
#[cfg(target_pointer_width = "32")]
const BL_IMPL_TYPE_ARRAY_ISIZE: usize = BL_IMPL_TYPE_ARRAY_I32 as usize;
#[cfg(target_pointer_width = "64")]
const BL_IMPL_TYPE_ARRAY_USIZE: usize = BL_IMPL_TYPE_ARRAY_U64 as usize;
#[cfg(target_pointer_width = "64")]
const BL_IMPL_TYPE_ARRAY_ISIZE: usize = BL_IMPL_TYPE_ARRAY_I64 as usize;

impl<T> ArrayType for *const T {
    const IMPL_IDX: usize = BL_IMPL_TYPE_ARRAY_USIZE;
}
impl<T> ArrayType for *mut T {
    const IMPL_IDX: usize = BL_IMPL_TYPE_ARRAY_USIZE;
}
impl<T> ArrayType for &T {
    const IMPL_IDX: usize = BL_IMPL_TYPE_ARRAY_USIZE;
}
impl<T> ArrayType for &mut T {
    const IMPL_IDX: usize = BL_IMPL_TYPE_ARRAY_USIZE;
}

impl_array_type! {
    impl i8 = BL_IMPL_TYPE_ARRAY_I8;
    impl u8 = BL_IMPL_TYPE_ARRAY_U8;
    impl i16 = BL_IMPL_TYPE_ARRAY_I16;
    impl u16 = BL_IMPL_TYPE_ARRAY_U16;
    impl i32 = BL_IMPL_TYPE_ARRAY_I32;
    impl u32 = BL_IMPL_TYPE_ARRAY_U32;
    impl i64 = BL_IMPL_TYPE_ARRAY_I64;
    impl u64 = BL_IMPL_TYPE_ARRAY_U64;
    impl isize = BL_IMPL_TYPE_ARRAY_ISIZE;
    impl usize = BL_IMPL_TYPE_ARRAY_USIZE;
    impl f32 = BL_IMPL_TYPE_ARRAY_F32;
    impl f64 = BL_IMPL_TYPE_ARRAY_F64;
}
use crate::{codec::ImageCodec, context::Context, geometry::Path, image::Image};
/* E0119, clashes with `impl<T> ArrayType for &T {[...]`
Specialization please (╯°□°）╯︵ ┻━┻
impl<T: WrappedBlCore> ArrayType for T {
  const IMPL_IDX: usize = BL_IMPL_TYPE_ARRAY_VAR as usize;
}
so we have to unfortunately go with a manual macro implementation */
impl_array_type! {
    impl Path, Image, ImageCodec, Context = BL_IMPL_TYPE_ARRAY_VAR;
}
use crate::Tag;
use std::borrow::Borrow;
impl_array_type! {
    impl Tag = BL_IMPL_TYPE_ARRAY_STRUCT_4;
}
