mod entity_manager;
mod game_state;

// todo: create gui implementation if a user wanted to play the game themselves.

fn main() -> () {
    println!("Hello World: Asteroids is not currently providing a gui layer :(");

    let mut current_state = game_state::game_load();
    let mut game_input = game_state::GameInput::default();


    let mut frame = 0;
    while frame < 10 {
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

	for e in current_state.get_units() {
	    println!("Available entities: {}", e.0);
	}

        current_state = game_state::game_update(current_state, 0.1, &game_input);

        println!("game state {}\n{}", frame, current_state.string());

        // clear out input to a defaulted state.
        game_input = game_state::GameInput::default();
        frame += 1;
    }
}
