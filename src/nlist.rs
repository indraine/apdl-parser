use anyhow::{Ok, anyhow};
use std::str::FromStr;

#[derive(Debug)]
pub struct Nlist {
    pub node: usize,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub thxy: f32,
    pub thyz: f32,
    pub thzx: f32,
}

impl FromStr for Nlist {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let node = parts.next().ok_or(anyhow!("not enought data at `{s}`"))?;
        let x = parts.next().ok_or(anyhow!("not enought data at `{s}`"))?;
        let y = parts.next().ok_or(anyhow!("not enought data at `{s}`"))?;
        let z = parts.next().ok_or(anyhow!("not enought data at `{s}`"))?;
        let thxy = parts.next().ok_or(anyhow!("not enought data at `{s}`"))?;
        let thyz = parts.next().ok_or(anyhow!("not enought data at `{s}`"))?;
        let thzx = parts.next().ok_or(anyhow!("not enought data at `{s}`"))?;

        Ok(Self {
            node: node.parse()?,
            x: x.parse()?,
            y: y.parse()?,
            z: z.parse()?,
            thxy: thxy.parse()?,
            thyz: thyz.parse()?,
            thzx: thzx.parse()?,
        })
    }
}
