pub mod cli;
pub mod entry;

use crate::entry::{Entry, FileType};
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
        let entry_name = entry.file_name().into_string().unwrap();
        let entry_meta = entry.metadata().unwrap();
        let hidden = is_hidden(&entry_name);

        if !all && hidden {
            continue;
        }

        let entry = Entry {
            name: entry.file_name().into_string().unwrap(),
            hidden,
            file_type: if entry_meta.is_dir() {
                FileType::Folder
            } else {
                FileType::File
            },
        };

        if let FileType::File = entry.file_type {
            file_list.push(entry);
        } else {
            folder_list.push(entry);
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
