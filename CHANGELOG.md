# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added `Sink` trait

### Removed

- Removed `HandleError` trait. `Result` errors can be handled via `ok().sink(sink_fn)` or `err().sink(sink_fn)`

## [0.5.0] - 10/31/2025

Started change tracking
