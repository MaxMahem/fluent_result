# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.0] - 2025/12/09

### Added

- Added `assert_true()` and `assert_false()` methods to `bool::dbg::Expect` and `bool::rls::Expect` traits
  - These methods use fixed panic messages, similar to Rust's `unwrap()` pattern
- Added `assert_none()` method to `expect::dbg::ExpectNone` and `expect::rls::ExpectNone` traits
  - Uses a fixed panic message, similar to Rust's `unwrap()` pattern

### Changed

- **BREAKING**: Renamed `unwrap_none()` to `assert_none()` in `ExpectNone` traits
  - **Migration**: Replace `unwrap_none()` calls with `assert_none()`

- **BREAKING**: Refactored `bool::Expect` trait module structure:
  - `bool::dbg::Expect` - Only panics in debug mode (`debug_assert!` behavior)
  - `bool::rls::Expect` - Always panics in all build modes (`assert!` behavior)
  
  **Migration**: Update imports from `use fluent_result::bool::expect::dbg::Expect` to `use fluent_result::bool::dbg::Expect`, or from `use fluent_result::bool::expect::rls::Expect` to `use fluent_result::bool::rls::Expect`

- **BREAKING**: Refactored `ExpectNone` trait module structure:
  - `expect::dbg::ExpectNone` - Only panics in debug mode (`debug_assert!` behavior)
  - `expect::rls::ExpectNone` - Always panics in all build modes (`assert!` behavior)
  
  **Migration**: Update imports from `use fluent_result::expect::expect_none::dbg::ExpectNone` to `use fluent_result::expect::dbg::ExpectNone`, or from `use fluent_result::expect::expect_none::rls::ExpectNone` to `use fluent_result::expect::rls::ExpectNone`

## [0.8.4] - 2025/12/08

### Added

- Added `#[track_caller]` attribute to panicing trait methods: 
- - `bool::Expect::expect_true` 
- - `bool::Expect::expect_false`
- - `expect::ExpectNone::expect_none`
- - `expect::ExpectNone::unwrap_none`

## [0.8.3] - 2025/12/07

### Added

- Added `expect_true` and `expect_false` traits for bool, for asserting expected `true` and `false` values

## [0.8.2] - 2025/12/07

### Changed

- Added `#[inline]` attribute to trait methods for better compiler optimization

## [0.8.1] - 2025/12/04

### Added

- Added `then_err_with` trait, for lazily creating an `Err` value when true

## [0.8.0] - 2025/11/27

### Removed

- Removed `UnitResult` and `InfallibleResult` type aliases

### Changed

- Moved `FlattenErr` and `BoxErr` traits to `nested` module
- Moved `ExpectNone` and `UnwrapNever` traits to `expect` module
- Moved `IntoOption` and `IntoResult` traits to `into` module
- Moved `SinkOption` and `SinkResult` traits to `sink` module
- Sealed `FlattenErr` and `BoxErr` traits

## [0.7.0] - 2025/11/27

### Added

- Added `BoxErr` trait, for handling nested `Result` types with `Box<dyn Error>`
- Added `FlattenErr` trait, for handling nested `Result` types with a flattened error type

### Changed

- Consolidated `ThenErr` and `ThenNone` traits into a single `bool` module file

### Removed

- Removed `log` series of traits. Use `sink` series of traits instead.
- Removed `OptionMapTo` and `ResultMapTo` traits. Use standard `Option::map`, `Result::map`, and `Result::map_err` methods instead.

## [0.6.3] - 2025/11/14

### Added

- Added `ThenErr` trait, for transforming a `bool` into an `Err` when true

## [0.6.2] - 2025/11/13

### Added

- Added `ExpectNone` trait, for unwrapping a `None` variant

## [0.6.1] - 2025/10/31

### Added

- Added `SinkOption` trait
- Added `ThenNone` trait - for transforming a `bool` into an `None` when true
- Added `SinkResult` trait

### Removed

- Removed `HandleError` trait. `Result` errors can be handled via `result.ok().sink(sink_fn)` or `result.err().sink(sink_fn)`
- Removed `UnwrapResult` trait. `Result` errors can be transformed via `result.unwrap().into_ok()` or `result.expect(msg).into_ok()`. `into_ok` can take a type parameter to specify the error type, if necessary.

## [0.5.0] - 2025/10/31

Started change tracking
