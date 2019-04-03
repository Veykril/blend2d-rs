use core::mem;
use std::{error, fmt};

pub type Result<T> = std::result::Result<T, Error>;

fn u32_to_bl_res(code: u32) -> Option<ffi::BLResultCode> {
    if code == 0
        || ffi::BLResultCode::BL_ERROR_INVALID_GLYPH as u32 >= code
            && ffi::BLResultCode::BL_ERROR_START_INDEX as u32 <= code
    {
        Some(unsafe { mem::transmute(code) })
    } else {
        None
    }
}

pub(in crate) fn errcode_to_result(code: u32) -> Result<()> {
    match u32_to_bl_res(code) {
        Some(ffi::BLResultCode::BL_SUCCESS) => Ok(()),
        Some(e) => Err(Error::from(e)),
        None => Err(Error::InvalidValue),
    }
}

#[derive(Debug)]
pub enum Error {
    OutOfMemory,
    InvalidValue,
    InvalidState,
    InvalidHandle,
    ValueTooLarge,
    NotIitialized,
    NotImplemented,
    NotPermitted,
    Io,
    Busy,
    Interrupted,
    TryAgain,
    BrokenPipe,
    InvalidSeek,
    SymlinkLoop,
    FileTooLarge,
    AlreadyExists,
    AccessDenied,
    MediaChanged,
    ReadOnlyFs,
    NoDevice,
    NoEntry,
    NoMedia,
    NoMoreData,
    NoMoreFiles,
    NoSpaceLeft,
    NotEmpty,
    NotFile,
    NotDirectory,
    NotSameDevice,
    NotBlockDevice,
    InvalidFileName,
    FileNameTooLong,
    TooManyOpenFiles,
    TooManyOpenFilesByOs,
    TooManyLinks,
    FileEmpty,
    OpenFailed,
    NotRootDevice,
    UnknownSystemError,
    InvalidSignature,
    InvalidData,
    InvalidString,
    DataTruncated,
    DataTooLarge,
    DecompressionFailed,
    InvalidGeometry,
    NoMatchingVertex,
    NoMatchingCookie,
    NoStatesToRestore,
    ImageTooLarge,
    ImageNoMatchingCodec,
    ImageUnknownFileFormat,
    ImageDecoderNotProvided,
    ImageEncoderNotProvided,
    PngMultipleIHDR,
    PngInvalidIDAT,
    PngInvalidIEND,
    PngInvalidPLTE,
    PngInvalidTRNS,
    PngInvalidFilter,
    JpegUnsupportedFeature,
    JpegInvalidSOS,
    JpegInvalidSOF,
    JpegMultipleSOF,
    JpegUnsupportedSOF,
    FontNoCharacterMapping,
    FontMissingImportantTable,
    FontFeatureNotAvailable,
    FontCffInvalidData,
    FontProgramTerminated,
    InvalidGlyph,
}

