use colored::*;

pub fn print_header() {
    println!("{}", "File Mover".bright_green().bold());
    println!("{}", "==========".bright_green());
}

pub fn print_test_mode() {
    println!("{} {}", "🧪".yellow(), "Running in test mode - no files will be modified".yellow());
}

pub fn print_file_match(filename: &str, rule_index: usize) {
    println!("{} Match rule #{} for file: {}", "🔍".bright_cyan(), rule_index + 1, filename.bright_white());
}

pub fn print_rename(from: &str, to: &str, test_mode: bool) {
    let action = if test_mode { "Would rename" } else { "Renamed" };
    println!("  {} {} {} → {}", "✏️".bright_yellow(), action, from, to.bright_white());
}

pub fn print_prefix(from: &str, to: &str, test_mode: bool) {
    let action = if test_mode { "Would prefix" } else { "Prefixed" };
    println!("  {} {} {} → {}", "🔤".bright_yellow(), action, from, to.bright_white());
}

pub fn print_suffix(from: &str, to: &str, test_mode: bool) {
    let action = if test_mode { "Would suffix" } else { "Suffixed" };
    println!("  {} {} {} → {}", "🔡".bright_yellow(), action, from, to.bright_white());
}

pub fn print_move(_from: &str, to: &str, test_mode: bool) {
    let action = if test_mode { "Would move" } else { "Moved" };
    println!("  {} {} → {}", "📦".bright_magenta(), action, to.bright_white());
}

pub fn print_error(message: &str) {
    eprintln!("{} {}", "❌".bright_red(), message.bright_red());
}

pub fn print_summary(processed_count: usize, total_count: usize) {
    println!("{} Processed {} of {} files", "✅".bright_green(), processed_count, total_count);
}

pub fn print_success() {
    println!("{} {}", "✅".bright_green(), "Operation completed successfully".bright_green());
}

pub fn print_skip(filename: &str, test_mode: bool) {
    let action = if test_mode { "Would skip" } else { "Skipped" };
    println!("  {} {} {} - already exists at destination", "⏭️".bright_yellow(), action, filename);
}
