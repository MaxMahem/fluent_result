# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

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
