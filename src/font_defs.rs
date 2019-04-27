use bitflags::bitflags;

use crate::geometry::{BoxD, PointD, PointI};

use ffi::BLGlyphItemFlags::*;
bitflags! {
    pub struct GlyphItemFlags: u32 {
        const MARK = BL_GLYPH_ITEM_FLAG_MARK as u32;
    }
}

use ffi::BLGlyphPlacementType::*;
bl_enum! {
    pub enum GlyphPlacementType {
        None          = BL_GLYPH_PLACEMENT_TYPE_NONE,
        AdvanceOffset = BL_GLYPH_PLACEMENT_TYPE_ADVANCE_OFFSET,
        DesignUnits   = BL_GLYPH_PLACEMENT_TYPE_DESIGN_UNITS,
        UserUnits     = BL_GLYPH_PLACEMENT_TYPE_USER_UNITS,
        AbsoluteUnits = BL_GLYPH_PLACEMENT_TYPE_ABSOLUTE_UNITS,
    }
    Default => None
}

use ffi::BLGlyphRunFlags::*;
bitflags! {
    pub struct GlyphRunFlags: u32 {
        const UCS4_CONTENT      = BL_GLYPH_RUN_FLAG_UCS4_CONTENT      as u32;
        const INVALID_TEXT      = BL_GLYPH_RUN_FLAG_INVALID_TEXT      as u32;
        const UNDEFINED_GLYPHS  = BL_GLYPH_RUN_FLAG_UNDEFINED_GLYPHS  as u32;
        const INVALID_FONT_DATA = BL_GLYPH_RUN_FLAG_INVALID_FONT_DATA as u32;
    }
}

use ffi::BLFontFaceType::*;
bl_enum! {
    #[repr(u8)]
    pub enum FontFaceType {
        None     = BL_FONT_FACE_TYPE_NONE,
        OpenType = BL_FONT_FACE_TYPE_OPENTYPE,
    }
    Default => None
}

use ffi::BLFontFaceFlags::*;
bitflags! {
    pub struct FontFaceFlags: u32 {
        const TYPOGRAPHIC_NAMES     = BL_FONT_FACE_FLAG_TYPOGRAPHIC_NAMES     as u32;
        const TYPOGRAPHIC_METRICS   = BL_FONT_FACE_FLAG_TYPOGRAPHIC_METRICS   as u32;
        const CHAR_TO_GLYPH_MAPPING = BL_FONT_FACE_FLAG_CHAR_TO_GLYPH_MAPPING as u32;
        const HORIZONTAL_METIRCS    = BL_FONT_FACE_FLAG_HORIZONTAL_METIRCS    as u32;
        const VERTICAL_METRICS      = BL_FONT_FACE_FLAG_VERTICAL_METRICS      as u32;
        const HORIZONTAL_KERNING    = BL_FONT_FACE_FLAG_HORIZONTAL_KERNING    as u32;
        const VERTICAL_KERNING      = BL_FONT_FACE_FLAG_VERTICAL_KERNING      as u32;
        const OPENTYPE_FEATURES     = BL_FONT_FACE_FLAG_OPENTYPE_FEATURES     as u32;
        const OPENTYPE_VARIATIONS   = BL_FONT_FACE_FLAG_OPENTYPE_VARIATIONS   as u32;
        const PANOSE_DATA           = BL_FONT_FACE_FLAG_PANOSE_DATA           as u32;
        const UNICODE_COVERAGE      = BL_FONT_FACE_FLAG_UNICODE_COVERAGE      as u32;
        const VARIATION_SEQUENCES   = BL_FONT_FACE_FLAG_VARIATION_SEQUENCES   as u32;
        const SYMBOL_FONT           = BL_FONT_FACE_FLAG_SYMBOL_FONT           as u32;
        const LAST_RESORT_FONT      = BL_FONT_FACE_FLAG_LAST_RESORT_FONT      as u32;
    }
}

use ffi::BLFontFaceDiagFlags::*;
bitflags! {
    pub struct FontFaceDiagFlags: u32 {
        const WRONG_NAME_DATA   = BL_FONT_FACE_DIAG_WRONG_NAME_DATA   as u32;
        const FIXED_NAME_DATA   = BL_FONT_FACE_DIAG_FIXED_NAME_DATA   as u32;
        const WRONG_KERN_DATA   = BL_FONT_FACE_DIAG_WRONG_KERN_DATA   as u32;
        const FIXED_KERN_DATA   = BL_FONT_FACE_DIAG_FIXED_KERN_DATA   as u32;
        const WRONG_CMAP_DATA   = BL_FONT_FACE_DIAG_WRONG_CMAP_DATA   as u32;
        const WRONG_CMAP_FORMAT = BL_FONT_FACE_DIAG_WRONG_CMAP_FORMAT as u32;
        const WRONG_GDEF_DATA   = BL_FONT_FACE_DIAG_WRONG_GDEF_DATA   as u32;
        const WRONG_GPOS_DATA   = BL_FONT_FACE_DIAG_WRONG_GPOS_DATA   as u32;
        const WRONG_GSUB_DATA   = BL_FONT_FACE_DIAG_WRONG_GSUB_DATA   as u32;
    }
}

