use anyhow::{Ok, anyhow};
use std::str::FromStr;

#[derive(Debug)]
pub struct Elist {
    pub elem: usize,
    pub mat: usize,
    pub typ: usize,
    pub rel: usize,
    pub esy: usize,
    pub sec: usize,
    pub node_i: usize,
    pub node_j: usize,
    pub node_k: usize,
}

impl FromStr for Elist {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let elem = parts.next().ok_or(anyhow!("not enought data at `{s}`"))?;
        let mat = parts.next().ok_or(anyhow!("not enought data at `{s}`"))?;
        let typ = parts.next().ok_or(anyhow!("not enought data at `{s}`"))?;
        let rel = parts.next().ok_or(anyhow!("not enought data at `{s}`"))?;
        let esy = parts.next().ok_or(anyhow!("not enought data at `{s}`"))?;
        let sec = parts.next().ok_or(anyhow!("not enought data at `{s}`"))?;
        let node_i = parts.next().ok_or(anyhow!("not enought data at `{s}`"))?;
        let node_j = parts.next().ok_or(anyhow!("not enought data at `{s}`"))?;
        let node_k = parts.next().ok_or(anyhow!("not enought data at `{s}`"))?;

        Ok(Self {
            elem: elem.parse()?,
            mat: mat.parse()?,
            typ: typ.parse()?,
            rel: rel.parse()?,
            esy: esy.parse()?,
            sec: sec.parse()?,
            node_i: node_i.parse()?,
            node_j: node_j.parse()?,
            node_k: node_k.parse()?,
        })
    }
}
