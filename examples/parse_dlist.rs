use std::path::Path;

use apdl_parser::{Dlist, get_list};

fn main() -> anyhow::Result<()> {
    let path = Path::new("files/DLIST.lis");
    let vals: Vec<Dlist> = get_list(path)?;

    println!("DLIST: {} records", vals.len());
    for (i, v) in vals.iter().take(5).enumerate() {
        println!("{i}: {v:?}");
    }

    Ok(())
}

