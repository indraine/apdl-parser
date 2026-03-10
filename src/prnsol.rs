use anyhow::{Ok, anyhow};
use std::str::FromStr;

#[derive(Debug)]
pub struct Prnsol {
    pub node: usize,
    pub temp: f32,
}

impl FromStr for Prnsol {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let node = parts.next().ok_or(anyhow!("not enought data at `{s}`"))?;
        let temp = parts.next().ok_or(anyhow!("not enought data at `{s}`"))?;

        Ok(Self {
            node: node.parse()?,
            temp: temp.parse()?,
        })
    }
}
