use std::collections::HashMap;

use crate::entity_manager::{Entity, EntityManager};
use crate::utils::{manhat_distance};

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

impl Position {
    pub fn new(x: u32, y: u32) -> Position {
	Position { x: x, y: y}
    }
}

/// How much energy a specific entity contains.
#[derive(Default, Clone)]
pub struct EnergyLevel {
    value: u32,
}

/// Indicates the item that a individual can hold of something.
pub struct Container {}

#[derive(Debug, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone)]
pub enum Command {
    /// used for moving to the next point.
    MoveP(Position),
    MoveD(Direction),

    /// used for extracting resources from the provided position.
    Harvest(Position),
    /// Used for dropping the current holding items off
    Deposit(Position),
}

/// Collision component, tracks if the the entity should collide.
/// Collision only occurs if both entity have collection.
#[derive(Clone, Default)]
pub struct Collision {
    value: bool,
}

#[derive(Default)]
pub struct Hive {
    minerals: u32,
    gas: u32,
    iron: u32,
    copper: u32,
}

#[derive(Default, Clone)]
pub struct MineableNode {
    current_amount: u32,
    initial_amount: u32,
}

#[derive(Clone)]
pub struct ComponentManager<T> {
    components: Vec<T>,
    entities: Vec<Entity>,
    // maps the entities to their corresponding index (component)
    lookup: HashMap<Entity, usize>,
}

impl<T> ComponentManager<T>
where
    T: Default,
{
    fn new() -> ComponentManager<T> {
        ComponentManager {
            components: Vec::new(),
            // after moving Entity and Entity manager a type is required????
            entities: Vec::<Entity>::new(),
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
    fn get_mut(&mut self, entity: &Entity) -> Option<&mut T> {
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
            }
            None => {}
        };
    }
}

// details a single tile aspect,
// is the "flooring" that items can stand on.
// items can not be standing on two tiles at the same time.
#[derive(Clone, Debug)]
pub struct Tile {}

