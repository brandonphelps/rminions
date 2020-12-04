
pub mod asteroids;
pub mod collision;



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_game_init() {
	let game_state = asteroids::game_init();
	let game_input = asteroids::GameInput {
	    rotation: 0.5,
	    shoot: true,
	    thrusters: false,
	};

	// asteroids::game_update(&game_state, 0.1, &game_input, 
    }
}
