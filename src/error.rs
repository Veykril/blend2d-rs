use std::{error, fmt};

use ffi::BLResultCode;

pub type Result<T> = std::result::Result<T, Error>;

#[inline]
pub(in crate) fn errcode_to_result(code: u32) -> Result<()> {
    match code as _ {
        0 => Ok(()),
        #[cold]
        BLResultCode::BL_ERROR_OUT_OF_MEMORY => panic!("memory allocation failed"),
        _ => Err(error_from_errcode(code)),
    }
}

#[inline]
pub(in crate) fn expect_mem_err(code: u32) {
    match code as _ {
        #[cold]
        BLResultCode::BL_ERROR_OUT_OF_MEMORY => panic!("memory allocation failed"),
        _ => (),
    }
}

/// An error returned by a function if it was unable to succeed due to not being
/// able to allocate enough memory.
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

/// An error returned by blend2d due to faulty png data.
#[derive(Debug)]
pub enum PngError {
    MultipleIHDR,
    InvalidIDAT,
    InvalidIEND,
    InvalidPLTE,
    InvalidTRNS,
    InvalidFilter,
}

/// An error returned by blend2d due to faulty jpeg data.
#[derive(Debug)]
pub enum JpegError {
    UnsupportedFeature,
    InvalidSOS,
    InvalidSOF,
    MultipleSOF,
    UnsupportedSOF,
}

/// A font error returned by blend2d.
#[derive(Debug)]
pub enum FontError {
    NoCharacterMapping,
    MissingImportantTable,
    FeatureNotAvailable,
    CffInvalidData,
    ProgramTerminated,
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
    Png(PngError),
    Jpeg(JpegError),
    Font(FontError),
    InvalidGlyph,
}

