use std::path::Path;

use apdl_parser::{Prnsol, get_list};

fn main() -> anyhow::Result<()> {
    let path = Path::new("files/PRNSOL.lis");
    let vals: Vec<Prnsol> = get_list(path)?;

    println!("PRNSOL: {} records", vals.len());
    for (i, v) in vals.iter().take(5).enumerate() {
        println!("{i}: {v:?}");
    }

    Ok(())
}

