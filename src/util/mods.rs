use std::{collections::HashMap, fs};

use bevy::ecs::system::Resource;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum ComponentType {
    Palette
}

pub type ComponentData = Value;

#[derive(Serialize, Deserialize)]
pub struct Component {
    id: String,
    r#type: ComponentType,
    data: ComponentData
}

#[derive(Serialize, Deserialize)]
pub enum ModifyEntityType {
    Location,
    Inventory
}

#[derive(Serialize, Deserialize)]
pub enum ActionType {
    SpawnEntity,
    ModifyEntity(ModifyEntityType),
    RemoveEntity
}

pub type ActionData = Value;

#[derive(Serialize, Deserialize)]
pub struct Action {
    id: String,
    r#type: ActionType,
    data: ActionData
}

#[derive(Serialize, Deserialize)]
pub struct Mod {
    #[serde(skip, default = "Uuid::new_v4")]
    pub uuid: Uuid,
    pub id: Option<String>,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub components: HashMap<String, Component>,
    pub actions: HashMap<String, Action>
}

#[derive(Resource)]
pub struct Mods {
    pub mods: HashMap<Uuid, Mod>,
    pub mod_names: Vec<String>,
    pub errors: Vec<String>
}

impl Mods {
    fn import(&mut self) -> Result<(Vec<String>, Vec<String>), Vec<String>> {
        let mut errors = vec![];

        if let Ok(dir) = fs::read_dir("C:/Users/xande/Documents/Rust/Barrenlands/assets/mods") {
            let mut names = vec![];

            for entry in dir.into_iter() {
                if entry.is_ok() {
                    let file = entry.unwrap();
                    let mut path = file.path();

                    path.push("mod.json");
                    
                    if path.is_dir() {
                        if let Ok(m) = super::read_json::<Mod>(path) { // https://github.com/3Hren/msgpack-rust    
                            names.push(m.name.clone());
                            self.mods.insert(m.uuid, m);
                            continue;
                        }
                    }
                }

                errors.push(String::from("Unable to load a mod."));
            }

            return Ok((names, errors));
        }
        
        errors.push(String::from("Unable to locate mods directory."));
        Err(errors)
    }
}

impl Default for Mods {
    fn default() -> Self {
        let mut m = Self {
            mods: HashMap::new(),
            mod_names: vec![],
            errors: vec![],
        };

        match m.import() {
            Ok((names, errors)) => {
                m.mod_names = names;
                m.errors = errors;
            },
            Err(errors) => m.errors = errors
        };

        m
    }
}