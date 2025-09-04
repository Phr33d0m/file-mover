mod config;
mod processor;
mod output;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "Renames and moves files based on configuration")]
struct Cli {
    /// Run in test mode (simulate actions without modifying files)
    #[arg(short = 't', long = "test")]
    test: bool,
}

fn main() {
    let cli = Cli::parse();
    let test_mode = cli.test;
    
    output::print_header();
    
    if test_mode {
        output::print_test_mode();
    }
    
    // Load configuration
    let config = match config::load_config() {
        Ok(config) => config,
        Err(err) => {
            output::print_error(&err);
            std::process::exit(1);
        }
    };
    
    // Process files
    match processor::process_files(&config, test_mode) {
        Ok(_) => output::print_success(),
        Err(err) => {
            output::print_error(&err);
            std::process::exit(1);
        }
    }
}
