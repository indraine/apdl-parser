use std::path::Path;

use apdl_parser::{Elist, get_list};

fn main() -> anyhow::Result<()> {
    let path = Path::new("files/ELIST.lis");
    let elems: Vec<Elist> = get_list(path)?;

    println!("ELIST: {} records", elems.len());
    for (i, e) in elems.iter().take(5).enumerate() {
        println!("{i}: {e:?}");
    }

    Ok(())
}

