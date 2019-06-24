use std::{error, fmt};

use ffi::BLResultCode;

pub type Result<T> = std::result::Result<T, Error>;

#[inline]
pub(in crate) fn errcode_to_result(code: u32) -> Result<()> {
    match code as _ {
        0 => Ok(()),
        #[cold]
        BLResultCode::BL_ERROR_OUT_OF_MEMORY => panic!("memory allocation failed"),
        _ => Err(Error::from_errcode(code)),
    }
}

#[inline]
pub(in crate) fn expect_mem_err(code: u32) {
    match code as _ {
        #[cold]
        BLResultCode::BL_ERROR_OUT_OF_MEMORY => panic!("memory allocation failed"),
        _ => (),
    };
}

/// An error returned by a function if it was unable to succeed due to not being able to allocate
/// enough memory.
#[derive(Debug)]
pub struct OutOfMemory;

impl OutOfMemory {
    pub(in crate) fn from_errcode(code: u32) -> std::result::Result<(), Self> {
        if code == 0 {
            Ok(())
        } else {
            #[cold]
            Err(Self)
        }
    }
}

impl error::Error for OutOfMemory {}
impl fmt::Display for OutOfMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// An error returned by blend2d.
#[derive(Debug)]
pub enum Error {
    InvalidValue,
    InvalidState,
    InvalidHandle,
    ValueTooLarge,
    NotInitialized,
    NotImplemented,
    NotPermitted,
    Io,
    Busy,
    Interrupted,
    TryAgain,
    TimedOut,
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
    TooManyThreads,
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
    pub(super) fn from_errcode(errcode: u32) -> Self {
        match errcode as ffi::BLResultCode::Type {
            BLResultCode::BL_ERROR_INVALID_VALUE => Error::InvalidValue,
            BLResultCode::BL_ERROR_INVALID_STATE => Error::InvalidState,
            BLResultCode::BL_ERROR_INVALID_HANDLE => Error::InvalidHandle,
            BLResultCode::BL_ERROR_VALUE_TOO_LARGE => Error::ValueTooLarge,
            BLResultCode::BL_ERROR_NOT_INITIALIZED => Error::NotInitialized,
            BLResultCode::BL_ERROR_NOT_IMPLEMENTED => Error::NotImplemented,
            BLResultCode::BL_ERROR_NOT_PERMITTED => Error::NotPermitted,
            BLResultCode::BL_ERROR_IO => Error::Io,
            BLResultCode::BL_ERROR_BUSY => Error::Busy,
            BLResultCode::BL_ERROR_INTERRUPTED => Error::Interrupted,
            BLResultCode::BL_ERROR_TRY_AGAIN => Error::TryAgain,
            BLResultCode::BL_ERROR_TIMED_OUT => Error::TimedOut,
            BLResultCode::BL_ERROR_BROKEN_PIPE => Error::BrokenPipe,
            BLResultCode::BL_ERROR_INVALID_SEEK => Error::InvalidSeek,
            BLResultCode::BL_ERROR_SYMLINK_LOOP => Error::SymlinkLoop,
            BLResultCode::BL_ERROR_FILE_TOO_LARGE => Error::FileTooLarge,
            BLResultCode::BL_ERROR_ALREADY_EXISTS => Error::AlreadyExists,
            BLResultCode::BL_ERROR_ACCESS_DENIED => Error::AccessDenied,
            BLResultCode::BL_ERROR_MEDIA_CHANGED => Error::MediaChanged,
            BLResultCode::BL_ERROR_READ_ONLY_FS => Error::ReadOnlyFs,
            BLResultCode::BL_ERROR_NO_DEVICE => Error::NoDevice,
            BLResultCode::BL_ERROR_NO_ENTRY => Error::NoEntry,
            BLResultCode::BL_ERROR_NO_MEDIA => Error::NoMedia,
            BLResultCode::BL_ERROR_NO_MORE_DATA => Error::NoMoreData,
            BLResultCode::BL_ERROR_NO_MORE_FILES => Error::NoMoreFiles,
            BLResultCode::BL_ERROR_NO_SPACE_LEFT => Error::NoSpaceLeft,
            BLResultCode::BL_ERROR_NOT_EMPTY => Error::NotEmpty,
            BLResultCode::BL_ERROR_NOT_FILE => Error::NotFile,
            BLResultCode::BL_ERROR_NOT_DIRECTORY => Error::NotDirectory,
            BLResultCode::BL_ERROR_NOT_SAME_DEVICE => Error::NotSameDevice,
            BLResultCode::BL_ERROR_NOT_BLOCK_DEVICE => Error::NotBlockDevice,
            BLResultCode::BL_ERROR_INVALID_FILE_NAME => Error::InvalidFileName,
            BLResultCode::BL_ERROR_FILE_NAME_TOO_LONG => Error::FileNameTooLong,
            BLResultCode::BL_ERROR_TOO_MANY_OPEN_FILES => Error::TooManyOpenFiles,
            BLResultCode::BL_ERROR_TOO_MANY_OPEN_FILES_BY_OS => Error::TooManyOpenFilesByOs,
            BLResultCode::BL_ERROR_TOO_MANY_LINKS => Error::TooManyLinks,
            BLResultCode::BL_ERROR_TOO_MANY_THREADS => Error::TooManyThreads,
            BLResultCode::BL_ERROR_FILE_EMPTY => Error::FileEmpty,
            BLResultCode::BL_ERROR_OPEN_FAILED => Error::OpenFailed,
            BLResultCode::BL_ERROR_NOT_ROOT_DEVICE => Error::NotRootDevice,
            BLResultCode::BL_ERROR_UNKNOWN_SYSTEM_ERROR => Error::UnknownSystemError,
            BLResultCode::BL_ERROR_INVALID_SIGNATURE => Error::InvalidSignature,
            BLResultCode::BL_ERROR_INVALID_DATA => Error::InvalidData,
            BLResultCode::BL_ERROR_INVALID_STRING => Error::InvalidString,
            BLResultCode::BL_ERROR_DATA_TRUNCATED => Error::DataTruncated,
            BLResultCode::BL_ERROR_DATA_TOO_LARGE => Error::DataTooLarge,
            BLResultCode::BL_ERROR_DECOMPRESSION_FAILED => Error::DecompressionFailed,
            BLResultCode::BL_ERROR_INVALID_GEOMETRY => Error::InvalidGeometry,
            BLResultCode::BL_ERROR_NO_MATCHING_VERTEX => Error::NoMatchingVertex,
            BLResultCode::BL_ERROR_NO_MATCHING_COOKIE => Error::NoMatchingCookie,
            BLResultCode::BL_ERROR_NO_STATES_TO_RESTORE => Error::NoStatesToRestore,
            BLResultCode::BL_ERROR_IMAGE_TOO_LARGE => Error::ImageTooLarge,
            BLResultCode::BL_ERROR_IMAGE_NO_MATCHING_CODEC => Error::ImageNoMatchingCodec,
            BLResultCode::BL_ERROR_IMAGE_UNKNOWN_FILE_FORMAT => Error::ImageUnknownFileFormat,
            BLResultCode::BL_ERROR_IMAGE_DECODER_NOT_PROVIDED => Error::ImageDecoderNotProvided,
            BLResultCode::BL_ERROR_IMAGE_ENCODER_NOT_PROVIDED => Error::ImageEncoderNotProvided,
            BLResultCode::BL_ERROR_PNG_MULTIPLE_IHDR => Error::PngMultipleIHDR,
            BLResultCode::BL_ERROR_PNG_INVALID_IDAT => Error::PngInvalidIDAT,
            BLResultCode::BL_ERROR_PNG_INVALID_IEND => Error::PngInvalidIEND,
            BLResultCode::BL_ERROR_PNG_INVALID_PLTE => Error::PngInvalidPLTE,
            BLResultCode::BL_ERROR_PNG_INVALID_TRNS => Error::PngInvalidTRNS,
            BLResultCode::BL_ERROR_PNG_INVALID_FILTER => Error::PngInvalidFilter,
            BLResultCode::BL_ERROR_JPEG_UNSUPPORTED_FEATURE => Error::JpegUnsupportedFeature,
            BLResultCode::BL_ERROR_JPEG_INVALID_SOS => Error::JpegInvalidSOS,
            BLResultCode::BL_ERROR_JPEG_INVALID_SOF => Error::JpegInvalidSOF,
            BLResultCode::BL_ERROR_JPEG_MULTIPLE_SOF => Error::JpegMultipleSOF,
            BLResultCode::BL_ERROR_JPEG_UNSUPPORTED_SOF => Error::JpegUnsupportedSOF,
            BLResultCode::BL_ERROR_FONT_NO_CHARACTER_MAPPING => Error::FontNoCharacterMapping,
            BLResultCode::BL_ERROR_FONT_MISSING_IMPORTANT_TABLE => Error::FontMissingImportantTable,
            BLResultCode::BL_ERROR_FONT_FEATURE_NOT_AVAILABLE => Error::FontFeatureNotAvailable,
            BLResultCode::BL_ERROR_FONT_CFF_INVALID_DATA => Error::FontCffInvalidData,
            BLResultCode::BL_ERROR_FONT_PROGRAM_TERMINATED => Error::FontProgramTerminated,
            BLResultCode::BL_ERROR_INVALID_GLYPH => Error::InvalidGlyph,
            _ => unreachable!("Custom fallback type"),
        }
    }
}

impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
