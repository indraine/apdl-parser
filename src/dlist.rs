use anyhow::{Ok, anyhow};
use std::str::FromStr;

#[derive(Debug)]
pub struct Dlist {
    pub node: usize,
    pub label: String,
    pub real: f32,
    pub imag: f32,
}

impl FromStr for Dlist {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let node = parts.next().ok_or(anyhow!("not enought data at `{s}`"))?;
        let label = parts.next().ok_or(anyhow!("not enought data at `{s}`"))?;
        let real = parts.next().ok_or(anyhow!("not enought data at `{s}`"))?;
        let imag = parts.next().ok_or(anyhow!("not enought data at `{s}`"))?;

        Ok(Self {
            node: node.parse()?,
            label: label.parse()?,
            real: real.parse()?,
            imag: imag.parse()?,
        })
    }
}
