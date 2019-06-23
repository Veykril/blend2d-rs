# Change Log

## [Unreleased]

### Changed

- OutOfMemory errors now panic, unless returned by allocating `try_*` functions
- Various functions now return T instead of Result<T>

### Added

- DerefMut for Image and Array
- Remaining ImageDecoder and ImageEncoder functions
- ImageInfo struct
- Runtime Api
- Implement the From trait for Region

### Removed

- FindByName and FindByData functions on ImageCodec, use the
corresponding functions on Array<ImageCodec> instead

### Fixed

- incorrect interpretation of image stride

## [0.2.0] - 2018-05-11

### Changed

- Clone is now a weak reference clone
- Lots of api changes

### Added

- DeepClone trait for objects that are deep cloneable
- Most Font and Glyph related stuff
- Debug implementation for most types
- Travis CI

## [0.1.2] - 2018-04-17

### Changed

- Fix building on non-windows platforms

## [0.1.1] - 2018-04-16

### Changed

- Fix enum type mismatch on non-windows platforms

## 0.1.0 - 2018-04-16

### Added

- Initial release 


[Unreleased]: https://github.com/Veykril/blend2d-rs/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/Veykril/blend2d-rs/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/Veykril/blend2d-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/Veykril/blend2d-rs/compare/v0.1.0...v0.1.1