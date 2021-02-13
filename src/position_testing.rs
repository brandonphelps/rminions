use crate::game_state::{Position};

// sample file to play around in.

// todo: move this to another file. or delete when done. 
fn pos(vec: f64, t: f64, x: f64) -> f64 {
    return vec * t + x;
}

fn Pos(vec: f32, t: f32, x: &Position) -> Position {
    let delta: f32 = vec * t;
    println!("Delta: {}", delta);
    let p = Position::new_with_offset(0, 0, delta, 0.0);
    return x.add(&p);
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_positional() {
	assert_eq!(pos(1.0, 0.0, 0.0), 0.0);
	assert_eq!(pos(1.0, 0.0, 1.0), 1.0);
	assert_eq!(pos(1.0, 0.0, 5.0), 5.0);
	assert_eq!(pos(1.0, 0.0, 10.0), 10.0);

	assert_eq!(pos(1.0, 1.0, 10.0), 11.0);
	assert_eq!(pos(1.0, 2.0, 10.0), 12.0);
	assert_eq!(pos(1.0, 3.0, 10.0), 13.0);
	assert_eq!(pos(0.5, 1.0, 10.0), 10.5);
	assert_eq!(pos(0.5, 2.0, 10.0), 11.0);
	assert_eq!(pos(0.5, 3.0, 10.0), 11.5);

    }
	
}


fn test_main() -> () {
    let mut current_pos: f64 = 0.0 ;
    let mut currentPos = Position::new(0, 0);
    let speed = 1.0; // meters per second
    
    for time_point in 0..100 {
	let seconds_past = time_point as f64;
	let dt = 1.0; // in seconds
	println!("Time point: {} seconds", seconds_past);
	println!("Current Pos: {:#?}", currentPos);
	println!("Reg pos: {}", current_pos);
	let displ = world_to_display(&currentPos, 100);
	println!("D: {} {}", displ.0, displ.1);

	currentPos = Pos(speed as f32, dt, &currentPos);

	current_pos = pos(speed, dt as f64, current_pos);
	println!("Reg pos: {}", current_pos);
	println!("\n");
    }
}
