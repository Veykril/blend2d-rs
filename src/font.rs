use std::{ffi::CString, path::Path};
use std::{fmt, slice, str};

use crate::array::Array;
use crate::error::{errcode_to_result, Result};
use crate::font_defs::*;
use crate::glyph_buffer::GlyphBuffer;
use crate::variant::{BlVariantImpl, WrappedBlCore};
use crate::DataAccessFlags;
use crate::Tag;

/// Font Data
#[repr(transparent)]
pub struct FontData {
    core: ffi::BLFontDataCore,
}

unsafe impl WrappedBlCore for FontData {
    type Core = ffi::BLFontDataCore;
    const IMPL_TYPE_INDEX: usize = crate::variant::ImplType::FontData as usize;

    fn from_core(core: Self::Core) -> Self {
        FontData { core }
    }
}

impl FontData {
    pub fn list_tags(&self) -> Result<Array<Tag>> {
        unsafe {
            let mut arr = Array::<Tag>::new();
            errcode_to_result((self.impl_().virt().listTags.unwrap())(
                self.impl_(),
                arr.core_mut(),
            ))
            .map(|_| arr)
        }
    }

    // FIXME figure out how query tables works
    /*pub fn query_tables(&self, tags: &[Tag]) -> (FontTable, usize) {
        unsafe {
            let mut dst = FontTable {
                data: ptr::null(),
                size: 0,
            };
            let n = (self.impl_().virt().queryTables.unwrap())(
                self.impl_(),
                &mut dst as *mut _ as *mut _,
                tags.as_ptr() as *const _ as *const _,
                tags.len(),
            );
            (dst, n)
        }
    }*/
}

impl PartialEq for FontData {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::blFontDataEquals(self.core(), other.core()) }
    }
}

impl Drop for FontData {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::blFontDataReset(&mut self.core) };
    }
}

impl fmt::Debug for FontData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FontData").finish()
    }
}

/// Font Loader
#[repr(transparent)]
pub struct FontLoader {
    core: ffi::BLFontLoaderCore,
}

unsafe impl WrappedBlCore for FontLoader {
    type Core = ffi::BLFontLoaderCore;
    const IMPL_TYPE_INDEX: usize = crate::variant::ImplType::FontLoader as usize;

    fn from_core(core: Self::Core) -> Self {
        FontLoader { core }
    }
}

impl FontLoader {
    /// Creates a new font by reading a file at the given path.
    pub fn from_path<P: AsRef<Path>>(path: P, read_flags: DataAccessFlags) -> Result<Self> {
        let mut this = Self::from_core(*Self::none());
        let path = CString::new(path.as_ref().to_string_lossy().into_owned().into_bytes()).unwrap();
        unsafe {
            errcode_to_result(ffi::blFontLoaderCreateFromFile(
                this.core_mut(),
                path.as_ptr(),
                read_flags.bits(),
            ))
            .map(|_| this)
        }
    }

    /// Creates a new font from the given [`Array`].
    pub fn from_data_array(data: &Array<u8>) -> Result<Self> {
        let mut this = Self::from_core(*Self::none());
        unsafe {
            errcode_to_result(ffi::blFontLoaderCreateFromDataArray(
                this.core_mut(),
                data.core(),
            ))
            .map(|_| this)
        }
    }

    // FIXME lifetimes
    /*pub fn from_data<R: AsRef<[u8]>>(data: R) -> Result<Self> {
        let mut this = Self::from_core(*Self::none());
        unsafe {
            errcode_to_result(ffi::blFontLoaderCreateFromData(
                this.core_mut(),
                data.as_ref().as_ptr() as *const _,
                data.as_ref().len(),
                None,
                ptr::null_mut(),
            ))
            .map(|_| this)
        }
    }*/

    #[inline]
    pub fn create_font_face(&self, index: u32) -> Result<FontFace> {
        FontFace::from_loader(self, index)
    }

    #[inline]
    pub fn data_by_face_index(&mut self, idx: u32) -> FontData {
        FontData::from_core(ffi::BLFontDataCore {
            impl_: unsafe { (self.impl_().virt().dataByFaceIndex.unwrap())(self.impl_mut(), idx) },
        })
    }

    /// Type of font-face of the loader content.
    ///
    /// It doesn't matter if the content is a single font or a collection. In
    /// any case `face_type` would always return the type of the font-face
    /// that will be created by
    /// [`FontFace::from_loader`](struct.FontFace.html#method.from_loader).
    #[inline]
    pub fn face_type(&self) -> FontFaceType {
        u32::from(self.impl_().faceType).into()
    }

    /// Returns the number of faces this loader provides.
    ///
    /// If the loader is initialized to a single font it would be 1, and if the
    /// loader is initialized to a font collection then the return would
    /// correspond to the number of font-faces within that collection.
    #[inline]
    pub fn face_count(&self) -> u32 {
        self.impl_().faceCount
    }

    /// Returns the [`FontLoaderFlags`].
    #[inline]
    pub fn loader_flags(&self) -> FontLoaderFlags {
        FontLoaderFlags::from_bits_truncate(self.impl_().loaderFlags)
    }
}

impl PartialEq for FontLoader {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::blFontLoaderEquals(self.core(), other.core()) }
    }
}

impl Clone for FontLoader {
    fn clone(&self) -> Self {
        Self::from_core(self.init_weak())
    }
}

impl Drop for FontLoader {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::blFontLoaderReset(&mut self.core) };
    }
}

impl fmt::Debug for FontLoader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FontLoader").finish()
    }
}

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

    /// Creates a new FontFace from the given [`FontLoader`].
    pub fn from_loader(loader: &FontLoader, face_index: u32) -> Result<Self> {
        let mut this = Self::from_core(*Self::none());
        unsafe {
            errcode_to_result(ffi::blFontFaceCreateFromLoader(
                this.core_mut(),
                loader.core(),
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
        unsafe { &*(&self.impl_().data as *const _ as *const _) }
    }

    /// Returns the [`FontLoader`] associated with this font-face.
    #[inline]
    pub fn loader(&self) -> &FontLoader {
        unsafe { &*(&self.impl_().loader as *const _ as *const _) }
    }

    /// Returns the design metrics of this [`FontFace`].
    #[inline]
    pub fn design_metrics(&self) -> &FontDesignMetrics {
        unsafe { &*(&self.impl_().designMetrics as *const _ as *const _) }
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
    #[inline]
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
