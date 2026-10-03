# Simple Script Example

A basic Python script demonstrating common patterns for testing py2rs migration.

## What It Does

This script processes text files:
1. Reads configuration from JSON
2. Loads input text file
3. Processes text (removes extra whitespace, optional uppercase)
4. Counts word frequencies
5. Saves results to JSON output

## Files

- `main.py` — Main script with CLI argument parsing
- `config.json` — Sample configuration file
- `sample_input.txt` — Sample text for processing
- `README.md` — This file

## Usage

```bash
# Basic usage
python main.py sample_input.txt

# With options
python main.py sample_input.txt --config config.json --output results.json

# Convert to uppercase
python main.py sample_input.txt --uppercase
```

## Migration Notes

When migrating to Rust, expect these equivalents:

- `argparse` → `clap` (CLI argument parsing)
- `json` module → `serde_json` (JSON serialization)
- `pathlib.Path` → `std::path::PathBuf` (filesystem paths)
- `datetime` → `chrono` (date and time)
- `open()` / `read_text()` → `std::fs::read_to_string()` (file reading)
- `dict` → `HashMap<String, Value>` (key-value storage)

## Expected Output

After running with default settings, you should get `results.json`:

```json
{
  "input_file": "sample_input.txt",
  "processed_text": "Lorem ipsum dolor sit amet...",
  "word_count": 52,
  "unique_words": 41,
  "top_words": [
    ["python", 3],
    ["the", 2],
    ...
  ],
  "config": {
    "enabled": true,
    "max_results": 10
  },
  "timestamp": "2026-10-03T15:30:00"
}
```

## Testing Migration

Use this migration instruction in py2rs:

```markdown
Migrate this Python script to idiomatic Rust:
- Use `clap` with derive macros for CLI arguments
- Use `serde` and `serde_json` for JSON handling
- Use `std::fs` for file operations
- Use `std::path::PathBuf` for paths
- Use `chrono` for timestamps
- Use `HashMap<String, usize>` for word counting
- Preserve the same command-line interface
- Add proper error handling with `Result` and `?` operator
- Use `anyhow` or `thiserror` for error types
```
