#CHANGELOG
All notable changes will be documented in this file

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Released]

## [2.0.0] - 2020-12-16
### Added
- Official stable release
- CRUD atomic operations
- async/await
- a changelog file
- cargo-deny file to check for licensing compatiblity

### Changed
- `futures` crate to `futures-lite`
- removed `blocking` crate
- swapped file system access to use `async-fs`