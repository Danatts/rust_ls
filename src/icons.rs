use std::{collections::HashMap, sync::LazyLock};

pub static ICONS: LazyLock<HashMap<&str, char>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("bullet", '\u{25c6}');
    m.insert("hide_bullet", '\u{25c7}');
    m.insert("file", '\u{f15b}');
    m.insert("folder", '\u{e5ff}');
    m
});
