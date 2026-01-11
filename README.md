# eml2md

A fast, lightweight command-line tool to convert EML (email) files to Markdown format.

## Features

- **EML Parsing**: Parse standard EML files including headers (From, To, CC, Date, Subject) and body content
- **Multiple Formats**: Support for `simple` (plain text) and `html` output formats
- **Attachment Handling**: Embed inline images as base64 data URIs in the output
- **Clean Output**: Email metadata formatted as Markdown tables for easy reading

## Installation

### From Source

Requires [Rust](https://www.rust-lang.org/tools/install) 1.56+ (2021 edition).

```bash
git clone https://github.com/hydai/eml2md.git
cd eml2md
cargo build --release
```

The binary will be available at `target/release/eml2md`.

## Usage

```bash
eml2md -i <input.eml> -o <output.md> [-f <format>]
```

### Options

| Option | Description | Required | Default |
|--------|-------------|----------|---------|
| `-i, --input` | Input EML file path | Yes | - |
| `-o, --output` | Output Markdown file path | Yes | - |
| `-f, --format` | Output format (`simple` or `html`) | No | `simple` |

### Example

```bash
# Convert email to markdown
eml2md -i email.eml -o email.md

# Use HTML format
eml2md -i email.eml -o email.md -f html
```

### Output Format

The generated Markdown includes a metadata table and the email body:

```markdown
|||
|---|---|
|From|John Doe <john@example.com>|
|To|Jane Doe <jane@example.com>|
|CC||
|Date|2024-01-01 12:00:00|
|Subject|Meeting Notes|

Hello,

This is the email body content...
```

## Development

### Prerequisites

- Rust 1.56+ (2021 edition)
- Cargo

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release
```

### Code Quality

```bash
# Run all checks
make all

# Individual checks
make lint     # Format check (cargo fmt --all --check)
make clippy   # Linter (cargo clippy --all-targets --all-features)
make build    # Build project
```

### Testing

```bash
cargo test
```

## Project Structure

```
eml2md/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lib.rs           # Library exports
│   ├── eml.rs           # EML parsing (Email, Header, Body, Attachment)
│   ├── content_type.rs  # MIME type parsing
│   └── formatter/
│       ├── mod.rs       # Formatter trait and factory
│       └── simple.rs    # SimpleFormatter & SimpleHtmlFormatter
├── tests/
│   └── example/         # Example EML files for testing
├── Cargo.toml
└── Makefile
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| [clap](https://crates.io/crates/clap) | Command-line argument parsing |
| [mail-parser](https://crates.io/crates/mail-parser) | EML/MIME message parsing |
| [chrono](https://crates.io/crates/chrono) | Date/time handling |
| [base64](https://crates.io/crates/base64) | Base64 encoding for attachments |
| [anyhow](https://crates.io/crates/anyhow) | Error handling |
| [thiserror](https://crates.io/crates/thiserror) | Custom error types |

## License

Apache-2.0

## Author

[hydai](https://github.com/hydai)
