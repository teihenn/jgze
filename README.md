# jgze

A lightweight CLI tool to edit .json.gz files seamlessly.
Edit gzipped JSON files directly in your favorite editor
with automatic formatting and compression handling.

[crates.io/crates/jgze](https://crates.io/crates/jgze)

## Features

- Edit `.json.gz` files directly using your preferred text editor
- Automatically formats compact JSON for better readability while editing
- Preserves the original format (compact/pretty) after saving
- Validates JSON syntax after editing
- Supports custom editor selection (defaults to vim)

## Supported JSON Formats

The tool supports three JSON formats:

1. Compact JSON (single line)

```json
{"name":"John","age":30,"city":"Tokyo"}
```

2. Pretty-printed JSON

```json
{
  "name": "John",
  "age": 30,
  "city": "Tokyo"
}
```

3. JSON Lines (JSONL)

```json
{"name":"John","age":30}
{"name":"Alice","age":25}
{"name":"Bob","age":35}
```

For better readability, all formats are automatically converted to
pretty-printed format when opened in the editor.
After saving, the file is converted back to its original format before compression.

For example, if you open a compact JSON file:

1. Original: `{"name":"John","age":30,"city":"Tokyo"}`
2. While editing:

```json
{
"name": "John",
"age": 30,
"city": "Tokyo"
}
```

Suppose you edited this as follows:

```json
{
"name": "John",
"age": 30
}
```

3. After saving: `{"name":"John","age":30}`

## Usage

```bash
jgze [OPTIONS] <FILE>
```

| Option | Short | Description |
|--------|-------|-------------|
| --editor \<EDITOR\> | -e | Specify the editor to use (default: vim) |
| --help | -h | Display help message |
| --version | -v | Display version information |

### Examples

```bash
# Edit with default editor (vim)
jgze input.json.gz

# Edit with a specific editor
jgze -e nano input.json.gz

# Show version information
jgze -v

# Show help message
jgze -h
```

The usage can be verified using the test data in the testdata directory.

## Installation

### Using cargo

#### install, update

```bash
cargo install jgze
```

#### uninstall

```bash
cargo uninstall jgze
```

### From GitHub releases

#### install, update

- Linux (x86_64)

```bash
tar xz -C /usr/local/bin < <(curl -L https://github.com/teihenn/jgze/releases/latest/download/jgze-x86_64-unknown-linux-gnu.tar.gz)
```

- macOS (x86_64)

```bash
tar xz -C /usr/local/bin < <(curl -L https://github.com/teihenn/jgze/releases/latest/download/jgze-x86_64-apple-darwin.tar.gz)
```

- macOS (aarch64)

```bash
tar xz -C /usr/local/bin < <(curl -L https://github.com/teihenn/jgze/releases/latest/download/jgze-aarch64-apple-darwin.tar.gz)
```

- Windows (x86_64)

1. Download the latest release
    - https://github.com/teihenn/jgze/releases/latest/download/jgze-x86_64-pc-windows-msvc.zip
2. Extract the downloaded ZIP file
3. Create a directory and place jgze.exe (e.g., `C:\Program Files\jgze`)
4. Add the installation directory to the system PATH by running PowerShell as administrator

#### uninstall

```bash
ls -l /usr/local/bin/jgze
sudo rm /usr/local/bin/jgze
```

### From source

#### install, update

```bash
git clone https://github.com/teihenn/jgze
cd jgze
cargo install --path .
```

#### uninstall

```bash
cargo uninstall jgze
```

## How it works

1. Decompresses the input `.json.gz` file
2. Detects the JSON format and processes accordingly:
   - Compact JSON: Formats for better readability
   - JSONL: Formats each line individually
   - Pretty-printed JSON: Keeps as is
3. Opens the formatted JSON in the specified editor
4. After editing and saving:
   - Validates the JSON syntax
   - Converts based on original format:
     - For compact JSON: Compresses to single line
     - For JSONL: Compresses each object to single line
     - For pretty-printed JSON: Preserves formatting
   - Compresses with gzip and saves back to the original file
