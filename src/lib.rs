use std::{path::Path, str::FromStr};
mod dlist;
use anyhow::Context;

pub use self::dlist::Dlist;

mod nlist;
pub use self::nlist::Nlist;

mod elist;
pub use self::elist::Elist;

mod prnsol;
pub use self::prnsol::Prnsol;


pub fn clear_nlist(contents: &str) -> impl Iterator<Item = &str> {
    contents
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            trimmed.chars().next().and_then(|c| c.is_ascii_digit().then_some(trimmed))
        })
}


pub fn get_list<T: FromStr<Err = anyhow::Error>>(path: &Path) -> anyhow::Result<Vec<T>> {
    let binding = std::fs::read_to_string(path).with_context(|| format!("path `{}`", path.display()))?;
    let vals = clear_nlist(&binding);
    vals.map(FromStr::from_str).collect()
}
