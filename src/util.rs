use std::{error::Error, fs, io::BufReader, path::PathBuf};

use bevy::prelude::*;
use serde::Deserialize;

pub mod fps;
pub mod mods;
pub mod ui;

#[derive(Component)]
pub struct NoDespawn;

pub fn read_json<T: for<'a> Deserialize<'a>>(path: PathBuf) -> Result<T, Box<dyn Error>> {
    let file = fs::File::open(path)?;
    let json = serde_json::from_reader(BufReader::new(file))?;

    Ok(json)
}