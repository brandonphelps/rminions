#![allow(dead_code)]

use std::collections::HashMap;

use sdl2::render::{Canvas, TextureCreator};

// todo: remove unused imports.
use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

// Texture, TextureCreator
use sdl2::video::Window;

use crate::circles::create_circle_texture;
use crate::collision::Circle;
use crate::entity_manager::{Entity, EntityManager};
use crate::utils::manhat_distance;
use crate::utils::uclid_distance;

//
// visual width, visual height
// meter width, meter height

/// @brief a positional offset centered on the x/y pos
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PosOffset {
    // todo: change these to u32
    x: f32,
    y: f32,
}

// Components
/// Position component, tracks the x, y of an entity.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Position {
    x: u32,
    y: u32,
    offset: PosOffset,
}

/// @brief
/// position is in meters (world units)
/// p / m
/// 10 p / 1 m == 10 pixels foreach meeter thus |<--->| == 1 meter and 10 pixels
pub fn world_to_display(pos: &Position, pixels_per_meter: u16) -> (i32, i32) {
    let tile_pos_x = pos.x as f32 + (pos.offset.x / 100.0);
    let tile_pos_y = pos.y as f32 + (pos.offset.y / 100.0);
    let x_pos: i32 = (tile_pos_x * pixels_per_meter as f32) as i32;
    let y_pos: i32 = (tile_pos_y * pixels_per_meter as f32) as i32;
    return (x_pos, y_pos);
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x: x,
            y: y,
            offset: PosOffset { x: 0.0, y: 0.0 },
        }
    }

    pub fn get_x(&self) -> u32 {
        self.x
    }
    pub fn get_y(&self) -> u32 {
        self.y
    }

    pub fn distance(&self, other: &Self) -> f32 {
        let p_x1 = self.x as f32 * 100.0 + self.offset.x;
        let p_y1 = self.y as f32 * 100.0 + self.offset.y;
        let p_x2 = other.x as f32 * 100.0 + other.offset.x;
        let p_y2 = other.y as f32 * 100.0 + other.offset.y;

        return uclid_distance(p_x1, p_y1, p_x2, p_y2);
    }

    /// @brief position, but without default
    pub fn new_with_offset(x: u32, y: u32, x_off: f32, y_off: f32) -> Self {
        let mut p = Self::new(x, y);

        if x_off >= 100.0 {
            let x_carry_over = x_off / 100.0;
            p.x += x_carry_over as u32;
            p.offset.x = x_off % 100.0;
        } else if x_off < 0.0 {
            // subtraction

            let tmp_x_off = x_off.abs();

            if p.offset.x < tmp_x_off {
                if p.x < (tmp_x_off / 100.0) as u32 {
                    panic!("Can't carry as {} < {} ", p.x, tmp_x_off);
                }
                // todo: how to get rid of this whileloop?
                while p.offset.x < tmp_x_off {
                    p.offset.x += 100.0;
                    p.x -= 1;
                }
                p.offset.x -= tmp_x_off;
            }
        } else {
            p.offset.x = x_off;
        }

        if y_off >= 100.0 {
            let y_carry_over = y_off / 100.0;
            p.y += y_carry_over as u32;
        } else if y_off < 0.0 {
            // subtraction

            let tmp_y_off = y_off.abs();

            if p.offset.y < tmp_y_off {
                if p.y < (tmp_y_off / 100.0) as u32 {
                    panic!("Can't carry as {} < {} ", p.y, tmp_y_off);
                }
                // todo: how to get rid of this whileloop?
                while p.offset.y < tmp_y_off {
                    p.offset.y += 100.0;
                    p.y -= 1;
                }
                p.offset.y -= tmp_y_off;
            }
        } else {
            p.offset.y = y_off;
        }
        return p;
    }

    pub fn add(&self, other: &Self) -> Self {
        let x_off = other.offset.x + self.offset.x;
        let y_off = other.offset.y + self.offset.y;

        Position::new_with_offset(self.x + other.x, self.y + other.y, x_off, y_off)
    }

    // todo: add subtraction.
    #[allow(dead_code)]
    pub fn sub(&self, _other: Self) -> Self {
        todo!();
    }
}

/// How much energy a specific entity contains.
#[derive(Default, Clone)]
pub struct EnergyLevel {
    value: u32,
}

/// Indicates the item that a individual can hold of something.
/// Storage of solids.
#[derive(Default, Clone, Debug)]
pub struct SolidContainer {
    iron_count: u32,
    copper_count: u32,
}

