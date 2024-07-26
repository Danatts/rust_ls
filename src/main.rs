use std::{env, fs};

fn main() {
    let cwd = env::current_dir().unwrap();
    let entries = fs::read_dir(cwd).unwrap();
    let mut files = Vec::new();
    let mut hidden_files = Vec::new();
    let mut folders = Vec::new();
    let mut hidden_folders = Vec::new();
    for entry in entries {
        let entry = entry.unwrap();
        let entry_name = entry.file_name().into_string().unwrap();
        let metadata = entry.metadata().unwrap();
        if metadata.is_file() {
            if is_hidden(&entry_name) {
                hidden_files.push(entry_name);
            } else {
                files.push(entry_name);
            }
        } else {
            if is_hidden(&entry_name) {
                hidden_folders.push(entry_name);
            } else {
                folders.push(entry_name);
            }
        }
    }

    files.sort();
    folders.sort();
    hidden_files.sort();
    hidden_folders.sort();

    for folder in folders {
        println!("\u{257C} \u{e5ff} {folder}");
    }

    for folder in hidden_folders {
        println!("\u{257C} \u{ea83} {folder}");
    }

    for file in files {
        println!("\u{257C} \u{f15b} {file}");
    }

    for file in hidden_files {
        println!("\u{257C} \u{ea7b} {file}");
    }
}

fn is_hidden(entry: &str) -> bool {
    let first_char = entry.chars().next().unwrap();
    first_char == '.'
}
