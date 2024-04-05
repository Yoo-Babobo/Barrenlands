pub struct CustomNPC {

}

pub enum NPCType {
    CUSTOM(CustomNPC)
}

pub struct CustomMonster {

}

pub enum MonsterType {
    CUSTOM(CustomMonster)
}

pub enum EntityType {
    ENTITY,
    PLAYER,
    NPC(NPCType),
    Monster(MonsterType)
}

pub trait Entity {
    fn r#type() -> EntityType {
        EntityType::ENTITY
    }

    fn bundle() -> Self;
}

pub struct Player {

}

impl Entity for Player {
    fn r#type() -> EntityType {
        EntityType::PLAYER
    }
    
    fn bundle() -> Self {
        Self {

        }
    }
}