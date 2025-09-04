# File Mover

A powerful command-line tool written in Rust that automatically renames and moves files based on configurable patterns and rules. Perfect for organizing downloads, media files, documents, and more.

## Features

- 🚀 **Fast and Efficient**: Built with Rust for optimal performance
- 🔧 **Flexible Configuration**: JSON-based rule system for complex file organization
- 🎯 **Pattern Matching**: Match files by name patterns with customizable renaming rules
- 📁 **Automatic Directory Creation**: Creates destination directories as needed
- 🧪 **Test Mode**: Simulate operations without modifying files
- 🎨 **Colored Output**: User-friendly terminal output with emojis
- 🔤 **Sorted Processing**: Files processed in alphabetical order
- 🔄 **Cross-Device Support**: Handles moves across different filesystems

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) (1.70+ recommended)

### From Source

```bash
git clone https://github.com/Phr33d0m/file-mover.git
cd file-mover
cargo build --release
```

The compiled binary will be available at `target/release/file-mover`.

### System-wide Installation

```bash
cargo install --path .
```

## Usage

### Basic Usage

1. Create a `.mover.json` configuration file in your target directory
2. Run the tool:

```bash
# Normal mode (moves files)
file-mover

# Test mode (simulate without moving files)
file-mover --test
# or
file-mover -t
```

### Configuration

Create a `.mover.json` file in the directory you want to process:

```json
{
  "rules": [
    {
      "pattern": "movie_",
      "renames": [
        {
          "from": "movie_",
          "to": "Movie - "
        },
        {
          "from": "_",
          "to": " "
        }
      ],
      "destination": "/home/user/Movies"
    },
    {
      "pattern": "doc_",
      "renames": [
        {
          "from": "doc_",
          "to": "Document - "
        }
      ],
      "destination": "/home/user/Documents"
    }
  ]
}
```

### Configuration Format

Each rule consists of:

- `pattern`: String pattern to match in filenames
- `renames`: Optional array of rename operations (applied in order)
- `destination`: Target directory path

### Example Configurations

#### Media Organizer

```json
{
  "rules": [
    {
      "pattern": "movie_",
      "renames": [
        {"from": "movie_", "to": "Movie - "},
        {"from": "_", "to": " "}
      ],
      "destination": "/home/user/Videos/Movies"
    },
    {
      "pattern": "tv_",
      "renames": [
        {"from": "tv_", "to": "TV Show - "},
        {"from": "_", "to": " "}
      ],
      "destination": "/home/user/Videos/TV Shows"
    },
    {
      "pattern": "music_",
      "renames": [
        {"from": "music_", "to": ""}
      ],
      "destination": "/home/user/Music"
    }
  ]
}
```

#### Document Organizer

```json
{
  "rules": [
    {
      "pattern": "invoice_",
      "destination": "/home/user/Documents/Finances/Invoices"
    },
    {
      "pattern": "receipt_",
      "destination": "/home/user/Documents/Finances/Receipts"
    },
    {
      "pattern": "contract_",
      "destination": "/home/user/Documents/Legal/Contracts"
    },
    {
      "pattern": "report_",
      "destination": "/home/user/Documents/Work/Reports"
    }
  ]
}
```

#### Download Organizer

```json
{
  "rules": [
    {
      "pattern": ".zip",
      "destination": "/home/user/Downloads/Archives"
    },
    {
      "pattern": ".tar",
      "destination": "/home/user/Downloads/Archives"
    },
    {
      "pattern": ".pdf",
      "destination": "/home/user/Downloads/Documents"
    },
    {
      "pattern": ".jpg",
      "destination": "/home/user/Downloads/Images"
    },
    {
      "pattern": ".mp4",
      "destination": "/home/user/Downloads/Videos"
    }
  ]
}
```

## Command Line Options

```
Usage: file-mover [OPTIONS]

Options:
  -t, --test     Run in test mode (simulate actions without modifying files)
  -h, --help     Print help
  -V, --version  Print version
```

## How It Works

1. **File Collection**: Scans the current directory for all files (excluding `.mover.json` and directories)
2. **Sorting**: Files are sorted alphabetically by filename
3. **Pattern Matching**: Each file is checked against rules in order
4. **First Match Wins**: Processing stops after the first matching rule
5. **Renaming**: All rename operations are applied sequentially
6. **Moving**: Files are moved to the specified destination (creates directories if needed)

## Development

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Check for compilation errors
cargo check

# Run linter
cargo clippy

# Format code
cargo fmt
```

### Testing

```bash
# Run tests
cargo test

# Run in test mode
cargo run -- --test
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Changelog

### v0.1.0

- Initial release
- Pattern-based file matching
- Configurable renaming rules
- Test mode simulation
- Colored terminal output
- Sorted file processing
- Cross-device move support

## Support

If you encounter any issues or have questions, please:

1. Check the [Issues](https://github.com/Phr33d0m/file-mover/issues) page
2. Create a new issue if needed
3. Provide details about your configuration and the problem

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [clap](https://github.com/clap-rs/clap) for command-line parsing
- Uses [serde](https://github.com/serde-rs/serde) for JSON configuration
- Uses [colored](https://github.com/mackwic/colored) for terminal output
