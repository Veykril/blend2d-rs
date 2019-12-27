///! The contents of this module imitate the internal BLVariant structure
use bitflags::bitflags;

use crate::array::{Array, ArrayType};
use crate::error::expect_mem_err;
use crate::image::Image;
use crate::path::Path;
use crate::pattern::Pattern;
use crate::region::Region;
use crate::util::cast_ref;

use ffi::BLImplType::*;
bl_enum! {
    pub enum ImplType {
        Null                 = BL_IMPL_TYPE_NULL,
        ArrayVar             = BL_IMPL_TYPE_ARRAY_VAR,
        ArrayI8              = BL_IMPL_TYPE_ARRAY_I8,
        ArrayU8              = BL_IMPL_TYPE_ARRAY_U8,
        ArrayI16             = BL_IMPL_TYPE_ARRAY_I16,
        ArrayU16             = BL_IMPL_TYPE_ARRAY_U16,
        ArrayI32             = BL_IMPL_TYPE_ARRAY_I32,
        ArrayU32             = BL_IMPL_TYPE_ARRAY_U32,
        ArrayI64             = BL_IMPL_TYPE_ARRAY_I64,
        ArrayU64             = BL_IMPL_TYPE_ARRAY_U64,
        ArrayF32             = BL_IMPL_TYPE_ARRAY_F32,
        ArrayF64             = BL_IMPL_TYPE_ARRAY_F64,
        ArrayStruct1         = BL_IMPL_TYPE_ARRAY_STRUCT_1,
        ArrayStruct2         = BL_IMPL_TYPE_ARRAY_STRUCT_2,
        ArrayStruct3         = BL_IMPL_TYPE_ARRAY_STRUCT_3,
        ArrayStruct4         = BL_IMPL_TYPE_ARRAY_STRUCT_4,
        ArrayStruct6         = BL_IMPL_TYPE_ARRAY_STRUCT_6,
        ArrayStruct8         = BL_IMPL_TYPE_ARRAY_STRUCT_8,
        ArrayStruct10        = BL_IMPL_TYPE_ARRAY_STRUCT_10,
        ArrayStruct12        = BL_IMPL_TYPE_ARRAY_STRUCT_12,
        ArrayStruct16        = BL_IMPL_TYPE_ARRAY_STRUCT_16,
        ArrayStruct20        = BL_IMPL_TYPE_ARRAY_STRUCT_20,
        ArrayStruct24        = BL_IMPL_TYPE_ARRAY_STRUCT_24,
        ArrayStruct32        = BL_IMPL_TYPE_ARRAY_STRUCT_32,
        BitArray             = BL_IMPL_TYPE_BIT_ARRAY,
        BitSet               = BL_IMPL_TYPE_BIT_SET,
        String               = BL_IMPL_TYPE_STRING,
        Path                 = BL_IMPL_TYPE_PATH,
        Region               = BL_IMPL_TYPE_REGION,
        Image                = BL_IMPL_TYPE_IMAGE,
        ImageCodec           = BL_IMPL_TYPE_IMAGE_CODEC,
        ImageDecoder         = BL_IMPL_TYPE_IMAGE_DECODER,
        ImageEncoder         = BL_IMPL_TYPE_IMAGE_ENCODER,
        Gradient             = BL_IMPL_TYPE_GRADIENT,
        Pattern              = BL_IMPL_TYPE_PATTERN,
        Context              = BL_IMPL_TYPE_CONTEXT,
        Font                 = BL_IMPL_TYPE_FONT,
        FontFace             = BL_IMPL_TYPE_FONT_FACE,
        FontData             = BL_IMPL_TYPE_FONT_DATA,
        FontManager          = BL_IMPL_TYPE_FONT_MANAGER,
        FontFeatureOptions   = BL_IMPL_TYPE_FONT_FEATURE_OPTIONS,
        FontVariationOptions = BL_IMPL_TYPE_FONT_VARIATION_OPTIONS,
    }
    Default => Null
}

bitflags! {
    pub struct ImplTraits: u8 {
        const NULL      = ffi::BLImplTraits::BL_IMPL_TRAIT_NULL as u8;
        const VIRTUAL   = ffi::BLImplTraits::BL_IMPL_TRAIT_VIRT as u8;
        const MUTABLE   = ffi::BLImplTraits::BL_IMPL_TRAIT_MUTABLE as u8;
        const IMMUTABLE = ffi::BLImplTraits::BL_IMPL_TRAIT_IMMUTABLE as u8;
        const EXTERNAL  = ffi::BLImplTraits::BL_IMPL_TRAIT_EXTERNAL as u8;
        const FOREIGN   = ffi::BLImplTraits::BL_IMPL_TRAIT_FOREIGN as u8;
    }
}

/// Marker trait for virtual function table struct/
pub trait VTable {}

