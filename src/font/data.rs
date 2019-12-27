use std::{ffi::CString, path::Path};
use std::{fmt, slice};

use crate::array::Array;
use crate::error::{errcode_to_result, Result};
use crate::font_defs::*;
use crate::variant::WrappedBlCore;
use crate::DataAccessFlags;
use crate::Tag;

use super::FontFace;

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
    /// Creates a new font by reading a file at the given path.
    pub fn from_path<P: AsRef<Path>>(path: P, read_flags: DataAccessFlags) -> Result<Self> {
        let mut this = Self::from_core(*Self::none());
        let path = CString::new(path.as_ref().to_string_lossy().into_owned().into_bytes()).unwrap();
        unsafe {
            errcode_to_result(ffi::blFontDataCreateFromFile(
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
            errcode_to_result(ffi::blFontDataCreateFromDataArray(
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
            errcode_to_result(ffi::blFontDataCreateFromData(
                this.core_mut(),
                data.as_ref().as_ptr() as *const _,
                data.as_ref().len(),
                None,
                ptr::null_mut(),
            ))
            .map(|_| this)
        }
    }*/

    pub fn create_font_face(&self, face_index: u32) -> Result<FontFace> {
        FontFace::from_data(self, face_index)
    }

    pub fn list_tags(&self, face_index: u32) -> Result<Array<Tag>> {
        unsafe {
            let mut arr = Array::<Tag>::new();
            errcode_to_result(ffi::blFontDataListTags(
                self.core(),
                face_index,
                arr.core_mut(),
            ))
            .map(|_| arr)
        }
    }

    pub fn query_table(&self, face_index: u32, tag: Tag) -> (FontTable<'_>, usize) {
        self.query_tables(face_index, &[tag])
    }

    pub fn query_tables(&self, face_index: u32, tags: &[Tag]) -> (FontTable<'_>, usize) {
        unsafe {
            let mut dst = ffi::BLFontTable {
                data: std::ptr::null(),
                size: 0,
            };
            let n = ffi::blFontDataQueryTables(
                self.core(),
                face_index,
                &mut dst,
                tags.as_ptr() as *const _ as *const _,
                tags.len(),
            );
            (
                FontTable {
                    data: slice::from_raw_parts(dst.data, dst.size),
                },
                n,
            )
        }
    }

    /// Type of font-face.
    ///
    /// It doesn't matter if the content is a single font or a collection. In
    /// any case `face_type` would always return the type of the font-face
    /// that will be created by
    #[inline]
    pub fn face_type(&self) -> FontFaceType {
        u32::from(self.impl_().faceType).into()
    }

    /// Returns the number of faces this data provides.
    ///
    /// If the data is initialized to a single font it would be 1, and if the
    /// data is initialized to a font collection then the return would
    /// correspond to the number of font-faces within that collection.
    ///
    /// You should not use [`face_count`] to check whether the font is a
    /// collection as it's possible to have a font-collection with just a single
    /// font. Using [is_collection`] is more reliable and would always return
    /// the right value.
    #[inline]
    pub fn face_count(&self) -> u32 {
        self.impl_().faceCount
    }

    /// Tests whether this font-data is a font-collection.
    #[inline]
    pub fn is_collection(&self) -> bool {
        self.flags().intersects(FontDataFlags::COLLECTION)
    }

    /// Returns the [`FontDataFlags`].
    #[inline]
    pub fn flags(&self) -> FontDataFlags {
        FontDataFlags::from_bits_truncate(self.impl_().flags)
    }
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
