use core::{fmt, ptr};

use crate::{
    error::expect_mem_err,
    font_defs::{GlyphRun, GlyphRunFlags},
};

pub type GlyphId = u16;

/// Glyph buffer.
///
/// Can hold either text or glyphs and provides basic memory management that is
/// used for text shaping, character to glyph mapping, glyph substitution, and
/// glyph positioning.
///
/// Glyph buffer provides two separate buffers called 'primary' and 'secondary'
/// that serve different purposes during processing. Primary buffer always hold
/// a [`GlyphItem`](../font_defs/struct.GlyphItem.html) array, and secondary
/// buffer is either used as a scratch buffer during glyph substitution or hold
/// glyph positions after the processing is complete and glyph positions were
/// calculated.
pub struct GlyphBuffer {
    pub(in crate) core: ffi::BLGlyphBufferCore,
}

impl GlyphBuffer {
    /// Creates a new empty [`GlyphBuffer`].
    #[inline]
    pub fn new() -> Self {
        let mut this = GlyphBuffer {
            core: ffi::BLGlyphBufferCore {
                impl_: ptr::null_mut(),
            },
        };
        unsafe { ffi::blGlyphBufferInit(&mut this.core) };
        this
    }

    /// Creates a new [`GlyphBuffer`] initialized with the given text.
    #[inline]
    pub fn from_utf8_text(text: &str) -> Self {
        let mut this = Self::new();
        this.set_utf8_text(text);
        this
    }

    #[inline]
    pub fn glyph_run(&self) -> GlyphRun<'_> {
        GlyphRun {
            raw: unsafe { &(*self.core.impl_).__bindgen_anon_1.glyphRun },
        }
    }

    #[inline]
    pub fn glyph_item_data(&self) -> GlyphRun<'_> {
        GlyphRun {
            raw: unsafe { &(*self.core.impl_).__bindgen_anon_1.glyphRun },
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.glyph_run().raw.size
    }

    /// Returns the [`GlyphBuffer`]'s [`GlyphRunFlags`].
    #[inline]
    pub fn flags(&self) -> GlyphRunFlags {
        GlyphRunFlags::from_bits_truncate(self.glyph_run().raw.flags)
    }

    /// Returns true if this [`GlyphBuffer`] contains unicode data.
    #[inline]
    pub fn has_text(&self) -> bool {
        self.flags().contains(GlyphRunFlags::UCS4_CONTENT)
    }

    /// Returns true if this [`GlyphBuffer`] contains [`GlyphId`] data.
    #[inline]
    pub fn has_glyphs(&self) -> bool {
        !self.has_text()
    }

    /// Returns true if this [`GlyphBuffer`] contains invalid characters(unicode
    /// encoding errors).
    #[inline]
    pub fn has_invalid_chars(&self) -> bool {
        self.flags().contains(GlyphRunFlags::INVALID_TEXT)
    }

    /// Returns true if this [`GlyphBuffer`] contains undefined characters that
    /// weren't mapped properly to glyphs.
    #[inline]
    pub fn has_undefined_chars(&self) -> bool {
        self.flags().contains(GlyphRunFlags::UNDEFINED_GLYPHS)
    }

    /// Returns true if one or more operations were terminated before completion
    /// because of invalid data in a font.
    #[inline]
    pub fn has_invalid_font_data(&self) -> bool {
        self.flags().contains(GlyphRunFlags::INVALID_FONT_DATA)
    }

    /// Clears the content of this [`GlyphBuffer`] without releasing internal
    /// buffers.
    #[inline]
    pub fn clear(&mut self) {
        unsafe { ffi::blGlyphBufferClear(&mut self.core) };
    }

    /// Sets text content of this [`GlyphBuffer`].
    #[inline]
    pub fn set_utf8_text(&mut self, text: &str) {
        unsafe {
            expect_mem_err(ffi::blGlyphBufferSetText(
                &mut self.core,
                text.as_bytes().as_ptr() as *const _,
                text.len(),
                ffi::BLTextEncoding::BL_TEXT_ENCODING_UTF8 as u32,
            ))
        };
    }
}

impl From<&'_ str> for GlyphBuffer {
    fn from(text: &str) -> Self {
        Self::from_utf8_text(text)
    }
}

impl Drop for GlyphBuffer {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::blGlyphBufferReset(&mut self.core) };
    }
}

impl fmt::Debug for GlyphBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GlyphBuffer").finish()
    }
}
