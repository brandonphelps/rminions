mod entity_manager;
mod game_state;
mod utils;

use game_state::{UserCommand, Command, Position};
use utils::{Path, generate_path};

// todo: create gui implementation if a user wanted to play the game themselves.

fn generate_pathing_program(path: &Path) -> Vec<Command> {
    let mut program = Vec::<Command>::new();
    
    for p in path.path_points.iter() {
	program.push(Command::MoveP(Position::new(p.0, p.1)));
    }

    return program;
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