#[derive(Debug, Clone)]
pub enum Direction {
    #[allow(dead_code)]
    North,
    #[allow(dead_code)]
    East,
    #[allow(dead_code)]
    South,
    #[allow(dead_code)]
    West,
}

#[derive(Debug, Clone)]
pub enum Command {
    /// used for moving to the next point.
    MoveP(Position),
    #[allow(dead_code)]
    MoveD(Position),

    /// used for extracting resources from the provided position.
    Harvest(Entity),
    /// Used for dropping the current holding items off
    Deposit(Entity), // just the inverse of harvest is needed?
}

/// Collision component, tracks if the the entity should collide.
/// Collision only occurs if both entity have collection.
#[derive(Clone, Default)]
pub struct Collision {
    value: bool,
    // todo: change type to generic list of shapes, such as rectangles /circles etc.
    bounding_box: Circle,
}

#[derive(Default, Clone)]
pub struct MineableNode {
    current_amount: u32,
    initial_amount: u32,
}

#[derive(Clone, Debug)]
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

    /// Create a component with the initial value as specified by init_v.
    #[allow(dead_code)]
    fn create_with(&mut self, _entity: &Entity, _init_v: &T) -> &mut T {
        todo!();
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
    solid_containers: ComponentManager<SolidContainer>,
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
            solid_containers: ComponentManager::<SolidContainer>::new(),
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

                let mut f = self
                    .solid_containers
                    .create(&(self.hive_entity.expect("Faile to build hive")));
                f.iron_count = 0;
                f.copper_count = 0;
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

    pub fn get_programable_units(&self) -> Vec<&Entity> {
        return self.memory.entities.iter().collect();
    }

    pub fn get_entity_position(&self, entity: &Entity) -> Position {
        return self.positions.get(entity).unwrap().clone();
    }

    pub fn get_mineable_nodes(&self) -> Vec<&Entity> {
        return self.solid_containers.entities.iter().collect();
    }

    pub fn get_mineable_count(&self, mineable_entity: &Entity) -> Option<u32> {
        match self.solid_containers.get(mineable_entity) {
            Some(t) => Some(t.iron_count),
            None => None,
        }
    }

    // testing / debug
    pub fn string(&self) -> String {
        let mut res = String::new();

        for entity in self.entity_manager.entities.iter() {
            res.push_str(&format!("Entity: {}\n", entity.0));
            match self.positions.get(&entity) {
                Some(t) => {
                    res.push_str(&format!(
                        "\t P: {}, {}\n",
                        (t.x as f32 + t.offset.x),
                        (t.y as f32 + t.offset.y)
                    ));
                }
                None => {}
            };
            match self.energy_levels.get(&entity) {
                Some(t) => {
                    res.push_str(&format!("\t E: {}\n", t.value));
                }
                None => {}
            }
            match self.solid_containers.get(&entity) {
                Some(t) => {
                    res.push_str(&format!("\t I: {}\n", t.iron_count));
                }
                None => {}
            }
        }

        return res;
    }
}

#[derive(Debug, Clone)]
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
        Memory {
            program_counter: 0,
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
        let mut p = new_game_state.solid_containers.create(&iron_e);
        p.iron_count = 2;
    }
    {
        let mut p = new_game_state.positions.create(&iron_e);
        p.x = 10;
        p.y = 5;
    }

    {
        let mut p = new_game_state.collision.create(&iron_e);
        p.value = true;
    }

    let iron_2_two = new_game_state.entity_manager.create();

    {
        let mut p = new_game_state.solid_containers.create(&iron_2_two);
        p.iron_count = 900;
    }
    {
        let mut p = new_game_state.positions.create(&iron_2_two);
        p.x = 5;
        p.y = 10;
    }

    {
        let mut p = new_game_state.collision.create(&iron_2_two);
        p.value = true;
    }

    // unit
    // let new_entity = new_game_state.entity_manager.create();
    // println!("First unit!: {}", &new_entity.0);
    // let mut pos_component = new_game_state.positions.create(&new_entity);
    // pos_component.x = 0;
    // pos_component.y = 1;

    // let mut memory = new_game_state.memory.create(&new_entity);
    // memory.commands.push(Command::MoveP(Position::new(0, 4)));
    // memory.commands.push(Command::MoveP(Position::new(0, 5)));
    // memory.commands.push(Command::MoveP(Position::new(0, 7)));
    // memory.program_counter = 0;
    return new_game_state;
}

