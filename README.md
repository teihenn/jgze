# jgze

A command-line tool for editing gzipped JSON files directly without manual decompression/compression steps.

[crates.io/crates/jgze](https://crates.io/crates/jgze)

## Features

- Edit `.json.gz` files directly using your preferred text editor
- Automatically formats compact JSON for better readability while editing
- Preserves the original format (compact/pretty) after saving
- Validates JSON syntax after editing
- Supports custom editor selection (defaults to vim)

## Installation

TODD: write this section

## Usage

Basic usage:

```bash
# Edit with default editor (vim)
jgze input.json.gz

# Edit with a specific editor
jgze -e nano input.json.gz
jgze --editor nano input.json.gz
```

## How it works

1. Decompresses the input `.json.gz` file
2. If the JSON is in compact format, formats it for better readability
3. Opens the formatted JSON in the specified editor
4. After editing and saving:
   - Validates the JSON syntax
   - If the original was in compact format, converts back to compact
   - Compresses and saves back to the original file

## Examples

Using test files in the `testdata` directory:

```bash
# Edit a compact JSON file
jgze testdata/test_compact.json.gz

# Edit a pretty-formatted JSON file
jgze testdata/test_pretty.json.gz
```

## Error Handling

The tool includes error handling for common scenarios:

- Invalid JSON syntax after editing
- File access issues
- Compression/decompression errors

Error messages are descriptive and include the specific failure point.
