use std::{ffi::CString, path::Path};
use std::{fmt, slice, str};

use crate::error::{errcode_to_result, Result};
use crate::font_defs::*;
use crate::util::cast_ref;
use crate::variant::WrappedBlCore;
use crate::DataAccessFlags;

use super::Font;
use super::FontData;

/// Font Face
#[repr(transparent)]
pub struct FontFace {
    pub(in crate) core: ffi::BLFontFaceCore,
}

unsafe impl WrappedBlCore for FontFace {
    type Core = ffi::BLFontFaceCore;
    const IMPL_TYPE_INDEX: usize = crate::variant::ImplType::FontFace as usize;

    #[inline]
    fn from_core(core: Self::Core) -> Self {
        FontFace { core }
    }
}

impl FontFace {
    /// Creates a new FontFace from a given path.
    pub fn from_path<P: AsRef<Path>>(path: P, read_flags: DataAccessFlags) -> Result<Self> {
        let mut this = Self::from_core(*Self::none());
        let path = CString::new(path.as_ref().to_string_lossy().into_owned().into_bytes()).unwrap();
        unsafe {
            errcode_to_result(ffi::blFontFaceCreateFromFile(
                this.core_mut(),
                path.as_ptr(),
                read_flags.bits(),
            ))
            .map(|_| this)
        }
    }

    /// Creates a new FontFace from the given [`FontData`].
    pub fn from_data(data: &FontData, face_index: u32) -> Result<Self> {
        let mut this = Self::from_core(*Self::none());
        unsafe {
            errcode_to_result(ffi::blFontFaceCreateFromData(
                this.core_mut(),
                data.core(),
                face_index,
            ))
            .map(|_| this)
        }
    }

    /// Creates a new [`Font`] from this FontFace.
    pub fn create_font(&self, size: f32) -> Result<Font> {
        Font::from_face(self, size)
    }

    /// Returns the [`FontFaceInfo`].
    #[inline]
    pub fn face_info(&self) -> &FontFaceInfo {
        unsafe { &*(&self.impl_().faceInfo as *const _ as *const _) }
    }

    /// Returns the [`FontFaceType`].
    #[inline]
    pub fn face_type(&self) -> FontFaceType {
        self.face_info().face_type
    }

    /// Returns the [`FontFaceFlags`].
    #[inline]
    pub fn face_flags(&self) -> FontFaceFlags {
        self.face_info().face_flags
    }

    /// Tests whether the font-face uses typographic family and subfamily names.
    pub fn has_typographic_names(&self) -> bool {
        self.face_flags()
            .intersects(FontFaceFlags::TYPOGRAPHIC_NAMES)
    }

    /// Tests whether the font-face uses typographic metrics.
    pub fn has_typographic_metrics(&self) -> bool {
        self.face_flags()
            .intersects(FontFaceFlags::TYPOGRAPHIC_METRICS)
    }

    /// Tests whether the font-face provides character to glyph mapping.
    pub fn has_char_to_glyph_mapping(&self) -> bool {
        self.face_flags()
            .intersects(FontFaceFlags::CHAR_TO_GLYPH_MAPPING)
    }

    /// Tests whether the font-face has horizontal glyph metrics (advances, side
    /// bearings).
    pub fn has_horizontal_metrics(&self) -> bool {
        self.face_flags()
            .intersects(FontFaceFlags::HORIZONTAL_METIRCS)
    }

    /// Tests whether the font-face has vertical glyph metrics (advances, side
    /// bearings).
    pub fn has_vertical_metrics(&self) -> bool {
        self.face_flags()
            .intersects(FontFaceFlags::VERTICAL_METRICS)
    }

    /// Tests whether the font-face has a legacy horizontal kerning feature
    /// ('kern' table with horizontal kerning data).
    pub fn has_horizontal_kerning(&self) -> bool {
        self.face_flags()
            .intersects(FontFaceFlags::HORIZONTAL_KERNING)
    }

    /// Tests whether the font-face has a legacy vertical kerning feature
    /// ('kern' table with vertical kerning data).
    pub fn has_vertical_kerning(&self) -> bool {
        self.face_flags()
            .intersects(FontFaceFlags::VERTICAL_KERNING)
    }

    /// Tests whether the font-face has OpenType features (GDEF, GPOS, GSUB).
    pub fn has_open_type_features(&self) -> bool {
        self.face_flags()
            .intersects(FontFaceFlags::OPENTYPE_FEATURES)
    }

    /// Tests whether the font-face has panose classification.
    pub fn has_panose_data(&self) -> bool {
        self.face_flags().intersects(FontFaceFlags::PANOSE_DATA)
    }

    /// Tests whether the font-face has unicode coverage information.
    pub fn has_unicode_coverage(&self) -> bool {
        self.face_flags()
            .intersects(FontFaceFlags::UNICODE_COVERAGE)
    }

    /// Tests whether the font-face's baseline equals 0.
    pub fn has_baseline_y_at_0(&self) -> bool {
        self.face_flags()
            .intersects(FontFaceFlags::BASELINE_Y_EQUALS_0)
    }

