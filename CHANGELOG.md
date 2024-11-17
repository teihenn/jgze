# Changelog

## [Unreleased]

## [Released]

### [1.1.1] - 2024-11-17

(Document-only update, but patch version bumped for release)

- Moved development-related documentation from README.md to DEVELOP.md.

### [1.1.0] - 2024-11-17

#### Feature

- Support for JSONL format

#### Fix

- JSON content order unintentionally changes after editing
- Rewrite JSON in compact format even if the original is in pretty format
- Differences occur even when a file is closed without changes

### [1.0.0] - 2024-11-17

#### Feature

- Initial release
- Basic functionality for editing .json.gz files
- Support for custom editor selection
