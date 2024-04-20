# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2024-4-21

### Changed

- `tui` related integration is replaced by `ratatui`, feature and type names are changed. If you still want use this crate with old `tui` crate, please stick to `0.3.0`. (Thanks [@joshka])

## [0.3.0] - 2022-10-19

### Added

- Add `crossterm` feature for enable builtin generators for crossterm crate: `CrosstermCommandsGenerator`.

## [0.2.1] - 2022-8-18

### Added

- `ParseError`, `GeneratorInfallible` now implements std `Error` trait.
- `Error` now implements `Clone` trait if `GE` is `Clone`.

### Changed

- Remove the dependency on `thiserror`.

## [0.2.0] - 2022-08-17

### Added

- `parser` module, contains parse function, `Item` type and `Error`.
- `Generator` trait for custom generators.
- `Tag` type for standard tag variants.
- `TagConvertor` trait for convert raw tag string into Tag type with custom color and modifier type.
- `generator::helper` module for helper functions to write generator, including `unescape`, `CustomTagParser`, `GeneratorInfallible`, `flatten` etc.
- Add `tui` feature for enable builtin generators for tui crate: `TuiTextGenerator`.
- Add `ansi` feature for enable built in generator for ansi terminal string: `ANSIStringsGenerator`.
- `LocateError` trait for get location of error in source text.
- `compile_with` entry function for use a custom configured generator.

### Changed

- Entry function renamed from `parse` to `compile`, use default instance of a generator type.
- Root `Error` type changed from `(usize, usize)` to a enum type, for better error reporting.
- Bump `tui` version to `0.19`.

## [0.1.1] - 2022-08-14

### Fixed

- backslash(`\`) is missing from the parsed result.

## [0.1.0] - 2022-08-13

### Added

- First release.

[Unreleased]: https://github.com/7sDream/tui-markup/compare/v0.4.0..HEAD
[0.4.0]: https://github.com/7sDream/tui-markup/compare/v0.3.0..v0.4.0
[0.3.0]: https://github.com/7sDream/tui-markup/compare/v0.2.1..v0.3.0
[0.2.1]: https://github.com/7sDream/tui-markup/compare/v0.2.0..v0.2.1
[0.2.0]: https://github.com/7sDream/tui-markup/compare/v0.1.1..v0.2.0
[0.1.1]: https://github.com/7sDream/tui-markup/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/7sDream/tui-markup/releases/tag/v0.1.0

[@joshka]: https://github.com/joshka