/// @breif helper function for spawning units as the corresponding position
fn spawn_unit(game_state: &mut GameState, p: Position) {
    let new_entity = game_state.entity_manager.create();
    // todo: add collision detection to where the spawn point is located relative to the hive.
    let pos_component = game_state.positions.create(&new_entity);
    *pos_component = p;
    game_state.memory.create(&new_entity);
    game_state.collision.create(&new_entity);
    game_state.solid_containers.create(&new_entity);
}

// todo: harvest might be just switchable to "transfer from one entity to another"
// harvest entity is the entity that is being harvested.
// harvest type marks which item to pull out of the harvest entity
fn harvest_system(
    entity: &Entity,
    positions: &mut ComponentManager<Position>,
    solid_containers: &mut ComponentManager<SolidContainer>,
    harvest_entity: &Entity,
    harvest_type: &str,
) {
    println!("Harvest system");
    // todo: do the error handling.
    let entity_pos = positions.get(entity).unwrap();
    let harvest_pos = positions.get(harvest_entity).unwrap();

    if manhat_distance(entity_pos.x, entity_pos.y, harvest_pos.x, harvest_pos.y) > 2 {
        println!("Failed to harvest due to being to far away");
        return;
    }

    // amount checking.
    match solid_containers.get(harvest_entity) {
        Some(harvest_container) => {
            if harvest_type == "iron" {
                if harvest_container.iron_count <= 0 {
                    // harvest entity is out of resources.
                    println!("Failed to harvest since mine is empty");
                    return;
                }
            }
        }
        None => {
            // harvest entity doesn't have an associated container to pull from.
            println!("Nothing to harvest");
            return;
        }
    }

    {
        let mut harvest_c = solid_containers.get_mut(harvest_entity).unwrap();
        if harvest_type == "iron" {
            harvest_c.iron_count -= 1;
        } else {
            panic!("Unknown harvest type");
        }
    }

    {
        let mut entity_c = solid_containers.get_mut(entity).unwrap();
        if harvest_type == "iron" {
            entity_c.iron_count += 1;
        } else {
            panic!("Unknown harvest type");
        }
    }

    // can't do this due to barrow system.
    // // todo: error handling.
    // match solid_containers.get_mut(entity) {
    // 	Some(mut entity_container) => {
    // 	    match solid_containers.get_mut(harvest_entity) {
    // 		Some(mut harvest_container) => {
    // 		    // harvest_container.iron_count -= 1;
    // 		    // entity_container.iron_count += 1;
    // 		},
    // 		None => (),
    // 	    }
    // 	},
    // 	None => (),
    // };

    // // ensure entity_container isn't full.
    // // if entity_container.
}

// not all items that have positions are moveable, should there exist moveable componetns?
// currently not a good way to tie component X first entity to its other components. ./shrug
fn movement_system(
    entity: &Entity,
    positions: &mut ComponentManager<Position>,
    _collisions: &mut ComponentManager<Collision>,
    new_pos: Position,
) {
    // collision movement system.
    let is_colliding = false;
    // for e_collision in collisions.entities.iter() {
    //     // only do collision detection on non myself entities.
    //     // todo: some rust iterator thing for this?
    //     if e_collision != entity {
    //         match positions.get(&e_collision) {
    //             Some(t) => {
    //                 // need bounding box collision rather than point collision.
    //                 if t.x == new_pos.x && t.y == new_pos.y {
    //                     println!("A collision has occured at {}, {}", t.x, t.y);
    //                     is_colliding = true;
    //                     break;
    //                 }
    //             }
    //             // this collision entity doesn't have a position.
    //             None => (),
    //         }
    //     }
    // }

    if !is_colliding {
        // its okay to move to new_pos.
        let pos = positions
            .get_mut(&entity)
            .expect(&(format!("an entity didn't have a position? entity id: {}", entity.0)));

        // units can't move more than a distance of 1
        // todo: what happens if the dt becomes super large?
        // if this occurs then this distance restriction could be hit and the unit won't move as far
        // however there are a lot of other issues like collision detection and such.
        if pos.distance(&new_pos) > 100.0 {
            return;
        }

        *pos = new_pos;
    }
}