use ffi::BLFontLoaderFlags::*;
bitflags! {
    pub struct FontLoaderFlags: u32 {
        const COLLECTION = BL_FONT_LOADER_FLAG_COLLECTION as u32;
    }
}

use ffi::BLFontOutlineType::*;
bl_enum! {
    #[repr(u8)]
    pub enum FontOutlineType {
        None     = BL_FONT_OUTLINE_TYPE_NONE,
        TrueType = BL_FONT_OUTLINE_TYPE_TRUETYPE,
        Cff      = BL_FONT_OUTLINE_TYPE_CFF,
        Cff2     = BL_FONT_OUTLINE_TYPE_CFF2,
    }
    Default => None
}

use ffi::BLFontStretch::*;
bl_enum! {
    pub enum FontStretch {
        UltraCondensed = BL_FONT_STRETCH_ULTRA_CONDENSED,
        ExtraCondensed = BL_FONT_STRETCH_EXTRA_CONDENSED,
        Condensed      = BL_FONT_STRETCH_CONDENSED,
        SemiCondensed  = BL_FONT_STRETCH_SEMI_CONDENSED,
        Normal         = BL_FONT_STRETCH_NORMAL,
        SemiExpanded   = BL_FONT_STRETCH_SEMI_EXPANDED,
        Expanded       = BL_FONT_STRETCH_EXPANDED,
        ExtraExpanded  = BL_FONT_STRETCH_EXTRA_EXPANDED,
        UltraExpanded  = BL_FONT_STRETCH_ULTRA_EXPANDED,
    }
    Default => Normal
}

use ffi::BLFontStyle::*;
bl_enum! {
    pub enum FontStyle {
        Normal = BL_FONT_STYLE_NORMAL,
        Oblique = BL_FONT_STYLE_OBLIQUE,
        Italic = BL_FONT_STYLE_ITALIC,
    }
    Default => Normal
}

use ffi::BLFontWeight::*;
bl_enum! {
    pub enum FontWeight {
        Thin       = BL_FONT_WEIGHT_THIN,
        ExtraLight = BL_FONT_WEIGHT_EXTRA_LIGHT,
        Light      = BL_FONT_WEIGHT_LIGHT,
        SemiLight  = BL_FONT_WEIGHT_SEMI_LIGHT,
        Normal     = BL_FONT_WEIGHT_NORMAL,
        Medium     = BL_FONT_WEIGHT_MEDIUM,
        SemiBold   = BL_FONT_WEIGHT_SEMI_BOLD,
        Bold       = BL_FONT_WEIGHT_BOLD,
        ExtraBold  = BL_FONT_WEIGHT_EXTRA_BOLD,
        Black      = BL_FONT_WEIGHT_BLACK,
        ExtraBlack = BL_FONT_WEIGHT_EXTRA_BLACK,
    }
    Default => Normal
}

use ffi::BLFontStringId::*;
bl_enum! {
    pub enum FontStringId {
        CopyrightNotice            = BL_FONT_STRING_COPYRIGHT_NOTICE,
        FamilyName                 = BL_FONT_STRING_FAMILY_NAME,
        SubfamilyName              = BL_FONT_STRING_SUBFAMILY_NAME,
        UniqueIdentifier           = BL_FONT_STRING_UNIQUE_IDENTIFIER,
        FullName                   = BL_FONT_STRING_FULL_NAME,
        VersionString              = BL_FONT_STRING_VERSION_STRING,
        PostScriptName             = BL_FONT_STRING_POST_SCRIPT_NAME,
        Trademark                  = BL_FONT_STRING_TRADEMARK,
        ManufacturerName           = BL_FONT_STRING_MANUFACTURER_NAME,
        DesignerName               = BL_FONT_STRING_DESIGNER_NAME,
        Description                = BL_FONT_STRING_DESCRIPTION,
        VendorUrl                  = BL_FONT_STRING_VENDOR_URL,
        DesignerUrl                = BL_FONT_STRING_DESIGNER_URL,
        LicenseDescription         = BL_FONT_STRING_LICENSE_DESCRIPTION,
        LicenseInfoUrl             = BL_FONT_STRING_LICENSE_INFO_URL,
        Reserved                   = BL_FONT_STRING_RESERVED,
        TypographicsFamilyName     = BL_FONT_STRING_TYPOGRAPHIC_FAMILY_NAME,
        TypographicsSubfamilyName  = BL_FONT_STRING_TYPOGRAPHIC_SUBFAMILY_NAME,
        CompatibleFullname         = BL_FONT_STRING_COMPATIBLE_FULL_NAME,
        SampleText                 = BL_FONT_STRING_SAMPLE_TEXT,
        PostScriptCidName          = BL_FONT_STRING_POST_SCRIPT_CID_NAME,
        WwsFamilyName              = BL_FONT_STRING_WWS_FAMILY_NAME,
        WwsSubfamilyName           = BL_FONT_STRING_WWS_SUBFAMILY_NAME,
        LightBackgroundPalette     = BL_FONT_STRING_LIGHT_BACKGROUND_PALETTE,
        DarkBackgroundPalette      = BL_FONT_STRING_DARK_BACKGROUND_PALETTE,
        VariationsPostScriptPrefix = BL_FONT_STRING_VARIATIONS_POST_SCRIPT_PREFIX,
        CommonCount                = BL_FONT_STRING_COMMON_COUNT,
        CustomStartIndex           = BL_FONT_STRING_CUSTOM_START_INDEX,
    }
    Default => Reserved
}