pub(super) fn error_from_errcode(errcode: u32) -> Error {
    use BLResultCode::*;
    match errcode as ffi::BLResultCode::Type {
        BL_ERROR_INVALID_VALUE => Error::InvalidValue,
        BL_ERROR_INVALID_STATE => Error::InvalidState,
        BL_ERROR_INVALID_HANDLE => Error::InvalidHandle,
        BL_ERROR_VALUE_TOO_LARGE => Error::ValueTooLarge,
        BL_ERROR_NOT_INITIALIZED => Error::NotInitialized,
        BL_ERROR_NOT_IMPLEMENTED => Error::NotImplemented,
        BL_ERROR_NOT_PERMITTED => Error::NotPermitted,
        BL_ERROR_IO => Error::Io,
        BL_ERROR_BUSY => Error::Busy,
        BL_ERROR_INTERRUPTED => Error::Interrupted,
        BL_ERROR_TRY_AGAIN => Error::TryAgain,
        BL_ERROR_TIMED_OUT => Error::TimedOut,
        BL_ERROR_BROKEN_PIPE => Error::BrokenPipe,
        BL_ERROR_INVALID_SEEK => Error::InvalidSeek,
        BL_ERROR_SYMLINK_LOOP => Error::SymlinkLoop,
        BL_ERROR_FILE_TOO_LARGE => Error::FileTooLarge,
        BL_ERROR_ALREADY_EXISTS => Error::AlreadyExists,
        BL_ERROR_ACCESS_DENIED => Error::AccessDenied,
        BL_ERROR_MEDIA_CHANGED => Error::MediaChanged,
        BL_ERROR_READ_ONLY_FS => Error::ReadOnlyFs,
        BL_ERROR_NO_DEVICE => Error::NoDevice,
        BL_ERROR_NO_ENTRY => Error::NoEntry,
        BL_ERROR_NO_MEDIA => Error::NoMedia,
        BL_ERROR_NO_MORE_DATA => Error::NoMoreData,
        BL_ERROR_NO_MORE_FILES => Error::NoMoreFiles,
        BL_ERROR_NO_SPACE_LEFT => Error::NoSpaceLeft,
        BL_ERROR_NOT_EMPTY => Error::NotEmpty,
        BL_ERROR_NOT_FILE => Error::NotFile,
        BL_ERROR_NOT_DIRECTORY => Error::NotDirectory,
        BL_ERROR_NOT_SAME_DEVICE => Error::NotSameDevice,
        BL_ERROR_NOT_BLOCK_DEVICE => Error::NotBlockDevice,
        BL_ERROR_INVALID_FILE_NAME => Error::InvalidFileName,
        BL_ERROR_FILE_NAME_TOO_LONG => Error::FileNameTooLong,
        BL_ERROR_TOO_MANY_OPEN_FILES => Error::TooManyOpenFiles,
        BL_ERROR_TOO_MANY_OPEN_FILES_BY_OS => Error::TooManyOpenFilesByOs,
        BL_ERROR_TOO_MANY_LINKS => Error::TooManyLinks,
        BL_ERROR_TOO_MANY_THREADS => Error::TooManyThreads,
        BL_ERROR_FILE_EMPTY => Error::FileEmpty,
        BL_ERROR_OPEN_FAILED => Error::OpenFailed,
        BL_ERROR_NOT_ROOT_DEVICE => Error::NotRootDevice,
        BL_ERROR_UNKNOWN_SYSTEM_ERROR => Error::UnknownSystemError,
        BL_ERROR_INVALID_SIGNATURE => Error::InvalidSignature,
        BL_ERROR_INVALID_DATA => Error::InvalidData,
        BL_ERROR_INVALID_STRING => Error::InvalidString,
        BL_ERROR_DATA_TRUNCATED => Error::DataTruncated,
        BL_ERROR_DATA_TOO_LARGE => Error::DataTooLarge,
        BL_ERROR_DECOMPRESSION_FAILED => Error::DecompressionFailed,
        BL_ERROR_INVALID_GEOMETRY => Error::InvalidGeometry,
        BL_ERROR_NO_MATCHING_VERTEX => Error::NoMatchingVertex,
        BL_ERROR_NO_MATCHING_COOKIE => Error::NoMatchingCookie,
        BL_ERROR_NO_STATES_TO_RESTORE => Error::NoStatesToRestore,
        BL_ERROR_IMAGE_TOO_LARGE => Error::ImageTooLarge,
        BL_ERROR_IMAGE_NO_MATCHING_CODEC => Error::ImageNoMatchingCodec,
        BL_ERROR_IMAGE_UNKNOWN_FILE_FORMAT => Error::ImageUnknownFileFormat,
        BL_ERROR_IMAGE_DECODER_NOT_PROVIDED => Error::ImageDecoderNotProvided,
        BL_ERROR_IMAGE_ENCODER_NOT_PROVIDED => Error::ImageEncoderNotProvided,

        BL_ERROR_PNG_MULTIPLE_IHDR => Error::Png(PngError::MultipleIHDR),
        BL_ERROR_PNG_INVALID_IDAT => Error::Png(PngError::InvalidIDAT),
        BL_ERROR_PNG_INVALID_IEND => Error::Png(PngError::InvalidIEND),
        BL_ERROR_PNG_INVALID_PLTE => Error::Png(PngError::InvalidPLTE),
        BL_ERROR_PNG_INVALID_TRNS => Error::Png(PngError::InvalidTRNS),
        BL_ERROR_PNG_INVALID_FILTER => Error::Png(PngError::InvalidFilter),

        BL_ERROR_JPEG_UNSUPPORTED_FEATURE => Error::Jpeg(JpegError::UnsupportedFeature),
        BL_ERROR_JPEG_INVALID_SOS => Error::Jpeg(JpegError::InvalidSOS),
        BL_ERROR_JPEG_INVALID_SOF => Error::Jpeg(JpegError::InvalidSOF),
        BL_ERROR_JPEG_MULTIPLE_SOF => Error::Jpeg(JpegError::MultipleSOF),
        BL_ERROR_JPEG_UNSUPPORTED_SOF => Error::Jpeg(JpegError::UnsupportedSOF),

        BL_ERROR_FONT_NO_CHARACTER_MAPPING => Error::Font(FontError::NoCharacterMapping),
        BL_ERROR_FONT_MISSING_IMPORTANT_TABLE => Error::Font(FontError::MissingImportantTable),
        BL_ERROR_FONT_FEATURE_NOT_AVAILABLE => Error::Font(FontError::FeatureNotAvailable),
        BL_ERROR_FONT_CFF_INVALID_DATA => Error::Font(FontError::CffInvalidData),
        BL_ERROR_FONT_PROGRAM_TERMINATED => Error::Font(FontError::ProgramTerminated),

        BL_ERROR_INVALID_GLYPH => Error::InvalidGlyph,
        _ => unreachable!("Custom fallback type"),
    }
}

impl error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
