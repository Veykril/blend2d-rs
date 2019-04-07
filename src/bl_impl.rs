use bitflags::bitflags;

bitflags! {
    pub(in crate) struct ImplTraits: u8 {
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

pub(in crate) unsafe trait BlImpl: Sized {
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

unsafe impl BlImpl for ffi::BLArrayImpl {}
unsafe impl BlImpl for ffi::BLContextImpl {}
unsafe impl BlImpl for ffi::BLGradientImpl {}
unsafe impl BlImpl for ffi::BLImageImpl {}
unsafe impl BlImpl for ffi::BLImageCodecImpl {}
unsafe impl BlImpl for ffi::BLImageDecoderImpl {}
unsafe impl BlImpl for ffi::BLImageEncoderImpl {}
unsafe impl BlImpl for ffi::BLPathImpl {}
unsafe impl BlImpl for ffi::BLPatternImpl {}
unsafe impl BlImpl for ffi::BLRegionImpl {}
unsafe impl BlImpl for ffi::BLStringImpl {}
unsafe impl BlImpl for ffi::BLVariantImpl {}

pub(in crate) unsafe trait BlCore: Sized {
    type Impl: BlImpl;

    #[inline]
    fn as_variant_core(&self) -> &ffi::BLVariantCore {
        unsafe { &*(self as *const _ as *const _) }
    }

    #[inline]
    fn impl_(&self) -> &Self::Impl {
        unsafe { &*(self.as_variant_core().impl_ as *const _) }
    }

    #[inline]
    fn impl_mut(&mut self) -> &mut Self::Impl {
        unsafe { &mut *(self.as_variant_core().impl_ as *mut _) }
    }
}

unsafe impl BlCore for ffi::BLArrayCore {
    type Impl = ffi::BLArrayImpl;
}
unsafe impl BlCore for ffi::BLContextCore {
    type Impl = ffi::BLContextImpl;
}
unsafe impl BlCore for ffi::BLGradientCore {
    type Impl = ffi::BLGradientImpl;
}
unsafe impl BlCore for ffi::BLImageCore {
    type Impl = ffi::BLImageImpl;
}
unsafe impl BlCore for ffi::BLImageCodecCore {
    type Impl = ffi::BLImageCodecImpl;
}
unsafe impl BlCore for ffi::BLImageDecoderCore {
    type Impl = ffi::BLImageDecoderImpl;
}
unsafe impl BlCore for ffi::BLImageEncoderCore {
    type Impl = ffi::BLImageEncoderImpl;
}
unsafe impl BlCore for ffi::BLPathCore {
    type Impl = ffi::BLPathImpl;
}
unsafe impl BlCore for ffi::BLPatternCore {
    type Impl = ffi::BLPatternImpl;
}
unsafe impl BlCore for ffi::BLRegionCore {
    type Impl = ffi::BLRegionImpl;
}
unsafe impl BlCore for ffi::BLStringCore {
    type Impl = ffi::BLStringImpl;
}
unsafe impl BlCore for ffi::BLVariantCore {
    type Impl = ffi::BLVariantImpl;
}

/// Implementing type must be #[repr(transparent)] and its only field may be a struct that contains
/// a pointer to a BlxxxxImpl
pub(in crate) unsafe trait WrappedBlCore {
    type Core: BlCore;

    #[inline]
    fn core(&self) -> &Self::Core {
        unsafe { &*(self as *const _ as *const _) }
    }

    #[inline]
    fn core_mut(&mut self) -> &mut Self::Core {
        unsafe { &mut *(self as *mut _ as *mut _) }
    }

    #[inline]
    fn impl_(&self) -> &<Self::Core as BlCore>::Impl {
        self.core().impl_()
    }

    #[inline]
    fn impl_mut(&mut self) -> &mut <Self::Core as BlCore>::Impl {
        self.core_mut().impl_mut()
    }
}