impl Error {
    fn from(errcode: ffi::BLResultCode) -> Self {
        match errcode {
            ffi::BLResultCode::BL_ERROR_OUT_OF_MEMORY => Error::OutOfMemory,
            ffi::BLResultCode::BL_ERROR_INVALID_VALUE => Error::InvalidValue,
            ffi::BLResultCode::BL_ERROR_INVALID_STATE => Error::InvalidState,
            ffi::BLResultCode::BL_ERROR_INVALID_HANDLE => Error::InvalidHandle,
            ffi::BLResultCode::BL_ERROR_VALUE_TOO_LARGE => Error::ValueTooLarge,
            ffi::BLResultCode::BL_ERROR_NOT_INITIALIZED => Error::NotIitialized,
            ffi::BLResultCode::BL_ERROR_NOT_IMPLEMENTED => Error::NotImplemented,
            ffi::BLResultCode::BL_ERROR_NOT_PERMITTED => Error::NotPermitted,
            ffi::BLResultCode::BL_ERROR_IO => Error::Io,
            ffi::BLResultCode::BL_ERROR_BUSY => Error::Busy,
            ffi::BLResultCode::BL_ERROR_INTERRUPTED => Error::Interrupted,
            ffi::BLResultCode::BL_ERROR_TRY_AGAIN => Error::TryAgain,
            ffi::BLResultCode::BL_ERROR_BROKEN_PIPE => Error::BrokenPipe,
            ffi::BLResultCode::BL_ERROR_INVALID_SEEK => Error::InvalidSeek,
            ffi::BLResultCode::BL_ERROR_SYMLINK_LOOP => Error::SymlinkLoop,
            ffi::BLResultCode::BL_ERROR_FILE_TOO_LARGE => Error::FileTooLarge,
            ffi::BLResultCode::BL_ERROR_ALREADY_EXISTS => Error::AlreadyExists,
            ffi::BLResultCode::BL_ERROR_ACCESS_DENIED => Error::AccessDenied,
            ffi::BLResultCode::BL_ERROR_MEDIA_CHANGED => Error::MediaChanged,
            ffi::BLResultCode::BL_ERROR_READ_ONLY_FS => Error::ReadOnlyFs,
            ffi::BLResultCode::BL_ERROR_NO_DEVICE => Error::NoDevice,
            ffi::BLResultCode::BL_ERROR_NO_ENTRY => Error::NoEntry,
            ffi::BLResultCode::BL_ERROR_NO_MEDIA => Error::NoMedia,
            ffi::BLResultCode::BL_ERROR_NO_MORE_DATA => Error::NoMoreData,
            ffi::BLResultCode::BL_ERROR_NO_MORE_FILES => Error::NoMoreFiles,
            ffi::BLResultCode::BL_ERROR_NO_SPACE_LEFT => Error::NoSpaceLeft,
            ffi::BLResultCode::BL_ERROR_NOT_EMPTY => Error::NotEmpty,
            ffi::BLResultCode::BL_ERROR_NOT_FILE => Error::NotFile,
            ffi::BLResultCode::BL_ERROR_NOT_DIRECTORY => Error::NotDirectory,
            ffi::BLResultCode::BL_ERROR_NOT_SAME_DEVICE => Error::NotSameDevice,
            ffi::BLResultCode::BL_ERROR_NOT_BLOCK_DEVICE => Error::NotBlockDevice,
            ffi::BLResultCode::BL_ERROR_INVALID_FILE_NAME => Error::InvalidFileName,
            ffi::BLResultCode::BL_ERROR_FILE_NAME_TOO_LONG => Error::FileNameTooLong,
            ffi::BLResultCode::BL_ERROR_TOO_MANY_OPEN_FILES => Error::TooManyOpenFiles,
            ffi::BLResultCode::BL_ERROR_TOO_MANY_OPEN_FILES_BY_OS => Error::TooManyOpenFilesByOs,
            ffi::BLResultCode::BL_ERROR_TOO_MANY_LINKS => Error::TooManyLinks,
            ffi::BLResultCode::BL_ERROR_FILE_EMPTY => Error::FileEmpty,
            ffi::BLResultCode::BL_ERROR_OPEN_FAILED => Error::OpenFailed,
            ffi::BLResultCode::BL_ERROR_NOT_ROOT_DEVICE => Error::NotRootDevice,
            ffi::BLResultCode::BL_ERROR_UNKNOWN_SYSTEM_ERROR => Error::UnknownSystemError,
            ffi::BLResultCode::BL_ERROR_INVALID_SIGNATURE => Error::InvalidSignature,
            ffi::BLResultCode::BL_ERROR_INVALID_DATA => Error::InvalidData,
            ffi::BLResultCode::BL_ERROR_INVALID_STRING => Error::InvalidString,
            ffi::BLResultCode::BL_ERROR_DATA_TRUNCATED => Error::DataTruncated,
            ffi::BLResultCode::BL_ERROR_DATA_TOO_LARGE => Error::DataTooLarge,
            ffi::BLResultCode::BL_ERROR_DECOMPRESSION_FAILED => Error::DecompressionFailed,
            ffi::BLResultCode::BL_ERROR_INVALID_GEOMETRY => Error::InvalidGeometry,
            ffi::BLResultCode::BL_ERROR_NO_MATCHING_VERTEX => Error::NoMatchingVertex,
            ffi::BLResultCode::BL_ERROR_NO_MATCHING_COOKIE => Error::NoMatchingCookie,
            ffi::BLResultCode::BL_ERROR_NO_STATES_TO_RESTORE => Error::NoStatesToRestore,
            ffi::BLResultCode::BL_ERROR_IMAGE_TOO_LARGE => Error::ImageTooLarge,
            ffi::BLResultCode::BL_ERROR_IMAGE_NO_MATCHING_CODEC => Error::ImageNoMatchingCodec,
            ffi::BLResultCode::BL_ERROR_IMAGE_UNKNOWN_FILE_FORMAT => Error::ImageUnknownFileFormat,
            ffi::BLResultCode::BL_ERROR_IMAGE_DECODER_NOT_PROVIDED => {
                Error::ImageDecoderNotProvided
            }
            ffi::BLResultCode::BL_ERROR_IMAGE_ENCODER_NOT_PROVIDED => {
                Error::ImageEncoderNotProvided
            }
            ffi::BLResultCode::BL_ERROR_PNG_MULTIPLE_IHDR => Error::PngMultipleIHDR,
            ffi::BLResultCode::BL_ERROR_PNG_INVALID_IDAT => Error::PngInvalidIDAT,
            ffi::BLResultCode::BL_ERROR_PNG_INVALID_IEND => Error::PngInvalidIEND,
            ffi::BLResultCode::BL_ERROR_PNG_INVALID_PLTE => Error::PngInvalidPLTE,
            ffi::BLResultCode::BL_ERROR_PNG_INVALID_TRNS => Error::PngInvalidTRNS,
            ffi::BLResultCode::BL_ERROR_PNG_INVALID_FILTER => Error::PngInvalidFilter,
            ffi::BLResultCode::BL_ERROR_JPEG_UNSUPPORTED_FEATURE => Error::JpegUnsupportedFeature,
            ffi::BLResultCode::BL_ERROR_JPEG_INVALID_SOS => Error::JpegInvalidSOS,
            ffi::BLResultCode::BL_ERROR_JPEG_INVALID_SOF => Error::JpegInvalidSOF,
            ffi::BLResultCode::BL_ERROR_JPEG_MULTIPLE_SOF => Error::JpegMultipleSOF,
            ffi::BLResultCode::BL_ERROR_JPEG_UNSUPPORTED_SOF => Error::JpegUnsupportedSOF,
            ffi::BLResultCode::BL_ERROR_FONT_NO_CHARACTER_MAPPING => Error::FontNoCharacterMapping,
            ffi::BLResultCode::BL_ERROR_FONT_MISSING_IMPORTANT_TABLE => {
                Error::FontMissingImportantTable
            }
            ffi::BLResultCode::BL_ERROR_FONT_FEATURE_NOT_AVAILABLE => {
                Error::FontFeatureNotAvailable
            }
            ffi::BLResultCode::BL_ERROR_FONT_CFF_INVALID_DATA => Error::FontCffInvalidData,
            ffi::BLResultCode::BL_ERROR_FONT_PROGRAM_TERMINATED => Error::FontProgramTerminated,
            ffi::BLResultCode::BL_ERROR_INVALID_GLYPH => Error::InvalidGlyph,
            ffi::BLResultCode::BL_SUCCESS => unreachable!(),
        }
    }
}

impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