// hive should be the only building that is non moveable.
// all other "buildings" are moveable units.
pub fn game_update(game_state: GameState, dt: f32, game_input: &GameInput) -> GameState {
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
            UserCommand::LoadProgram(entity_p, prog) => {
                println!("Loading full program into {}", entity_p.0);
                match new_game_state.positions.get(&entity_p) {
                    Some(t) => {
                        // 0, 0 is hive position
                        // has a range of 5.
                        if manhat_distance(t.x, t.y, 0, 0) > 5 {
                            println!("unable to load commands into entity as its far away");
                            // todo: need some sort of log listing and reporting to the user.
                        } else {
                            match new_game_state.memory.get_mut(&entity_p) {
                                Some(t) => {
                                    t.commands.clear();
                                    let mut new_program = prog.clone();
                                    t.commands.append(&mut new_program);
                                    // let new_program = Prog.clone();
                                    // t.commands.append(&mut (Prog.clone()));
                                }
                                None => (),
                            }
                        }
                    }
                    None => (),
                }
            }
            UserCommand::LoadCommand(entity_c, command) => {
                match new_game_state.positions.get(&entity_c) {
                    Some(t) => {
                        // 0, 0 is hive position
                        // has a range of 5.
                        if manhat_distance(t.x, t.y, 0, 0) > 5 {
                            println!("unable to load commands into entity as its far away");
                            // todo: need some sort of log listing and reporting to the user.
                        } else {
                            match new_game_state.memory.get_mut(&entity_c) {
                                Some(t) => {
                                    t.commands.push(command.clone());
                                }
                                None => (),
                            }
                        }
                    }
                    None => (),
                };
            }
        }
    }

    if new_game_state.has_hive() {
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
                spawn_unit(&mut new_game_state, Position::new(0, 1));
            }
        }
    }

    for e in new_game_state.entity_manager.entities.iter() {
        match new_game_state.memory.get_mut(&e) {
            Some(mut memory_comp) => {
                // process memory.
                if memory_comp.commands.len() > 0 {
                    let current_command =
                        &memory_comp.commands[memory_comp.program_counter as usize];
                    let mut move_pc = true;
                    match current_command {
                        Command::MoveP(position) => {
                            if new_game_state.positions.get(&e).is_some() {
                                movement_system(
                                    &e,
                                    &mut new_game_state.positions,
                                    &mut new_game_state.collision,
                                    position.clone(),
                                );
                            }
                        }
                        Command::MoveD(destination) => {
                            // println!("Move D: {},{}", destination.get_x(), destination.get_y());
                            let new_x;
                            let new_y;
                            let mut new_offset_x;
                            let mut new_offset_y;
                            {
                                let tmp_p = new_game_state.positions.get(&e).unwrap();
                                // current position

                                let speed = 10.0; // meter per second
                                new_x = tmp_p.x;
                                new_y = tmp_p.y;
                                new_offset_x = tmp_p.offset.x;
                                new_offset_y = tmp_p.offset.y;

                                // 1 is the speed 1 meter per second.

                                let x_dist = uclid_distance(
                                    tmp_p.x as f32 * 100.0 + tmp_p.offset.x,
                                    0.0,
                                    destination.x as f32 * 100.0 + destination.offset.x,
                                    0.0,
                                );
                                let y_dist = uclid_distance(
                                    0.0,
                                    tmp_p.y as f32 * 100.0 + tmp_p.offset.y,
                                    0.0,
                                    destination.y as f32 * 100.0 + destination.offset.y,
                                );

                                if tmp_p.x >= destination.x && x_dist > 2.5 {
                                    new_offset_x -= speed * dt;
                                } else if tmp_p.x <= destination.x && x_dist > 2.5 {
                                    new_offset_x += speed * dt;
                                }

                                if tmp_p.y >= destination.y && y_dist > 2.5 {
                                    new_offset_y -= speed * dt;
                                } else if tmp_p.y <= destination.y && y_dist > 2.5 {
                                    new_offset_y += speed * dt;
                                }
                            }

                            let new_pos =
                                Position::new_with_offset(new_x, new_y, new_offset_x, new_offset_y);

                            movement_system(
                                &e,
                                &mut new_game_state.positions,
                                &mut new_game_state.collision,
                                new_pos.clone(),
                            );

                            if new_pos.distance(&destination) > 5.0 {
                                move_pc = false;
                            }
                        }
                        Command::Harvest(minable_entity) => {
                            if new_game_state.positions.get(&e).is_some() {
                                harvest_system(
                                    &e,
                                    &mut new_game_state.positions,
                                    &mut new_game_state.solid_containers,
                                    &minable_entity,
                                    "iron",
                                );
                            }
                        }
                        Command::Deposit(mineable_entity) => {
                            if new_game_state.positions.get(&mineable_entity).is_some() {
                                harvest_system(
                                    &mineable_entity,
                                    &mut new_game_state.positions,
                                    &mut new_game_state.solid_containers,
                                    &e,
                                    "iron",
                                );
                            }
                        }
                        #[allow(unreachable_patterns)]
                        _ => {
                            todo!("Unhandled command")
                        }
                    }

                    if move_pc {
                        memory_comp.program_counter += 1;
                        if (memory_comp.program_counter as usize) >= memory_comp.commands.len() {
                            memory_comp.program_counter = 0;
                        }
                    }
                }
            }
            None => (),
        }
    }

    return new_game_state;
}