impl VTable for ffi::BLContextVirt {}
impl VTable for ffi::BLFontDataVirt {}
impl VTable for ffi::BLFontManagerVirt {}
impl VTable for ffi::BLImageCodecVirt {}
impl VTable for ffi::BLImageDecoderVirt {}
impl VTable for ffi::BLImageEncoderVirt {}

pub unsafe trait BlVariantImpl: Sized {
    type VTable;

    #[inline]
    fn virt(&self) -> &Self::VTable
    where
        Self::VTable: VTable,
    {
        unsafe { &*(self.as_variant_impl().__bindgen_anon_1.virt as *const _ as *const _) }
    }

    #[inline]
    fn ref_count(&self) -> usize {
        self.as_variant_impl().refCount
    }

    #[inline]
    fn impl_type(&self) -> ImplType {
        u32::from(self.as_variant_impl().implType).into()
    }

    #[inline]
    fn impl_traits(&self) -> ImplTraits {
        ImplTraits::from_bits_truncate(self.as_variant_impl().implTraits)
    }

    #[inline]
    fn as_variant_impl(&self) -> &ffi::BLVariantImpl {
        unsafe { cast_ref(self) }
    }
}

unsafe impl BlVariantImpl for ffi::BLArrayImpl {
    type VTable = ();
}
unsafe impl BlVariantImpl for ffi::BLContextImpl {
    type VTable = ffi::BLContextVirt;
}
unsafe impl BlVariantImpl for ffi::BLGradientImpl {
    type VTable = ();
}
unsafe impl BlVariantImpl for ffi::BLFontImpl {
    type VTable = ();
}
unsafe impl BlVariantImpl for ffi::BLFontDataImpl {
    type VTable = ffi::BLFontDataVirt;
}
unsafe impl BlVariantImpl for ffi::BLFontFaceImpl {
    type VTable = ffi::BLFontFaceVirt;
}
unsafe impl BlVariantImpl for ffi::BLFontManagerImpl {
    type VTable = ffi::BLFontManagerVirt;
}
unsafe impl BlVariantImpl for ffi::BLGlyphBufferImpl {
    type VTable = ();
}
unsafe impl BlVariantImpl for ffi::BLImageImpl {
    type VTable = ();
}
unsafe impl BlVariantImpl for ffi::BLImageCodecImpl {
    type VTable = ffi::BLImageCodecVirt;
}
unsafe impl BlVariantImpl for ffi::BLImageDecoderImpl {
    type VTable = ffi::BLImageDecoderVirt;
}
unsafe impl BlVariantImpl for ffi::BLImageEncoderImpl {
    type VTable = ffi::BLImageDecoderVirt;
}
unsafe impl BlVariantImpl for ffi::BLPathImpl {
    type VTable = ();
}
unsafe impl BlVariantImpl for ffi::BLPatternImpl {
    type VTable = ();
}
unsafe impl BlVariantImpl for ffi::BLRegionImpl {
    type VTable = ();
}
unsafe impl BlVariantImpl for ffi::BLStringImpl {
    type VTable = ();
}
unsafe impl BlVariantImpl for ffi::BLVariantImpl {
    type VTable = ();
}

pub unsafe trait BlVariantCore: Sized {
    type Impl: BlVariantImpl;

    #[inline]
    fn as_variant_core(&self) -> &ffi::BLVariantCore {
        unsafe { cast_ref(self) }
    }

    #[inline]
    fn impl_(&self) -> &Self::Impl {
        unsafe { &*(self.as_variant_core().impl_ as *const _) }
    }

    #[inline]
    fn impl_mut(&mut self) -> &mut Self::Impl {
        unsafe { &mut *(self.as_variant_core().impl_ as *mut _) }
    }

    #[inline]
    fn init_weak(&self, other: &mut Self) {
        unsafe { ffi::blVariantInitWeak(other as *mut _ as *mut _, self as *const _ as *const _) };
    }
}

unsafe impl BlVariantCore for ffi::BLArrayCore {
    type Impl = ffi::BLArrayImpl;
}
unsafe impl BlVariantCore for ffi::BLContextCore {
    type Impl = ffi::BLContextImpl;
}
unsafe impl BlVariantCore for ffi::BLGradientCore {
    type Impl = ffi::BLGradientImpl;
}
unsafe impl BlVariantCore for ffi::BLFontCore {
    type Impl = ffi::BLFontImpl;
}
unsafe impl BlVariantCore for ffi::BLFontDataCore {
    type Impl = ffi::BLFontDataImpl;
}
unsafe impl BlVariantCore for ffi::BLFontFaceCore {
    type Impl = ffi::BLFontFaceImpl;
}
unsafe impl BlVariantCore for ffi::BLFontManagerCore {
    type Impl = ffi::BLFontManagerImpl;
}
unsafe impl BlVariantCore for ffi::BLGlyphBufferCore {
    type Impl = ffi::BLGlyphBufferImpl;
}
unsafe impl BlVariantCore for ffi::BLImageCore {
    type Impl = ffi::BLImageImpl;
}
unsafe impl BlVariantCore for ffi::BLImageCodecCore {
    type Impl = ffi::BLImageCodecImpl;
}
unsafe impl BlVariantCore for ffi::BLImageDecoderCore {
    type Impl = ffi::BLImageDecoderImpl;
}
unsafe impl BlVariantCore for ffi::BLImageEncoderCore {
    type Impl = ffi::BLImageEncoderImpl;
}
unsafe impl BlVariantCore for ffi::BLPathCore {
    type Impl = ffi::BLPathImpl;
}
unsafe impl BlVariantCore for ffi::BLPatternCore {
    type Impl = ffi::BLPatternImpl;
}
unsafe impl BlVariantCore for ffi::BLRegionCore {
    type Impl = ffi::BLRegionImpl;
}
unsafe impl BlVariantCore for ffi::BLStringCore {
    type Impl = ffi::BLStringImpl;
}
unsafe impl BlVariantCore for ffi::BLVariantCore {
    type Impl = ffi::BLVariantImpl;
}

