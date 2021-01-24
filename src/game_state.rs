use std::collections::HashSet;

//
// visual width, visual height
// meter width, meter height

// Components
/// Position component, tracks the x, y of an entity. 
#[derive(Default)]
pub struct Position {
    x: u32,
    y: u32,
}

/// Collision component, tracks if the the entity should collide.
/// Collision only occurs if both entity have collection.
#[derive(Default)]
pub struct Collision {
    value: bool,
}


#[derive(PartialEq, Copy, Clone)]
pub struct Entity(u64);

pub struct EntityManager {
    entities: HashSet<Entity>,
    current_entity_id: Entity,
}

impl EntityManager {
    pub fn new() -> EntityManager {
	// purposely starting from 1. 
	EntityManager { current_entity_id: Entity(1),
			entities: HashSet::new() }
    }

    pub fn create(&self) -> Entity {
	let new_id = Entity(self.current_entity_id.0);
	return new_id;
    }
}

pub struct ComponentManager<T> {
    components: Vec<T>,
    entities: Vec<Entity>
}

impl<T> ComponentManager<T> where T: Default
{
    fn new() -> ComponentManager<T> {
	ComponentManager {
	    components: Vec::new(),
	    entities: Vec::new()
	}
    }

    /// Checks if an the associated entity contains the component of type T
    fn contains(&self, entity: &Entity) -> bool {
	match self.entities.iter().position(|x| x.0 == entity.0) {
	    Some(_) => true,
	    None => false,
	}
    }

    /// Creates a component of type T and associates it to the entity 
    fn create(&mut self, entity: &Entity) -> &mut T {
	if self.contains(entity) {
	    todo!();
	}
	
	// T must define a default value. 
	self.components.push(T::default());
	self.entities.push(*entity);

	let size = self.components.len()-1;
	return &mut self.components[size];
    }

    /// Returns the associated component for the entity provided.
    /// Returns None if the entity does not have such a component. 
    fn get(&self, entity: Entity) -> Option<&T> {
	todo!();
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
	assert_eq!(p.current_entity_id.0, 1);
    }

    #[test]
    fn components() {
	let position_component_manager = ComponentManager::<Position>::new();
	let p = EntityManager::new();
	let new_e = p.create();
	assert_eq!(new_e.0, 1);

	assert_eq!(position_component_manager.contains(&new_e), false);
    }

    #[test]
    fn components_create() {
	let mut position_component_manager = ComponentManager::<Position>::new();
	let p = EntityManager::new();
	let new_e = p.create();
	assert_eq!(new_e.0, 1);

	{
	    let pos = position_component_manager.create(&new_e);
	    pos.x = 10;
	    pos.y = 20;
	}
	assert_eq!(position_component_manager.contains(&new_e), true);
    }
}
