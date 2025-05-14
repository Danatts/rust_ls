pub mod cli;
pub mod entry;

use crate::entry::Entry;
use std::{env, fs, path::PathBuf};

pub fn get_entries(path: Option<String>, all: bool) -> (Vec<Entry>, Vec<Entry>) {
    let path = match path {
        Some(path) => PathBuf::from(path),
        None => env::current_dir().unwrap(),
    };
    let entries = fs::read_dir(path).unwrap();

    let mut file_list: Vec<Entry> = Vec::new();
    let mut folder_list: Vec<Entry> = Vec::new();

    for entry in entries {
        let entry = entry.unwrap();

        let mut entry = Entry {
            name: entry.file_name().into_string().unwrap(),
            hidden: false,
            metadata: entry.metadata().unwrap(),
        };

        entry.hidden = is_hidden(&entry.name);
        if (all & entry.hidden) | !entry.hidden {
            if entry.metadata.is_file() {
                file_list.push(entry);
            } else {
                folder_list.push(entry);
            }
        }
    }

    file_list.sort();
    folder_list.sort();

    (file_list, folder_list)
}

fn is_hidden(entry: &str) -> bool {
    let first_char = entry.chars().next().unwrap();
    first_char == '.'
}
