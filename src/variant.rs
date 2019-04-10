use crate::vtables::VTable;
use bitflags::bitflags;

bitflags! {
    pub struct ImplTraits: u8 {
        const NULL = ffi::BLImplTraits::BL_IMPL_TRAIT_NULL as u8;
        const VIRTUAL = ffi::BLImplTraits::BL_IMPL_TRAIT_VIRT as u8;
        const EXTERNAL = ffi::BLImplTraits::BL_IMPL_TRAIT_EXTERNAL as u8;
        const FOREIGN = ffi::BLImplTraits::BL_IMPL_TRAIT_FOREIGN as u8;
    }
}

#[inline]
pub(in crate) unsafe fn none<T: Sized>(impl_type: usize) -> &'static T {
    &*(&ffi::blNone[impl_type] as *const _ as *const _)
}

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
    fn impl_traits(&self) -> ImplTraits {
        ImplTraits::from_bits_truncate(self.as_variant_impl().implTraits)
    }

    #[inline]
    fn as_variant_impl(&self) -> &ffi::BLVariantImpl {
        unsafe { &*(self as *const _ as *const _) }
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
unsafe impl BlVariantImpl for ffi::BLFontDataImpl {
    type VTable = ffi::BLFontDataVirt;
}
unsafe impl BlVariantImpl for ffi::BLFontLoaderImpl {
    type VTable = ffi::BLFontLoaderVirt;
}
unsafe impl BlVariantImpl for ffi::BLImageImpl {
    type VTable = ();
}
unsafe impl BlVariantImpl for ffi::BLImageCodecImpl {
    type VTable = ();
}
unsafe impl BlVariantImpl for ffi::BLImageDecoderImpl {
    type VTable = ();
}
unsafe impl BlVariantImpl for ffi::BLImageEncoderImpl {
    type VTable = ();
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
        unsafe { &*(self as *const _ as *const _) }
    }

    #[inline]
    fn impl_(&self) -> &Self::Impl {
        unsafe { &*(self.as_variant_core().impl_ as *const _) }
    }

    #[inline]
    #[allow(clippy::mut_from_ref)]
    fn impl_mut(&self) -> &mut Self::Impl {
        unsafe { &mut *(self.as_variant_core().impl_ as *mut _) }
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
unsafe impl BlVariantCore for ffi::BLFontDataCore {
    type Impl = ffi::BLFontDataImpl;
}
unsafe impl BlVariantCore for ffi::BLFontLoaderCore {
    type Impl = ffi::BLFontLoaderImpl;
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

/// Implementing type must be #[repr(transparent)] and its only field may be a struct that contains
/// a pointer to a BlxxxxImpl
pub unsafe trait WrappedBlCore: Sized {
    type Core: BlVariantCore;

    #[inline]
    fn core(&self) -> &Self::Core {
        unsafe { &*(self as *const _ as *const _) }
    }

    #[inline]
    fn core_mut(&mut self) -> &mut Self::Core {
        unsafe { &mut *(self as *mut _ as *mut _) }
    }

    #[inline]
    fn impl_(&self) -> &<Self::Core as BlVariantCore>::Impl {
        self.core().impl_()
    }

    #[inline]
    #[allow(clippy::mut_from_ref)]
    fn impl_mut(&self) -> &mut <Self::Core as BlVariantCore>::Impl {
        self.core().impl_mut()
    }
}
