use anyhow::{Context, Result};
use clap::Parser;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use serde_json::{from_reader, to_string, to_string_pretty, Value};
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
    let gz = GzDecoder::new(file);
    let json: Value = from_reader(gz).context("Failed to parse JSON")?;

    // Format JSON
    let formatted = to_string_pretty(&json).context("Failed to format JSON")?;

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

    // Parse and validate edited JSON
    let edited_json: Value =
        serde_json::from_str(&edited_content).context("Invalid JSON after editing")?;

    // If original JSON was compact, convert back to compact format
    let original_was_compact = to_string(&json)?.len() == json.to_string().len();
    let final_content = if original_was_compact {
        to_string(&edited_json)?
    } else {
        to_string_pretty(&edited_json)?
    };

    // Save result as .json.gz
    let output_file = File::create(&cli.file).context("Failed to create output file")?;
    let mut encoder = GzEncoder::new(output_file, Compression::default());
    encoder
        .write_all(final_content.as_bytes())
        .context("Failed to write compressed output")?;

    Ok(())
}
