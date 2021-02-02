mod entity_manager;
mod game_state;

use game_state::{UserCommand, Command, Position};

// todo: create gui implementation if a user wanted to play the game themselves.

struct Path {
    // should use position? 
    path_points: Vec<(u32, u32)>
}

impl Path {
    pub fn new() -> Path {
	Path { path_points: Vec::new()}
    }

    pub fn add_return_path(&mut self) {
	let mut p = self.path_points.clone();
	let mut reverse = p.into_iter().rev().collect();
	self.path_points.append(&mut reverse);
    }
}

fn generate_pathing_program(path: &Path) -> Vec<Command> {
    let mut program = Vec::<Command>::new();
    
    for p in path.path_points.iter() {
	program.push(Command::MoveP(Position::new(p.0, p.1)));
    }

    return program;
}

/// naive direct path handling, no detection of things in the way in the slight est. 
fn generate_path(start_pos: (u32, u32), end_pos: (u32, u32)) -> Path {
    let mut r_path = Path::new();

    let mut current_pos = start_pos;
    while current_pos != end_pos {
	let mut next_x = current_pos.0;
	let mut next_y = current_pos.1;
	if current_pos.0 < end_pos.0 {
	     next_x = current_pos.0 + 1;
	} else if current_pos.0 > end_pos.0 {
	     next_x = current_pos.0 - 1;
	} else {
	    // movement is restricted to 1 tile at a time.
	    // thus no diagional movement on this non best-agon grided layout
	    // todo: change grid layout to best-agons
	    if current_pos.1 < end_pos.1 {
		next_y = current_pos.1 + 1;
	    } else if current_pos.1 > end_pos.1 {
		next_y = current_pos.1 - 1;
	    }
	}
	current_pos = (next_x, next_y);
	r_path.path_points.push(current_pos);
    }
    return r_path;
}

fn main() -> () {
    println!("Hello World: Asteroids is not currently providing a gui layer :(");

    let mut current_state = game_state::game_load();
    let mut game_input = game_state::GameInput::default();

    // todo: determine these programatically. 
    let iron_pos = (10, 5);
    let newly_spawned_entity_id = 4;

    let mut frame = 0;
    while frame < 30 {
        // set values get user input.

        if frame == 1 {
            game_input.create_hive = true;
        }
        if frame == 2 {
            game_input.create_hive = true;
        }

        if frame == 4 || frame == 5 || frame == 6 {
            game_input.create_unit = true;
        }

	// should be like get programable units. 
	for e in current_state.get_units() {
	    println!("Available entities: {}", e.0);
	    if e.0 == newly_spawned_entity_id {
		let mut path = generate_path((0, 1), iron_pos);
		path.add_return_path();
		let prog = generate_pathing_program(&path);
		game_input.user_commands.push(UserCommand::LoadProgram(*e, prog));
	    }
	}

        current_state = game_state::game_update(current_state, 0.1, &game_input);

        println!("game state {}\n{}", frame, current_state.string());

        // clear out input to a defaulted state.
        game_input = game_state::GameInput::default();
        frame += 1;
    }
}
