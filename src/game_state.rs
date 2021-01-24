
use std::collections::HashSet;

//
// visual width, visual height
// meter width, meter height

pub struct Position {
    x: u32,
    y: u32,
}

pub struct Collision {
    value: bool,
}

pub struct Entity(u64);

pub struct EntityManager {
    entities: HashSet<Entity>,
    current_entity_id: Entity,
}

impl EntityManager {
    pub fn new() -> EntityManager {
	EntityManager { current_entity_id: Entity(0),
			entities: HashSet::new() }
    }

    pub fn create(&self) -> Entity {
	let new_id = Entity(self.current_entity_id.0);
	return new_id;
    }
}

// details a single tile aspect,
// is the "flooring" that items can stand on.
// items can not be standing on two tiles at the same time. 
#[derive(Clone, Debug)]
pub struct Tile {
    
}

// holds a single frame of the game at a given point. 
#[derive(Clone, Debug)]
pub struct GameState {
    
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn entitys() {
	let p = Entity(20);
	assert_eq!(p.0, 20);
    }

    #[test]
    fn entity_manager() {
	let p = EntityManager::new();
	assert_eq!(p.current_entity_id.0, 0);
    }
}
