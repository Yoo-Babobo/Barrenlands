use std::{collections::HashMap, fs};

use bevy::ecs::system::Resource;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::VERSION;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ComponentType {
    Palette
}

pub type ComponentData = Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Component {
    id: String,
    r#type: ComponentType,
    data: ComponentData
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ModifyEntityType {
    Location,
    Inventory
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    SpawnEntity,
    ModifyEntity(ModifyEntityType),
    RemoveEntity
}

pub type ActionData = Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    id: String,
    r#type: ActionType,
    data: ActionData
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mod {
    #[serde(skip, default = "Uuid::new_v4")]
    pub uuid: Uuid,
    pub id: Option<String>,
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub components: Vec<Component>,
    #[serde(default)]
    pub actions: Vec<Action>
}

#[derive(Resource)]
pub struct Mods {
    pub mods: HashMap<Uuid, Mod>,
    pub errors: Vec<String>
}

impl Mods {
    fn import(&mut self) -> Result<Vec<String>, Vec<String>> {
        let mut errors = vec![];

        if let Ok(dir) = fs::read_dir("C:/Users/xande/Documents/Barrenlands/assets/mods") {
            for entry in dir.into_iter() {
                if entry.is_ok() {
                    let file = entry.unwrap();
                    let mut path = file.path();
                    
                    if path.is_dir() {
                        path.push("mod.json");

                        match super::read_json::<Mod>(path) { // https://github.com/3Hren/msgpack-rust
                            Ok(m) => {
                                self.mods.insert(m.uuid, m);
                            },
                            Err(error) => errors.push(error.to_string())
                        }

                        continue;
                    }
                }

                errors.push(String::from("Unable to load a mod."));
            }

            return Ok(errors);
        }
        
        errors.push(String::from("Unable to locate mods directory."));
        Err(errors)
    }

    fn names(&self) -> Vec<String> {
        self.mods.iter().map(|(_, m)| m.name.clone()).collect()
    }
}

impl Default for Mods {
    fn default() -> Self {
        let mut mods = Self {
            mods: HashMap::new(),
            errors: vec![]
        };

        match mods.import() {
            Ok(errors) => {
                // mods.mod_names = names;
                mods.errors = errors;
            },
            Err(errors) => mods.errors = errors
        };

        let m = Mod {
            uuid: Uuid::new_v4(),
            id: Some(String::from("barrenlands")),
            name: String::from("Barrenlands"),
            version: String::from(VERSION),
            description: String::from("The core mod of Barrenlands."),
            author: String::from("Barrenlands"),
            components: vec![

            ],
            actions: vec![

            ]
        };

        mods.mods.insert(m.uuid, m);

        mods
    }
}