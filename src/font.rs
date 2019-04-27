use core::{fmt, slice, str};
use std::ffi::CString;

use crate::{
    array::Array,
    error::{errcode_to_result, Result},
    font_defs::*,
    variant::{BlVariantImpl, WrappedBlCore},
    Tag,
};

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
    pub fn from_path(file_name: &str) -> Result<Self> {
        let mut this = Self::from_core(*Self::none());
        let file_name = CString::new(file_name).unwrap();
        unsafe {
            errcode_to_result(ffi::blFontLoaderCreateFromFile(
                this.core_mut(),
                file_name.as_ptr(),
            ))
            .map(|_| this)
        }
    }

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

    /// FIXME lifetimes
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

    #[inline]
    pub fn face_type(&self) -> FontFaceType {
        (self.impl_().faceType as u32).into()
    }

    #[inline]
    pub fn face_count(&self) -> u32 {
        self.impl_().faceCount
    }

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

#[repr(transparent)]
pub struct FontFace {
    pub(in crate) core: ffi::BLFontFaceCore,
}

unsafe impl WrappedBlCore for FontFace {
    type Core = ffi::BLFontFaceCore;
    const IMPL_TYPE_INDEX: usize = crate::variant::ImplType::FontFace as usize;

    fn from_core(core: Self::Core) -> Self {
        FontFace { core }
    }
}

impl FontFace {
    pub fn from_path(file_name: &str) -> Result<Self> {
        let mut this = Self::from_core(*Self::none());
        let file_name = CString::new(file_name).unwrap();
        unsafe {
            errcode_to_result(ffi::blFontFaceCreateFromFile(
                this.core_mut(),
                file_name.as_ptr(),
            ))
            .map(|_| this)
        }
    }

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

    pub fn create_font(&self, size: f32) -> Result<Font> {
        Font::from_face(self, size)
    }

    #[inline]
    pub fn face_type(&self) -> FontFaceType {
        self.face_info().face_type
    }

    #[inline]
    pub fn face_flags(&self) -> FontFaceFlags {
        self.face_info().face_flags
    }

    #[inline]
    pub fn face_index(&self) -> u32 {
        self.face_info().face_index
    }
    #[inline]
    pub fn outline_type(&self) -> FontOutlineType {
        self.face_info().outline_type
    }

    #[inline]
    pub fn glyph_count(&self) -> u32 {
        self.face_info().glyph_count
    }

    #[inline]
    pub fn diag_flags(&self) -> FontFaceDiagFlags {
        self.face_info().diag_flags
    }

    #[inline]
    pub fn face_unique_id(&self) -> u64 {
        self.impl_().faceUniqueId
    }

    #[inline]
    pub fn weight(&self) -> FontWeight {
        (self.impl_().weight as u32).into()
    }

    #[inline]
    pub fn stretch(&self) -> FontStretch {
        (self.impl_().stretch as u32).into()
    }

    #[inline]
    pub fn style(&self) -> FontStyle {
        (self.impl_().style as u32).into()
    }

    #[inline]
    pub fn data(&self) -> &FontData {
        unsafe { &*(&self.impl_().data as *const _ as *const _) }
    }

    #[inline]
    pub fn loader(&self) -> &FontLoader {
        unsafe { &*(&self.impl_().loader as *const _ as *const _) }
    }

    #[inline]
    pub fn face_info(&self) -> &FontFaceInfo {
        unsafe { &*(&self.impl_().faceInfo as *const _ as *const _) }
    }

    #[inline]
    pub fn design_metrics(&self) -> &FontDesignMetrics {
        unsafe { &*(&self.impl_().designMetrics as *const _ as *const _) }
    }

    #[inline]
    pub fn units_per_em(&self) -> i32 {
        self.design_metrics().units_per_em
    }

    #[inline]
    pub fn unicode_coverage(&self) -> &FontUnicodeCoverage {
        unsafe { &*(&self.impl_().unicodeCoverage as *const _ as *const _) }
    }

    #[inline]
    pub fn full_name(&self) -> &str {
        bl_string_to_str(&self.impl_().fullName)
    }

    #[inline]
    pub fn family_name(&self) -> &str {
        bl_string_to_str(&self.impl_().familyName)
    }

    #[inline]
    pub fn subfamily_name(&self) -> &str {
        bl_string_to_str(&self.impl_().subfamilyName)
    }

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

#[repr(transparent)]
pub struct Font {
    core: ffi::BLFontCore,
}

unsafe impl WrappedBlCore for Font {
    type Core = ffi::BLFontCore;
    const IMPL_TYPE_INDEX: usize = crate::variant::ImplType::Font as usize;

    fn from_core(core: Self::Core) -> Self {
        Font { core }
    }
}

impl Font {
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

    pub fn face(&self) -> &FontFace {
        unsafe { &*(&self.impl_().face as *const _ as *const _) }
    }

    pub fn face_type(&self) -> FontFaceType {
        self.face().face_type()
    }

    pub fn face_flags(&self) -> FontFaceFlags {
        self.face().face_flags()
    }

    pub fn units_per_em(&self) -> i32 {
        self.face().units_per_em()
    }

    pub fn size(&self) -> f32 {
        self.impl_().metrics.size
    }

    pub fn features(&self) -> &Array<FontFeature> {
        unsafe { &*(&self.impl_().features as *const _ as *const _) }
    }

    pub fn variations(&self) -> &Array<FontVariation> {
        unsafe { &*(&self.impl_().variations as *const _ as *const _) }
    }

    #[inline]
    pub fn weight(&self) -> FontWeight {
        (self.impl_().weight as u32).into()
    }

    #[inline]
    pub fn stretch(&self) -> FontStretch {
        (self.impl_().stretch as u32).into()
    }

    #[inline]
    pub fn style(&self) -> FontStyle {
        (self.impl_().style as u32).into()
    }

    #[inline]
    pub fn font_matrix(&self) -> &FontMatrix {
        unsafe { &*(&self.impl_().matrix as *const _ as *const _) }
    }

    #[inline]
    pub fn font_metrics(&self) -> &FontMetrics {
        unsafe { &*(&self.impl_().metrics as *const _ as *const _) }
    }

    #[inline]
    pub fn design_metrics(&self) -> &FontDesignMetrics {
        self.face().design_metrics()
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