    /// Tests whether the font-face's left sidebearing point at `x` equals 0.
    pub fn has_lsb_point_x_at_0(&self) -> bool {
        self.face_flags()
            .intersects(FontFaceFlags::LSB_POINT_X_EQUALS_0)
    }

    /// Tests whether the font-face has unicode variation sequences feature.
    pub fn has_variation_sequences(&self) -> bool {
        self.face_flags()
            .intersects(FontFaceFlags::VARIATION_SEQUENCES)
    }

    /// Tests whether the font-face has OpenType Font Variations feature.
    pub fn has_open_type_variations(&self) -> bool {
        self.face_flags()
            .intersects(FontFaceFlags::OPENTYPE_VARIATIONS)
    }

    /// This is a symbol font.
    pub fn is_symbol_font(&self) -> bool {
        self.face_flags().intersects(FontFaceFlags::SYMBOL_FONT)
    }

    /// This is a last resort font.
    pub fn is_last_resort_font(&self) -> bool {
        self.face_flags()
            .intersects(FontFaceFlags::LAST_RESORT_FONT)
    }

    pub fn font_data(&self) -> &FontData {
        unsafe { cast_ref(&self.impl_().data) }
    }

    /// Returns a zero-based index of this font-face.
    ///
    /// NOTE: Face index does only make sense if this face is part of a TrueType
    /// or OpenType font collection. In that case the returned value would be
    /// the index of this face in that collection. If the face is not part of a
    /// collection then the returned value would always be zero.
    #[inline]
    pub fn face_index(&self) -> u32 {
        self.face_info().face_index
    }

    /// Returns the [`FontOutlineType`].
    #[inline]
    pub fn outline_type(&self) -> FontOutlineType {
        self.face_info().outline_type
    }

    /// Returns the number of glyphs the face provides.
    #[inline]
    pub fn glyph_count(&self) -> u32 {
        self.face_info().glyph_count
    }

    /// Returns the [`FontFaceDiagFlags`].
    #[inline]
    pub fn diag_flags(&self) -> FontFaceDiagFlags {
        self.face_info().diag_flags
    }

    /// Returns a unique identifier describing this FontFace.
    #[inline]
    pub fn face_unique_id(&self) -> u64 {
        self.impl_().faceUniqueId
    }

    /// Returns the [`FontWeight`].
    #[inline]
    pub fn weight(&self) -> FontWeight {
        u32::from(self.impl_().weight).into()
    }

    /// Returns the [`FontStretch`].
    #[inline]
    pub fn stretch(&self) -> FontStretch {
        u32::from(self.impl_().stretch).into()
    }

    /// Returns the [`FontStyle`].
    #[inline]
    pub fn style(&self) -> FontStyle {
        u32::from(self.impl_().style).into()
    }

    /// Returns the [`FontData`] associated with this font-face.
    #[inline]
    pub fn data(&self) -> &FontData {
        unsafe { cast_ref(&self.impl_().data) }
    }

    /// Returns the design metrics of this [`FontFace`].
    #[inline]
    pub fn design_metrics(&self) -> &FontDesignMetrics {
        unsafe { cast_ref(&self.impl_().designMetrics) }
    }

    /// Returns the units per em, which are part of font's design metrics.
    #[inline]
    pub fn units_per_em(&self) -> i32 {
        self.design_metrics().units_per_em
    }

    // TODO panose

    #[inline]
    pub fn unicode_coverage(&self) -> &FontUnicodeCoverage {
        unsafe { &*(&self.impl_().unicodeCoverage as *const _ as *const _) }
    }

    /// Returns the full name.
    #[inline]
    pub fn full_name(&self) -> &str {
        bl_string_to_str(&self.impl_().fullName)
    }

    /// Returns the family name.
    #[inline]
    pub fn family_name(&self) -> &str {
        bl_string_to_str(&self.impl_().familyName)
    }

    /// Returns the subfamily name.
    #[inline]
    pub fn subfamily_name(&self) -> &str {
        bl_string_to_str(&self.impl_().subfamilyName)
    }

    /// Returns the post script name.
    #[inline]
    pub fn post_script_name(&self) -> &str {
        bl_string_to_str(&self.impl_().postScriptName)
    }
}

#[inline]
fn bl_string_to_str(bl_string: &ffi::BLStringCore) -> &str {
    unsafe {
        let ffi_slice = (*bl_string.impl_).__bindgen_anon_1.__bindgen_anon_1;
        str::from_utf8_unchecked(slice::from_raw_parts(ffi_slice.data as _, ffi_slice.size))
    }
}

impl PartialEq for FontFace {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::blFontFaceEquals(self.core(), other.core()) }
    }
}

impl Clone for FontFace {
    fn clone(&self) -> Self {
        Self::from_core(self.init_weak())
    }
}

impl Drop for FontFace {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::blFontFaceReset(&mut self.core) };
    }
}

impl fmt::Debug for FontFace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FontFace").finish()
    }
}
