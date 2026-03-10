use std::path::Path;

use apdl_parser::{Nlist, get_list};

fn main() -> anyhow::Result<()> {
    let path = Path::new("files/NLIST.lis");
    let nodes: Vec<Nlist> = get_list(path)?;

    println!("NLIST: {} records", nodes.len());
    for (i, n) in nodes.iter().take(5).enumerate() {
        println!("{i}: {n:?}");
    }

    Ok(())
}

