use std::collections::HashSet;
use std::collections::HashMap;

//
// visual width, visual height
// meter width, meter height

// Components
/// Position component, tracks the x, y of an entity. 
#[derive(Default, Clone)]
pub struct Position {
    x: u32,
    y: u32,
}

/// How much energy a specific entity contains. 
#[derive(Default, Clone)]
pub struct EnergyLevel {
    value: u32,
}

/// Collision component, tracks if the the entity should collide.
/// Collision only occurs if both entity have collection.
#[derive(Default)]
pub struct Collision {
    value: bool,
}

#[derive(PartialEq, Copy, Clone, Eq, Hash)]
pub struct Entity(u64);

#[derive(Clone)]
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

    pub fn create(&mut self) -> Entity {
	let new_id = Entity(self.current_entity_id.0);
	self.entities.insert(new_id);
	self.current_entity_id = Entity(self.current_entity_id.0 + 1);
	return new_id;
    }
}

#[derive(Clone)]
pub struct ComponentManager<T> {
    components: Vec<T>,
    entities: Vec<Entity>,
    // maps the entities to their corresponding index (component)
    lookup: HashMap<Entity, usize>,
}

impl<T> ComponentManager<T> where T: Default
{
    fn new() -> ComponentManager<T> {
	ComponentManager {
	    components: Vec::new(),
	    entities: Vec::new(),
	    lookup: HashMap::new(), 
	}
    }

    /// Checks if an the associated entity contains the component of type T
    fn contains(&self, entity: &Entity) -> bool {
	match self.lookup.get(entity) { 
	    Some(_) => true,
	    None => false,
	}
    }

    /// Creates a component of type T and associates it to the entity 
    fn create(&mut self, entity: &Entity) -> &mut T {
	if self.contains(entity) {
	    todo!();
	}
	
	let entity_index = self.components.len();
	// T must define a default value. 
	self.components.push(T::default());

	self.entities.push(*entity);
	self.lookup.insert(*entity, entity_index);

	return &mut self.components[entity_index];
    }

    fn get(&self, entity: &Entity) -> Option<&T> {
	match self.lookup.get(entity) {
	    Some(&t) => Some(&self.components[t]),
	    None => None,
	}
    }

    /// Returns the associated component for the entity provided.
    /// Returns None if the entity does not have such a component. 
    fn mut_get(&mut self, entity: &Entity) -> Option<&mut T> {
	match self.lookup.get(entity) {
	    Some(&t) => Some(&mut self.components[t]),
	    None => None,
	}
    }

    /// removes the entity and its corresponding component. 
    fn remove(&mut self, entity: &Entity) {
	match self.lookup.get(entity) {
	    Some(&entity_index) => {
		self.components.swap_remove(entity_index);
		self.lookup.remove(entity);
	    },
	    None => {}, 
	};
    }
}
    

// details a single tile aspect,
// is the "flooring" that items can stand on.
// items can not be standing on two tiles at the same time. 
#[derive(Clone, Debug)]
pub struct Tile {
    
}

// holds a single frame of the game at a given point. 
#[derive(Clone)]
pub struct GameState {
    entity_manager: EntityManager,
    positions: ComponentManager<Position>,
    energy_levels: ComponentManager<EnergyLevel>,

    hive_entity: Option<Entity>,
}

impl GameState {
    pub fn new() -> GameState {
	GameState {
	    entity_manager: EntityManager::new(),
	    positions: ComponentManager::<Position>::new(),
	    energy_levels: ComponentManager::<EnergyLevel>::new(),
	    hive_entity: None,
	}
    }

    pub fn create_hive(&mut self, x: u32, y: u32) {
	match self.hive_entity {
	    None => {
		self.hive_entity = Some(self.entity_manager.create());
		let mut p = self.positions.create(&(self.hive_entity.expect("Failed to create entity")));
		p.x = x;
		p.y = y;
	    },
	    _ => (),
	};
	return ();
    }

    pub fn has_hive(&self) -> bool {
	match self.hive_entity {
	    Some(_) => true,
	    None => false,
	}
    }

    // testing / debug
    pub fn string(&self) -> String {
	let mut res = String::new();

	for entity in self.entity_manager.entities.iter() {
	    res.push_str(&format!("Entity: {}\n", entity.0));
	    match self.positions.get(&entity) {
		Some(t) => {
		    res.push_str(&format!("\t P: {}, {}\n", t.x, t.y));
		},
		None => {},
	    };
	    match self.energy_levels.get(&entity) {
		Some(t) => {
		    res.push_str(&format!("\t E: {}\n", t.value));
		},
		None => {},
	    }
	}

	return res;
    }
}

pub struct GameInput {
    // todo.
    // initial idea is game input is a order set of commands that are processed in order
    // invalid commands would thus return errors back and result in no further commands
    // being processed.
    pub create_unit: bool,
    pub create_hive: bool,
    
}

impl GameInput {
    pub fn default() -> GameInput {
	GameInput {
	     create_unit: false,
	     create_hive: false,
	}
    }
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

	match position_component_manager.lookup.get(&new_e) {
	    Some(&t) => assert_eq!(t, 0 as usize),
	    None => assert_eq!(true, false),
	};
	
	assert_eq!(position_component_manager.contains(&new_e), true);


	let pos = match position_component_manager.get(&new_e) {
	    Some(t) => t,
	    _ => panic!("Failed to get item"),
	};
	assert_eq!(pos.x, 10);
	assert_eq!(pos.y, 20);
    }

    #[test]
    fn component_remove() {
	let mut position_component_manager = ComponentManager::<Position>::new();
	let p = EntityManager::new();
	let new_e = p.create();

	{
	    let pos = position_component_manager.create(&new_e);
	    pos.x = 10;
	    pos.y = 20;
	}

	position_component_manager.remove(&new_e);
	assert_eq!(position_component_manager.contains(&new_e), false);
	assert_eq!(position_component_manager.components.len(), 0);
    }

    #[test]
    fn entity_create() {
	let p = EntityManager::new();
	let new_e = p.create();
	let new_f = p.create();
	assert_ne!(new_e, new_f);
    }
}

pub fn game_init() -> GameState {
    return GameState::new();
}

// hive should be the only building that is non moveable.
// all other "buildings" are moveable units. 

pub fn game_update(game_state: GameState, dt: f64, game_input: &GameInput) -> GameState {
    // this clone is cloning a &GameState and not a GameState?
    let mut new_game_state = game_state.clone();

    // todo: game logic update stuff. 
    if game_input.create_hive {
	if !new_game_state.has_hive() { 
	    new_game_state.create_hive(0, 0);
	}
    }

    if game_input.create_unit {
	let new_entity = new_game_state.entity_manager.create();
	let mut pos_component = new_game_state.positions.create(&new_entity);
	pos_component.x = 0;
	pos_component.y = 1;
    }

    return new_game_state;
}