/// Implementing type must be either:
///     #[repr(transparent)] and its only field may be a struct that contains a
///     pointer to a BlxxxxImpl
///
///     #[repr(C)] and its first field must be a pointer to its core's
///     [`Impl`] type
pub unsafe trait WrappedBlCore: Sized {
    type Core: BlVariantCore;
    const IMPL_TYPE_INDEX: usize;

    fn from_core(core: Self::Core) -> Self;

    /// The default implementation reinterprets &self as &Self::Core.
    #[inline]
    fn core(&self) -> &Self::Core {
        unsafe { cast_ref(self) }
    }

    /// The default implementation reinterprets &mut self as &mut Self::Core.
    #[inline]
    fn core_mut(&mut self) -> &mut Self::Core {
        unsafe { &mut *(self as *mut _ as *mut _) }
    }

    #[inline]
    fn impl_(&self) -> &<Self::Core as BlVariantCore>::Impl {
        self.core().impl_()
    }

    #[inline]
    fn impl_mut(&mut self) -> &mut <Self::Core as BlVariantCore>::Impl {
        self.core_mut().impl_mut()
    }

    /// Checks whether the wrapped implementation is a none object.
    #[inline]
    fn is_none(&self) -> bool {
        self.impl_().impl_traits().contains(ImplTraits::NULL)
    }

    /// Retrieves the none version of Self::Core
    #[inline]
    fn none() -> &'static Self::Core {
        unsafe { cast_ref(&ffi::blNone[Self::IMPL_TYPE_INDEX]) }
    }

    /// Checks equality of the objects implementations by comparing the pointer.
    #[inline]
    fn impl_equals(&self, other: &Self) -> bool {
        self.impl_() as *const _ == other.impl_() as *const _
    }

    /// Creates a weak refcount copy.
    #[inline]
    fn init_weak(&self) -> Self::Core {
        let mut other = unsafe { core::mem::zeroed() };
        self.core().init_weak(&mut other);
        other
    }
}

type BlAssignDeep<C> = unsafe extern "C" fn(*mut C, *const C) -> ffi::BLResult;

/// A trait for deep cloning the object. This is different from [Clone] for
/// blObjects in the regard that normal cloning only creates a weak
/// reference-counted clone.
pub trait DeepClone
where
    Self: WrappedBlCore,
    <Self as WrappedBlCore>::Core: Copy + 'static,
{
    #[doc(hidden)]
    const ASSIGN_DEEP: BlAssignDeep<<Self as WrappedBlCore>::Core>;
    /// Returns a deeply cloned copy of the value.
    fn clone_deep(&self) -> Self {
        let mut new = Self::from_core(*Self::none());
        unsafe { expect_mem_err(Self::ASSIGN_DEEP(new.core_mut(), self.core())) };
        new
    }
}

impl DeepClone for Image {
    const ASSIGN_DEEP: BlAssignDeep<Self::Core> = ffi::blImageAssignDeep;
}
impl DeepClone for Path {
    const ASSIGN_DEEP: BlAssignDeep<Self::Core> = ffi::blPathAssignDeep;
}

impl<T> DeepClone for Array<T>
where
    T: ArrayType,
    Array<T>: WrappedBlCore,
    <Array<T> as WrappedBlCore>::Core: Copy + 'static,
{
    const ASSIGN_DEEP: BlAssignDeep<Self::Core> = blArrayAssignDeep;
}

#[allow(non_snake_case)]
#[inline]
unsafe extern "C" fn blArrayAssignDeep<C>(dst: *mut C, src: *const C) -> ffi::BLResult {
    ffi::blArrayAssignDeep(dst as _, src as _)
}

impl DeepClone for Region {
    const ASSIGN_DEEP: BlAssignDeep<Self::Core> = ffi::blRegionAssignDeep;
}
impl DeepClone for Pattern {
    const ASSIGN_DEEP: BlAssignDeep<Self::Core> = ffi::blPatternAssignDeep;
}
