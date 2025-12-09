use std::fs;
use std::path::Path;
use crate::config::{Config};
use crate::output;

pub fn process_files(config: &Config, test_mode: bool, overwrite: bool) -> Result<(), String> {
    let current_dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(e) => return Err(format!("Failed to access current directory: {}", e)),
    };
    
    let entries = match fs::read_dir(&current_dir) {
        Ok(entries) => entries,
        Err(e) => return Err(format!("Failed to read directory: {}", e)),
    };
    
    let mut processed_count = 0;
    let total_count;
    
    // Collect all valid files and sort by filename
    let mut files = Vec::new();
    for entry_result in entries {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(e) => {
                output::print_error(&format!("Error accessing entry: {}", e));
                continue;
            }
        };
        
        // Skip directories and the config file itself
        if let Ok(file_type) = entry.file_type() {
            if !file_type.is_file() || entry.file_name() == ".mover.json" {
                continue;
            }
        }
        
        let path = entry.path();
        let filename = match path.file_name() {
            Some(name) => match name.to_str() {
                Some(s) => s.to_string(),
                None => continue, // Skip filenames that aren't valid UTF-8
            },
            None => continue,
        };
        
        files.push((path, filename));
    }
    
    // Sort files by filename
    files.sort_by(|a, b| a.1.cmp(&b.1));
    total_count = files.len();
    
    for (path, filename) in files {
        
        // Try to match the file against each rule
        for (rule_index, rule) in config.rules.iter().enumerate() {
            if matches_pattern(&filename, &rule.pattern) {
                processed_count += 1;
                output::print_file_match(&filename, rule_index);
                
                let mut new_filename = filename.clone();
                
                // Apply renames if any
                for rename in &rule.renames {
                    if new_filename.contains(&rename.from) {
                        let renamed = new_filename.replace(&rename.from, &rename.to);
                        output::print_rename(&new_filename, &renamed, test_mode);
                        new_filename = renamed;
                    }
                }
                
                // Prepare destination path
                let dest_dir = Path::new(&rule.destination);
                let dest_path = dest_dir.join(&new_filename);

                // Check if file already exists in destination
                if dest_path.exists() && !overwrite {
                    output::print_skip(&new_filename, test_mode);
                    break; // Skip to next file
                }

                output::print_move(&path.display().to_string(), &dest_path.display().to_string(), test_mode);

                // Actually move the file if not in test mode
                if !test_mode {
                    // Create destination directory if it doesn't exist
                    if !dest_dir.exists() {
                        if let Err(e) = fs::create_dir_all(dest_dir) {
                            output::print_error(&format!("Failed to create destination directory: {}", e));
                            continue;
                        }
                    }
                    
                    // Move the file
                    match fs::rename(&path, &dest_path) {
                        Ok(_) => {
                            // Successfully moved the file
                        },
                        Err(e) => {
                            // If error is cross-device link, try copy + delete approach
                            if e.kind() == std::io::ErrorKind::CrossesDevices || e.raw_os_error() == Some(18) {
                                match fs::copy(&path, &dest_path) {
                                    Ok(_) => {
                                        // Copy successful, now delete original
                                        if let Err(del_err) = fs::remove_file(&path) {
                                            output::print_error(&format!("Failed to delete original file after copying: {}", del_err));
                                        }
                                    },
                                    Err(copy_err) => {
                                        output::print_error(&format!("Failed to copy file: {}", copy_err));
                                    }
                                }
                            } else {
                                output::print_error(&format!("Failed to move file: {}", e));
                            }
                        }
                    }
                }
                
                break; // Stop after first matching rule
            }
        }
    }
    
    output::print_summary(processed_count, total_count);
    Ok(())
}

fn matches_pattern(filename: &str, pattern: &str) -> bool {
    filename.contains(pattern)
}
