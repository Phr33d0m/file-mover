# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based file mover utility that renames and moves files based on configuration rules. The tool processes files in the current directory, applies pattern matching and renaming rules, then moves them to specified destinations.

## Common Commands

### Building and Running

- `cargo build` - Build the project
- `cargo run` - Run the file mover normally
- `cargo run -- -t` or `cargo run -- --test` - Run in test mode (simulate without modifying files)
- `cargo check` - Check for compilation errors
- `cargo clippy` - Run linter
- `cargo fmt` - Format code

### Testing

- `cargo test` - Run tests (if any are added)

## Architecture

### Core Modules

- `main.rs` - Entry point with CLI argument parsing using clap
- `config.rs` - Configuration loading and validation from `.mover.json`
- `processor.rs` - Main file processing logic including pattern matching, renaming, and moving
- `output.rs` - Colored terminal output for user feedback

### Configuration System

- Uses `.mover.json` in the current directory
- Config structure: `Config` contains `rules` vector of `FileRule` objects
- Each `FileRule` has:
  - `pattern` - String to match in filenames
  - `renames` - Optional vector of `RenameRule` objects for string replacements
  - `destination` - Target directory path

### File Processing Flow

1. Read all files in current directory (skipping directories and `.mover.json`)
2. For each file, check against rules in order
3. First matching rule wins (stops after first match)
4. Apply all rename transformations sequentially
5. Move file to destination directory (creates directories if needed)
6. Handles cross-device moves with copy+delete fallback

### Key Features

- Test mode simulation (`-t` flag)
- Cross-device move handling
- Colored terminal output with emojis
- UTF-8 filename validation
- Error handling with user-friendly messages

### Dependencies

- `serde`/`serde_json` - Configuration serialization
- `clap` - CLI argument parsing
- `colored` - Terminal output formatting

## Configuration File Format

See `.mover.example.json` for the expected format. Rules are processed in order, with pattern matching using simple string containment.
