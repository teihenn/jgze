# jgze

A command-line tool for editing gzipped JSON files
directly without manual decompression/compression steps.

[crates.io/crates/jgze](https://crates.io/crates/jgze)

## Features

- Edit `.json.gz` files directly using your preferred text editor
- Automatically formats compact JSON for better readability while editing
- Preserves the original format (compact/pretty) after saving
- Validates JSON syntax after editing
- Supports custom editor selection (defaults to vim)

## Installation

Using cargo:

```bash
cargo install jgze
```

From source:

```bash
git clone https://github.com/teihenn/jgze
cd jgze
cargo install --path .
```

## Usage

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

## For Developers

### Release Process

1. Update version in `Cargo.toml`:

```toml
[package]
name = "jgze"
version = "1.1.0"  # Update this version
```

2. Update `CHANGELOG.md`:

```markdown
# Changelog

## [1.1.0] - 2024-11-26
- Add your changes here

## [1.0.0] - 2024-11-17
```

3. Commit changes:

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "Prepare for v1.1.0 release"
```

4. Create and push a new tag:

```bash
git tag v1.1.0
git push origin main v1.1.0
```

This will trigger the GitHub Actions workflow that:

- Creates a new GitHub release
- Builds binaries
- Uploads the binaries to the release page

### Supported Platforms

The automated builds create binaries for:

| Platform | Architecture | Target Triple | File Name |
|----------|-------------|---------------|-----------|
| Linux | x86_64 | x86_64-unknown-linux-gnu | jgze-x86_64-linux.tar.gz |
| macOS | x86_64 | x86_64-apple-darwin | jgze-x86_64-darwin.tar.gz |
| macOS | aarch64 | aarch64-apple-darwin | jgze-aarch64-darwin.tar.gz |
| Windows | x86_64 | x86_64-pc-windows-msvc | jgze-x86_64-windows.zip |

5. Publish to crates.io:

```bash
cargo login
cargo publish
```
