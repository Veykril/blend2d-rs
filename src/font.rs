mod face;
pub use self::face::FontFace;

mod manager;
pub use self::manager::FontManager;

mod data;
pub use self::data::FontData;

use std::fmt;

use crate::array::Array;
use crate::error::{errcode_to_result, Result};
use crate::font_defs::*;
use crate::glyph_buffer::GlyphBuffer;
use crate::variant::WrappedBlCore;

/// Font
#[repr(transparent)]
pub struct Font {
    core: ffi::BLFontCore,
}

unsafe impl WrappedBlCore for Font {
    type Core = ffi::BLFontCore;
    const IMPL_TYPE_INDEX: usize = crate::variant::ImplType::Font as usize;

    #[inline]
    fn from_core(core: Self::Core) -> Self {
        Font { core }
    }
}

impl Font {
    /// Creates a new font from the given [`FontFace`].
    pub fn from_face(face: &FontFace, size: f32) -> Result<Self> {
        let mut this = Self::from_core(*Self::none());
        unsafe {
            errcode_to_result(ffi::blFontCreateFromFace(
                this.core_mut(),
                face.core(),
                size,
            ))
            .map(|_| this)
        }
    }

    /// Returns a font-face of the font.
    ///
    /// Returns the same font-face, which was passed to
    /// [`from_face`](struct.Font.html#method.from_face).
    pub fn face(&self) -> &FontFace {
        unsafe { &*(&self.impl_().face as *const _ as *const _) }
    }

    /// Returns the [`FontFaceType`] of the font.
    pub fn face_type(&self) -> FontFaceType {
        self.face().face_type()
    }

    /// Returns the [`FontFaceFlags`]  of the font.
    pub fn face_flags(&self) -> FontFaceFlags {
        self.face().face_flags()
    }

    /// Returns the "units per em" (UPEM) of the font's associated font-face.
    pub fn units_per_em(&self) -> i32 {
        self.face().units_per_em()
    }

    /// Returns the size of the font (as float).
    pub fn size(&self) -> f32 {
        self.impl_().metrics.size
    }

    /// Returns the font-features used by this font.
    pub fn features(&self) -> &Array<FontFeature> {
        unsafe { &*(&self.impl_().features as *const _ as *const _) }
    }

    /// Returns the font-variations used by this font.
    pub fn variations(&self) -> &Array<FontVariation> {
        unsafe { &*(&self.impl_().variations as *const _ as *const _) }
    }

    /// Returns the weight of the font.
    #[inline]
    pub fn weight(&self) -> FontWeight {
        u32::from(self.impl_().weight).into()
    }

    /// Returns the stretch of the font.
    #[inline]
    pub fn stretch(&self) -> FontStretch {
        u32::from(self.impl_().stretch).into()
    }

    /// Returns the style of the font.
    #[inline]
    pub fn style(&self) -> FontStyle {
        u32::from(self.impl_().style).into()
    }

    /// Returns a 2x2 matrix of the font.
    ///
    /// The returned [`FontMatrix`] is used to scale fonts from design units
    /// into user units. The matrix usually has a negative value at the 3rd
    /// index of the internal array as fonts use a different coordinate
    /// system than Blend2D.
    #[inline]
    pub fn font_matrix(&self) -> &FontMatrix {
        unsafe { &*(&self.impl_().matrix as *const _ as *const _) }
    }

    /// Returns a scaled metrics of this font.
    ///
    /// The returned metrics is a scale of design metrics that match the font
    /// size and its options.
    #[inline]
    pub fn font_metrics(&self) -> &FontMetrics {
        unsafe { &*(&self.impl_().metrics as *const _ as *const _) }
    }

    /// Returns a design metrics of this font.
    ///
    /// The returned metrics is compatible with the metrics of [FontFace]
    /// associated with this font.
    pub fn design_metrics(&self) -> &FontDesignMetrics {
        self.face().design_metrics()
    }

    #[inline]
    pub fn shape(&self, buf: &mut GlyphBuffer) -> Result<()> {
        unsafe { errcode_to_result(ffi::blFontShape(self.core(), &mut buf.core)) }
    }

    #[inline]
    pub fn map_text_to_glyphs(&self, buf: &mut GlyphBuffer) -> Result<GlyphMappingState> {
        let mut state = GlyphMappingState {
            glyph_count: 0,
            undefined_first: 0,
            undefined_count: 0,
        };
        unsafe {
            errcode_to_result(ffi::blFontMapTextToGlyphs(
                self.core(),
                &mut buf.core,
                &mut state as *mut _ as *mut _,
            ))
            .map(|_| state)
        }
    }

    // TODO positionGlyphs

    #[inline]
    pub fn apply_kerning(&self, buf: &mut GlyphBuffer) -> Result<()> {
        unsafe { errcode_to_result(ffi::blFontApplyKerning(self.core(), &mut buf.core)) }
    }

    #[inline]
    pub fn apply_g_sub(&self, buf: &mut GlyphBuffer, index: usize, lookups: usize) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blFontApplyGSub(
                self.core(),
                &mut buf.core,
                index,
                lookups,
            ))
        }
    }

    #[inline]
    pub fn apply_g_pos(&self, buf: &mut GlyphBuffer, index: usize, lookups: usize) -> Result<()> {
        unsafe {
            errcode_to_result(ffi::blFontApplyGPos(
                self.core(),
                &mut buf.core,
                index,
                lookups,
            ))
        }
    }

    #[inline]
    pub fn get_text_metrics(&self, buf: &mut GlyphBuffer) -> Result<TextMetrics> {
        let mut metrics = TextMetrics::default();
        unsafe {
            errcode_to_result(ffi::blFontGetTextMetrics(
                self.core(),
                &mut buf.core,
                &mut metrics as *mut _ as *mut _,
            ))
            .map(|_| metrics)
        }
    }

    //TODO getGlyphBounds

    //TODO getGlyphAdvances

    //TODO getGlyphOutlines

    //TODO getGlyphRunOutlines
}

impl PartialEq for Font {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::blFontEquals(self.core(), other.core()) }
    }
}

impl Clone for Font {
    fn clone(&self) -> Self {
        Self::from_core(self.init_weak())
    }
}

impl Drop for Font {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::blFontReset(&mut self.core) };
    }
}

impl fmt::Debug for Font {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Font").finish()
    }
}