use ffi::BLTextDirection::*;
bl_enum! {
    pub enum TextDirection {
        Ltr = BL_TEXT_DIRECTION_LTR,
        Rtl = BL_TEXT_DIRECTION_RTL,
    }
    Default => Ltr
}

use crate::Tag;
use ffi::BLTextOrientation::*;
bl_enum! {
    pub enum TextOrientation {
        Horizontal = BL_TEXT_ORIENTATION_HORIZONTAL,
        Vertical   = BL_TEXT_ORIENTATION_VERTICAL,
    }
    Default => Horizontal
}

#[cfg(target_endian = "little")]
#[repr(C)]
#[derive(Debug)]
pub(in crate) struct GlyphItem {
    pub glyph_id: u16,
    reserved: u16,
}

#[cfg(target_endian = "big")]
#[repr(C)]
#[derive(Debug)]
pub(in crate) struct GlyphItem {
    reserved: u16,
    pub glyph_id: u16,
}

#[repr(C)]
#[derive(Debug)]
pub(in crate) struct GlyphInfo {
    pub cluster: u32,
    reserved: [u32; 2],
}

#[repr(C)]
#[derive(Debug)]
pub struct GlyphPlacement {
    pub placement: PointI,
    pub advance: PointI,
}

#[repr(C)]
#[derive(Debug)]
pub struct GlyphMappingState {
    pub glyph_count: usize,
    pub undefined_first: usize,
    pub undefined_count: usize,
}

impl GlyphMappingState {
    pub fn undefined_first(&self) -> Option<usize> {
        if self.undefined_first == !0 {
            None
        } else {
            Some(self.undefined_first)
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct GlyphOutlineSinkInfo {
    pub glyph_index: usize,
    pub contour_count: usize,
}

#[repr(C)]
#[derive(Debug)]
struct GlyphRun {
    //todo GlyphRun
}

#[repr(C)]
#[derive(Debug)]
pub struct FontFaceInfo {
    pub face_type: FontFaceType,
    pub outline_type: FontOutlineType,
    pub glyph_count: u32,
    pub face_index: u32,
    pub face_flags: FontFaceFlags,
    pub diag_flags: FontFaceDiagFlags,
}

#[repr(C)]
#[derive(Debug)]
pub struct FontTable {
    pub data: *const u8,
    pub size: usize,
}
//pub type FontTable<'a> = &'a [u8];

#[repr(C)]
#[derive(Debug)]
pub struct FontFeature {
    pub tag: Tag,
    pub value: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct FontVariation {
    pub tag: Tag,
    pub value: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct FontUnicodeCoverage {
    pub data: [u32; 4],
}

#[repr(C)]
#[derive(Debug)]
pub struct FontMatrix(pub [f32; 4]);

#[repr(C)]
#[derive(Debug)]
pub struct FontMetrics {
    pub size: f32,
    pub horizontal_ascent: f32,
    pub vertical_ascent: f32,
    pub horizontal_descent: f32,
    pub vertical_descent: f32,
    pub line_gap: f32,
    pub x_height: f32,
    pub cap_height: f32,
    pub underline_position: f32,
    pub underline_thickness: f32,
    pub strikethrough_position: f32,
    pub strikethrough_thickness: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct FontDesignMetrics {
    pub units_per_em: i32,
    pub line_gap: i32,
    pub x_height: i32,
    pub cap_height: i32,
    pub horizontal_ascent: i32,
    pub vertical_ascent: i32,
    pub horizontal_descent: i32,
    pub vertical_descent: i32,
    pub horizontal_min_lsb: i32,
    pub vertical_min_lsb: i32,
    pub horizontal_min_tsb: i32,
    pub vertical_min_tsb: i32,
    pub horizontal_max_advance: i32,
    pub vertical_max_advance: i32,
    pub underline_position: i32,
    pub underline_thickness: i32,
    pub strikethrough_position: i32,
    pub strikethrough_thickness: i32,
}

#[repr(C)]
#[derive(Debug)]
pub struct TextMetrics {
    pub advance: PointD,
    pub bounding_box: BoxD,
}