// likely can be moved to another file.
// #[cfg(feature = "gui")]
pub fn game_sdl2_render(game_state: &GameState, canvas: &mut Canvas<Window>) -> () {
    canvas.set_draw_color(Color::RGB(0, 255, 0));

    // draw grid.
    // display aspect.
    let pixels_per_meter: u16 = 50;

    // todo: game state should have a world bounds.
    for x_pos in 0..20 {
        for y_pos in 0..20 {
            // fill rect operates in visible pixel space.
            // todo: have function for translate between pixel space -> world space and vise versa.
            let vis_tile_pos = world_to_display(&Position::new(x_pos, y_pos), pixels_per_meter);
            // let tile_width = Position::new(tile_width_meters, tile_width_meters);
            // let tile_draw = world_to_display(&tile_pos, pixels_per_meter);
            // let tile_draw_w = world_to_display(&tile_width, pixels_per_meter);

            let _p = canvas.fill_rect(Rect::new(
                vis_tile_pos.0,
                vis_tile_pos.1,
                // allows for a margin to be created if less than pixel_tile_width /
                (pixels_per_meter - 5) as u32,
                (pixels_per_meter - 5) as u32,
            ));
        }
    }

    canvas.set_draw_color(Color::RGB(255, 0, 0));

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    // draw units ontop of grid.
    for entity in game_state.entity_manager.entities.iter() {
        match game_state.positions.get(&entity) {
            Some(pos) => {
                // where to draw.
                let vis_pos = world_to_display(pos, pixels_per_meter);

                let circle_texture = create_circle_texture(canvas, &texture_creator, 10).unwrap();

                canvas
                    .copy(
                        &circle_texture,
                        None,
                        Rect::new(vis_pos.0 as i32, vis_pos.1 as i32, 11, 11),
                    )
                    .unwrap();
                // let _p = canvas.fill_rect(Rect::new(vis_pos.0 as i32, vis_pos.1 as i32, 10, 10));
                // how to determine what to draw?
            }
            None => (),
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
    fn spawn_unit() {
        let mut game_state = game_init();
        let mut game_input = GameInput::default();
        game_input.create_unit = true;

        game_state = game_update(game_state, 0.1, &game_input);

        // can't create entities if hive isn't a thing
        assert_eq!(game_state.entity_manager.count(), 0);
    }

    // todo: add in many of the failure cases.
    #[test]
    fn test_harvest_system() {
        let mut entity_manager = EntityManager::new();

        let unit = entity_manager.create();
        let mut pos_c = ComponentManager::<Position>::new();

        let mut solid_c = ComponentManager::<SolidContainer>::new();

        // unit
        {
            let mut unit_p = pos_c.create(&unit);
            unit_p.x = 0;
            unit_p.y = 0;
        }
        {
            let mut unit_s = solid_c.create(&unit);
            unit_s.iron_count = 0;
        }

        let iron_node = entity_manager.create();
        {
            let mut iron_p = pos_c.create(&iron_node);
            iron_p.x = 0;
            iron_p.y = 1;
        }
        {
            let mut iron_s = solid_c.create(&iron_node);
            iron_s.iron_count = 100;
        }

        harvest_system(&unit, &mut pos_c, &mut solid_c, &iron_node, "iron");

        let iron_s = solid_c.get(&iron_node).unwrap();
        assert_eq!(iron_s.iron_count, 99);

        let unit_s = solid_c.get(&unit).unwrap();
        assert_eq!(unit_s.iron_count, 1);
    }

    #[test]
    fn test_movement_system() {
        let mut entity_manager = EntityManager::new();

        let unit = entity_manager.create();
        let mut pos_c = ComponentManager::<Position>::new();
        let mut solid_c = ComponentManager::<Collision>::new();

        // unit
        {
            let mut unit_p = pos_c.create(&unit);
            unit_p.x = 0;
            unit_p.y = 0;
        }
        {
            let mut unit_s = solid_c.create(&unit);
            unit_s.value = true;
        }

        let iron_node = entity_manager.create();
        {
            let mut iron_p = pos_c.create(&iron_node);
            iron_p.x = 0;
            iron_p.y = 1;
        }
        {
            let mut iron_s = solid_c.create(&iron_node);
            iron_s.value = true;
        }

        movement_system(&unit, &mut pos_c, &mut solid_c, Position::new(0, 1));

        let iron_s = pos_c.get(&iron_node).unwrap();
        assert_eq!(*iron_s, Position::new(0, 1));

        let unit_s = pos_c.get(&unit).unwrap();
        assert_eq!(*unit_s, Position::new(0, 0));
    }

    // todo: parameterisze testing
    #[test]
    fn test_world_to_display() {
        let meter_to_pixels = 2;
        // assert_eq!(world_to_display(&Position::new(0, 0), meter_to_pixels, 16, 16), (0, 0));
        // assert_eq!(world_to_display(&Position::new(1, 0), meter_to_pixels, 16, 16), (32, 0));
        // assert_eq!(world_to_display(&Position::new(2, 0), meter_to_pixels, 16, 16), (64, 0));
        // assert_eq!(world_to_display(&Position::new(2, 1), meter_to_pixels, 16, 16), (64, 32));
        // assert_eq!(world_to_display(&Position::new_with_offset(2, 1, 10.0, 0.0),
        // 			    meter_to_pixels, 16, 16), (67, 32));
    }

    #[test]
    fn test_position_stuff() {
        let p1 = Position::new(30, 2);
        assert_eq!(
            p1,
            Position {
                x: 30,
                y: 2,
                offset: PosOffset { x: 0.0, y: 0.0 }
            }
        );

        let p2 = Position::new_with_offset(30, 2, 10.0, 2.0);
        assert_eq!(
            p2,
            Position {
                x: 30,
                y: 2,
                offset: PosOffset { x: 10.0, y: 2.0 }
            }
        );

        let p3 = Position::new_with_offset(30, 2, 200.0, 2.0);
        assert_eq!(
            p3,
            Position {
                x: 32,
                y: 2,
                offset: PosOffset { x: 0.0, y: 2.0 }
            }
        );

        let p4 = p2.add(&p3);
        assert_eq!(p4.x, 62);
        assert_eq!(p4.y, 4);

        assert_eq!(p2.add(&p3), p3.add(&p2));

        let p5 = p2.add(&Position::new_with_offset(30, 2, 90.0, 99.0));
        assert_eq!(p5.x, 61);
        // 2 + 2 + 1
        assert_eq!(p5.y, 5);
        assert_eq!(p5.offset.x, 0.0);
    }

    #[test]
    fn test_position_offsets_base() {
        let p1 = Position::new_with_offset(1, 1, 0.0, 0.0);
        let p2 = Position {
            x: 1,
            y: 1,
            offset: PosOffset { x: 0.0, y: 0.0 },
        };
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_position_offsets_negative_value() {
        let p1 = Position::new_with_offset(1, 1, 0.0, 0.0);
        let p3 = Position::new_with_offset(2, 1, -100.0, 0.0);
        assert_eq!(p1, p3);

        let p4 = Position::new_with_offset(1, 40, 0.0, -3900.0);

        assert_eq!(p1, p4);
    }
    #[test]
    fn test_position_always_store_positives() {
        let p5 = Position::new_with_offset(1, 1, -20.0, 0.0);
        assert_eq!(
            p5,
            Position {
                x: 0,
                y: 1,
                offset: PosOffset { x: 80.0, y: 0.0 }
            }
        );
    }
    #[test]
    fn test_position_with_remainder() {
        let p5 = Position::new_with_offset(1, 2, 30.0, 20.0);
        let p6 = Position::new_with_offset(1, 0, -32.0, 200.0);

        assert_eq!(
            p6,
            Position {
                x: 0,
                y: 2,
                offset: PosOffset { x: 68.0, y: 0.0 }
            }
        );

        let p5_p6 = Position::new_with_offset(1, 4, 98.0, 20.0);

        assert_eq!(p5.add(&p6), p5_p6);
    }

    #[test]
    fn test_position_distance() {
        let p1 = Position::new(1, 2);
        let p2 = Position::new(1, 2);

        assert_eq!(p1.distance(&p2), 0.0);

        let p3 = Position::new(2, 2);
        assert_eq!(p1.distance(&p3), 100.0);

        let p4 = Position::new_with_offset(1, 2, 50.0, 0.0);
        assert_eq!(p1.distance(&p4), 50.0);
    }
}
