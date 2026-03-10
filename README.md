# apdl-parser

Русская версия: [`README.ru.md`](README.ru.md)

A small Rust library for parsing **ANSYS APDL** text listings (output of commands like `NLIST`, `ELIST`, `DLIST`, `PRNSOL`).

It is useful when you need to **extract mesh data / results from APDL text output** and feed them into your own pipeline (CSV/JSON export, postprocessing, quality checks, Python integration, etc.).

## Features

- Parses whitespace-separated data lines into strongly typed structs:
  - `Nlist`: node coordinates (`NLIST`)
  - `Elist`: element info and its nodes (`ELIST`)
  - `Dlist`: tabular data like `node label real imag` (`DLIST`)
  - `Prnsol`: nodal results example (`PRNSOL`, currently `NODE TEMP`)
- Includes a generic helper `get_list()` to read a file and parse it into `Vec<T>`.

## Typical use cases

- **Mesh export** from APDL into your own format (nodes/elements) for further geometry processing.
- **Model quality checks** (detect numbering gaps, validate coordinates/materials/element types).
- **Results postprocessing**: collect values from `PRNSOL`/`DLIST` and generate plots/reports.
- **CI / simulation pipelines**: compare results between model versions, regression tests.

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
apdl-parser = { git = "https://github.com/<you>/<repo>" }
```

After publishing to crates.io you can switch to a normal version requirement.

## Usage

### Example: read `NLIST` from a file

```rust
use std::path::Path;

use apdl_parser::{Nlist, get_list};

fn main() -> anyhow::Result<()> {
    let nodes: Vec<Nlist> = get_list(Path::new("NLIST.txt"))?;
    println!("nodes: {}", nodes.len());
    println!("{nodes:#?}");
    Ok(())
}
```

### Examples for repository files (`files/*.lis`)

This repository contains a `files/` folder with sample listings. You can run them via the included examples:

```bash
cargo run --example parse_nlist
cargo run --example parse_elist
cargo run --example parse_dlist
cargo run --example parse_prnsol
```

### Example: parse a single line

```rust
use apdl_parser::Elist;

fn main() -> anyhow::Result<()> {
    let e: Elist = "1  1  1  0  0  0  10  20  30".parse()?;
    println!("{e:?}");
    Ok(())
}
```

## Supported types

- `Nlist` — `src/nlist.rs`
- `Elist` — `src/elist.rs`
- `Dlist` — `src/dlist.rs`
- `Prnsol` — `src/prnsol.rs`

## Roadmap (ideas)

- More `PRNSOL` variants (not only `TEMP`).
- Stricter format validation and better error messages.
- CSV/JSON export (in a separate crate to keep core dependencies minimal).
- Optional `f64` support.

## License

Dual-licensed under **MIT OR Apache-2.0**. See `LICENSE-MIT` and `LICENSE-APACHE`.
