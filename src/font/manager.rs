use std::fmt;

use crate::variant::WrappedBlCore;

/// Font Manager
#[repr(transparent)]
pub struct FontManager {
    core: ffi::BLFontManagerCore,
}

unsafe impl WrappedBlCore for FontManager {
    type Core = ffi::BLFontManagerCore;
    const IMPL_TYPE_INDEX: usize = crate::variant::ImplType::FontManager as usize;

    fn from_core(core: Self::Core) -> Self {
        FontManager { core }
    }
}

impl PartialEq for FontManager {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::blFontManagerEquals(self.core(), other.core()) }
    }
}

impl Clone for FontManager {
    fn clone(&self) -> Self {
        Self::from_core(self.init_weak())
    }
}

impl Drop for FontManager {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::blFontManagerReset(&mut self.core) };
    }
}

impl fmt::Debug for FontManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FontManager").finish()
    }
}
