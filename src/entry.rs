use crate::icons::ICONS;
use std::{fmt, fs::Metadata};

pub struct Entry {
    pub name: String,
    pub metadata: Metadata,
    pub hidden: bool,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arrow = if !self.hidden {
            ICONS.get("bullet").unwrap()
        } else {
            ICONS.get("hide_bullet").unwrap()
        };
        let icon = match self.metadata.is_dir() {
            false => ICONS.get("file").unwrap(),
            true => ICONS.get("folder").unwrap(),
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
