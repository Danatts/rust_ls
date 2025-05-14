use std::{fmt, fs::Metadata};

const ARROW: &str = "\u{25BA}";
const HIDDEN_ARROW: &str = "\u{25BB}";
const FILE: &str = "\u{f15b}";
const HIDDEN_FILE: &str = "\u{ea7b}";
const FOLDER: &str = "\u{e5ff}";
const HIDDEN_FOLDER: &str = "\u{ea83}";

pub struct Entry {
    pub name: String,
    pub metadata: Metadata,
    pub hidden: bool,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arrow = if !self.hidden { ARROW } else { HIDDEN_ARROW };
        let icon = match self.metadata.is_dir() {
            false => {
                if self.hidden {
                    HIDDEN_FILE
                } else {
                    FILE
                }
            }
            true => {
                if self.hidden {
                    HIDDEN_FOLDER
                } else {
                    FOLDER
                }
            }
        };
        write!(f, "{arrow} {icon} {}", self.name)
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Entry {}
