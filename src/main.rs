extern crate anyhow;
extern crate serde;
extern crate serde_json;

use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct ClusterMap {
    name: String,
    group: i32,
}

fn get_cluster_info() -> Result<ClusterMap> {
    let config = std::fs::read_to_string("cluster.json")
        .context("failed to read config file")?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    Ok(map)
}

fn main() {
    let _ = match get_cluster_info() {
        Ok(cm) => println!("{:?}", cm),
        Err(err) => println!("{:?}", err),
    };
}