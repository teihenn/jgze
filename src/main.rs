use anyhow::{Context, Result};
use clap::Parser;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use serde_json::{to_string, to_string_pretty, Value};
use std::{
    fs::File,
    io::{Read, Write},
    process::Command,
};
use tempfile::NamedTempFile;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input .json.gz file
    file: String,

    /// Editor to use (defaults to vim)
    #[arg(short, long, default_value = "vim")]
    editor: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Decode JSON from GZIP file
    let file = File::open(&cli.file).context("Failed to open input file")?;
    let mut gz = GzDecoder::new(file);
    let mut content = String::new();
    gz.read_to_string(&mut content)
        .context("Failed to read gzipped content")?;

    // Try to parse as a single JSON first
    let json = match serde_json::from_str::<Value>(&content) {
        Ok(json) => vec![json],
        Err(_) => {
            // If single JSON parse fails, try parsing as JSONL
            content
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line| serde_json::from_str(line))
                .collect::<Result<_, _>>()
                .context("Failed to parse JSON lines")?
        }
    };

    // Format JSON
    let formatted = json
        .iter()
        .map(|j| to_string_pretty(j))
        .collect::<Result<Vec<_>, _>>()?
        .join("\n");

    // Create temp file and write formatted JSON
    let mut temp_file = NamedTempFile::new().context("Failed to create temp file")?;
    temp_file
        .write_all(formatted.as_bytes())
        .context("Failed to write to temp file")?;

    // Open in editor
    Command::new(&cli.editor)
        .arg(temp_file.path())
        .status()
        .context("Failed to open editor")?;

    // Read the edited file
    let mut edited_content = String::new();
    File::open(temp_file.path())
        .context("Failed to open temp file after editing")?
        .read_to_string(&mut edited_content)
        .context("Failed to read edited content")?;

    // Try to parse edited content
    let edited_json = match serde_json::from_str::<Value>(&edited_content) {
        Ok(json) => vec![json],
        Err(_) => {
            // If single JSON parse fails, try parsing as JSONL
            // First, combine multi-line JSON objects into single lines
            let mut current_json = String::new();
            let mut json_objects = Vec::new();
            let mut brace_count = 0;

            for line in edited_content.lines() {
                let open_braces = line.matches('{').count();
                let close_braces = line.matches('}').count();
                brace_count += open_braces as i32 - close_braces as i32;

                current_json.push_str(line.trim());

                if brace_count == 0 && !current_json.is_empty() {
                    json_objects.push(current_json.clone());
                    current_json.clear();
                }
            }

            // Parse each JSON object
            json_objects
                .into_iter()
                .map(|json_str| serde_json::from_str(&json_str))
                .collect::<Result<Vec<_>, _>>()
                .context("Invalid JSON after editing")?
        }
    };

    // Check if the original JSON was pretty-printed by looking for indentation
    let was_pretty = content
        .lines()
        .any(|line| line.starts_with("  ") || line.starts_with("\t"));

    let final_content = if was_pretty {
        edited_json
            .iter()
            .map(|j| to_string_pretty(j))
            .collect::<Result<Vec<_>, _>>()?
            .join("\n")
    } else {
        edited_json
            .iter()
            .map(|j| to_string(j))
            .collect::<Result<Vec<_>, _>>()?
            .join("\n")
    } + "\n"; // Add final newline

    // Save result as .json.gz
    let output_file = File::create(&cli.file).context("Failed to create output file")?;
    let mut encoder = GzEncoder::new(output_file, Compression::default());
    encoder
        .write_all(final_content.as_bytes())
        .context("Failed to write compressed output")?;

    Ok(())
}
