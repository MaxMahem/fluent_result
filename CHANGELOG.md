# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