// holds a single frame of the game at a given point.
#[derive(Clone)]
pub struct GameState {
    entity_manager: EntityManager,
    positions: ComponentManager<Position>,
    collision: ComponentManager<Collision>,
    energy_levels: ComponentManager<EnergyLevel>,
    hive_entity: Option<Entity>,
    iron_mines: ComponentManager<MineableNode>,
    memory: ComponentManager<Memory>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            entity_manager: EntityManager::new(),
            positions: ComponentManager::<Position>::new(),
            energy_levels: ComponentManager::<EnergyLevel>::new(),
            hive_entity: None,
            collision: ComponentManager::<Collision>::new(),
            iron_mines: ComponentManager::<MineableNode>::new(),
            memory: ComponentManager::<Memory>::new(),
        }
    }

    pub fn create_hive(&mut self, x: u32, y: u32) {
        match self.hive_entity {
            None => {
                self.hive_entity = Some(self.entity_manager.create());
                let mut p = self
                    .positions
                    .create(&(self.hive_entity.expect("Failed to create entity")));
                p.x = x;
                p.y = y;
            }
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

    // return a list of available entities.
    pub fn get_units(&self) -> Vec<&Entity> {
        return self.entity_manager.entities.iter().collect();
    }

    pub fn get_mineable_nodes(&self) -> Vec<&Entity> {
	let mut result = Vec::new();
	for e in self.entity_manager.entities.iter() {
	    match self.iron_mines.get(&e) {
		Some(t) => result.push(e),
		None => (),
	    }
	}
	return result;
    }

    pub fn get_entity_pos(&self, entity: &Entity) -> Option<Position> {
	todo!("returns a position if corresponding entity has a position"); 
    }

    // testing / debug
    pub fn string(&self) -> String {
        let mut res = String::new();

        for entity in self.entity_manager.entities.iter() {
            res.push_str(&format!("Entity: {}\n", entity.0));
            match self.positions.get(&entity) {
                Some(t) => {
                    res.push_str(&format!("\t P: {}, {}\n", t.x, t.y));
                }
                None => {}
            };
            match self.energy_levels.get(&entity) {
                Some(t) => {
                    res.push_str(&format!("\t E: {}\n", t.value));
                }
                None => {}
            }
        }

        return res;
    }
}


#[derive(Clone)]
pub struct Memory {
    // current value of program counter
    // points to the "next" command to run, thus is updated after the command
    // runs succesfully.
    program_counter: u32,
    commands: Vec<Command>,
}

impl Default for Memory {
    fn default() -> Memory {
	println!("Memory default impl");
	Memory { program_counter: 0,
		 commands: Vec::<Command>::new(),
	}
    }
}

pub enum UserCommand {
    /// updates a specific entities memory with the following command.
    LoadCommand(Entity, Command),
    /// clears and sets an entire program to the corresponding entity.
    LoadProgram(Entity, Vec<Command>),
}

pub struct GameInput {
    // todo.
    // initial idea is game input is a order set of commands that are processed in order
    // invalid commands would thus return errors back and result in no further commands
    // being processed.
    pub create_unit: bool,
    pub create_hive: bool,
    pub user_commands: Vec<UserCommand>,
}

impl GameInput {
    pub fn default() -> GameInput {
        GameInput {
            create_unit: false,
            create_hive: false,
            user_commands: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn components() {
        let position_component_manager = ComponentManager::<Position>::new();
        let mut p = EntityManager::new();
        let new_e = p.create();
        assert_eq!(new_e.0, 1);

        assert_eq!(position_component_manager.contains(&new_e), false);
    }

    #[test]
    fn components_create() {
        let mut position_component_manager = ComponentManager::<Position>::new();
        let mut p = EntityManager::new();
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
        let mut p = EntityManager::new();
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
    fn manhat_dist() {
	assert_eq!(2, manhat_distance(0, 0, 1, 1));
    }
}

// todo better name
/// function provides basic impl for moving a unit around
fn unit_pathing(old_game_state: &GameState, new_game_state: &GameState) {}

pub fn game_init() -> GameState {
    return GameState::new();
}

// todo: allow for loading from file.
pub fn game_load() -> GameState {
    let mut new_game_state = GameState::new();

    // pre initizliaed level.

    new_game_state.create_hive(0, 0);

    let iron_e = new_game_state.entity_manager.create();

    {
        let mut p = new_game_state.iron_mines.create(&iron_e);
        p.current_amount = 900;
        p.initial_amount = 900;
    }
    {
        let mut p = new_game_state.positions.create(&iron_e);
        p.x = 10;
        p.y = 5;
    }

    // unit
    let new_entity = new_game_state.entity_manager.create();
    println!("First unit!: {}", &new_entity.0);
    let mut pos_component = new_game_state.positions.create(&new_entity);
    pos_component.x = 0;
    pos_component.y = 1;

    let mut memory = new_game_state.memory.create(&new_entity);
    memory
        .commands
        .push(Command::MoveP(Position { x: 0, y: 4 }));
    memory
        .commands
        .push(Command::MoveP(Position { x: 0, y: 5 }));
    memory
        .commands
        .push(Command::MoveP(Position { x: 0, y: 7 }));
    memory.program_counter = 0;
    return new_game_state;
}

/// @breif helper function for spawning units as the corresponding position
fn spawn_unit(game_state: &mut GameState, p: Position) {
    let new_entity = game_state.entity_manager.create();
    // todo: add collision detection to where the spawn point is located relative to the hive. 
    let mut pos_component = game_state.positions.create(&new_entity);
    *pos_component = p;
    game_state.memory.create(&new_entity);
    game_state.collision.create(&new_entity);
}


//
fn spawn_mineable<F>(game_state: &mut GameState, p: Position, node_type: F) {
    todo!("Spawn a node of type F");
}

// not all items that have positions are moveable, should there exist moveable componetns?
// currently not a good way to tie component X first entity to its other components. ./shrug
fn movement_system(entity: &Entity,
		   positions: &mut ComponentManager<Position>,
		   collisions: &mut ComponentManager<Collision>,
		   new_pos: Position
) {
    // todo: check if new pos is within bounds?

    let mut is_colliding = false;

    // collision movement system. 
    for e_collision in collisions.entities.iter() {
	match positions.get(&e_collision) {
	    Some(t) => {
		if t.x == new_pos.x && t.y == new_pos.y {
		    println!("A collision has occured at {}, {}", t.x, t.y);
		    is_colliding = true;
		    break;
		}
	    },
	    // this collision entity doesn't have a position.
	    None => (),
	}
    }

    if !is_colliding {
	// its okay to move to new_pos.
	let mut pos = positions.get_mut(&entity).expect(&(format!("an entity didn't have a position? entity id: {}", entity.0)));
	*pos = new_pos;
    }
}

// hive should be the only building that is non moveable.
// all other "buildings" are moveable units.
pub fn game_update(game_state: GameState, dt: f64, game_input: &GameInput) -> GameState {
    // this clone is cloning a &GameState and not a GameState?
    let mut new_game_state = game_state.clone();

    // Process player commands(input).

    // update each entity.

    // todo: game logic update stuff.
    if game_input.create_hive {
        if !new_game_state.has_hive() {
            new_game_state.create_hive(0, 0);
        }
    }

    // todo: entities to load commands must be near the hive. 
    for input_command in game_input.user_commands.iter() {
	match input_command {
	    UserCommand::LoadProgram(E, Prog) => {
		println!("Loading full program into {}", E.0);
		match new_game_state.positions.get(&E) {
		    Some(t) => {
			// 0, 0 is hive position
			// has a range of 5. 
			if manhat_distance(t.x, t.y, 0, 0) > 5 {
			    println!("unable to load commands into entity as its far away");
			    // todo: need some sort of log listing and reporting to the user. 
			}
			else
			{
			    match new_game_state.memory.get_mut(&E) {
				Some(mut t) => {
				    t.commands.clear();
				    let mut new_program = Prog.clone();
				    t.commands.append(&mut new_program);
				    // let new_program = Prog.clone();
				    // t.commands.append(&mut (Prog.clone()));
				},
				None => (),
			    }
			}
		    },
		    None => (),
		}
	    },
	    UserCommand::LoadCommand(E, C) => { 
		match new_game_state.positions.get(&E) {
		    Some(t) => {
			// 0, 0 is hive position
			// has a range of 5. 
			if manhat_distance(t.x, t.y, 0, 0) > 5 {
			    println!("unable to load commands into entity as its far away");
			    // todo: need some sort of log listing and reporting to the user. 
			}
			else
			{
			    match new_game_state.memory.get_mut(&E) {
				Some(mut t) => {
				    t.commands.push(C.clone());
				},
				None => (),
			    }
			}
		    },
		    None => (),
		};
	    }
	}
    }

    if game_input.create_unit {
        // if entity exists at pos_component (0, 1) then we can't spawn if that entity has collision.
        let mut is_colliding = false;
        for tmp_e in new_game_state.entity_manager.entities.iter() {
            match new_game_state.positions.get(&tmp_e) {
                Some(t) => {
                    if t.x == 0 && t.y == 1 {
                        is_colliding = true;
                    } else {
                        is_colliding = false;
                    }
                }
                None => is_colliding = false,
            }
        }

        if is_colliding {
            println!("Cannot create new entity at same position as another");
        } else {
	    spawn_unit(&mut new_game_state, Position { x: 0, y: 1});
        }
    }

    for e in new_game_state.entity_manager.entities.iter() {
        match new_game_state.memory.get_mut(&e) {
            Some(mut memory_comp) => {
                // process memory.
		println!("Process memory of unit: {}", e.0);
		if memory_comp.commands.len() > 0 { 
                    let current_command = &memory_comp.commands[memory_comp.program_counter as usize];
                    match current_command {
			Command::MoveP(P) => {
                            println!("Moving: {}, {}", P.x, P.y);
                            match new_game_state.positions.get_mut(&e) {
				Some(mut t) => {
				    movement_system(&e, &mut new_game_state.positions,
						    &mut new_game_state.collision,
						    P.clone());
				}
				None => {
                                    todo!("Moving an unpositional object")
				}
                            }
			}
			Command::MoveD(D) => {
                            println!("Moving: {:#?}", D)
			}
			_ => {
                            todo!("Unhandled command")
			}
                    }

                    memory_comp.program_counter += 1;
                    if (memory_comp.program_counter as usize) >= memory_comp.commands.len() {
			memory_comp.program_counter = 0;
                    }
		}
            }
            None => (),
        }
    }

    

    return new_game_state;
}
