mod collision;
mod entity_manager;
mod game_state;
mod utils;

use sdl2;
use sdl2::event::Event;
// use sdl2::EventPump;

use sdl2::keyboard::Keycode;
//use sdl2::render::{Canvas, Texture, TextureCreator};
//use sdl2::video::{Window, WindowContext};

use entity_manager::Entity;
use game_state::{Command, Position, UserCommand, world_to_display};
use utils::{generate_path, Path};

// todo: create gui implementation if a user wanted to play the game themselves.

fn pos(vec: f64, t: f64, x: f64) -> f64 {
    return vec * t + x;
}

fn Pos(vec: f32, t: f32, x: &Position) -> Position {
    let delta: f32 = vec * t;
    println!("Delta: {}", delta);
    let p = Position::new_with_offset(0, 0, delta, 0.0);
    return x.add(&p);
}

fn generate_pathing_program(path: &Path) -> Vec<Command> {
    let mut program = Vec::<Command>::new();

    let pos_offset_dist: f32 = 1.0;
    let speed = 1.0; // meters per second
    let tile_width = 16;
    for p in path.path_points.iter() {
	let mut current_pos = Position::new(p.0, p.1);
        program.push(Command::MoveP(current_pos));
    }

    return program;
}

fn main() -> () {
    println!("Hello World: Asteroids is not currently providing a gui layer :(");

    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Window", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .unwrap();
    canvas.clear();

    let mut current_state = game_state::game_load();
    let mut game_input = game_state::GameInput::default();

    // todo: determine these programatically.
    let iron_pos = (10, 5);
    let newly_spawned_entity_id = 3;

    let mut frame = 0;
    while frame < 60 {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    frame = 60;
                }
                _ => {}
            };
        }

        // set values get user input.

        // if frame == 1 {
        //     game_input.create_hive = true;
        // }
        if frame == 2 {
            game_input.create_unit = true;
        }

        // if frame == 4 || frame == 5 || frame == 6 {
        //     game_input.create_unit = true;
        // }

        // should be like get programable units.
        for e in current_state.get_units() {
            if e.0 == newly_spawned_entity_id {
                let path = generate_path((0, 1), iron_pos);

                let mut prog = generate_pathing_program(&path);
                let mut return_path = generate_pathing_program(&generate_path(iron_pos, (0, 1)));
                // entity 2 is iron mine
                prog.push(Command::Harvest(Entity(2)));
                prog.append(&mut return_path);
                // entity 1 is hive.
                prog.push(Command::Deposit(Entity(1)));
                game_input
                    .user_commands
                    .push(UserCommand::LoadProgram(*e, prog));
            }
        }

        current_state = game_state::game_update(current_state, 0.1, &game_input);

        game_state::game_sdl2_render(&current_state, &mut canvas);

        canvas.present();

        use std::{thread, time};

        let ten_millis = time::Duration::from_millis(1000);

        thread::sleep(ten_millis);

        println!("game state {}\n{}", frame, current_state.string());

        // clear out input to a defaulted state.
        game_input = game_state::GameInput::default();
        frame += 1;
    }

    // hold the app and wait for user to quit.
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
    }
}

fn pos_to_Pos(p: f32) -> Position {

    return Position::new(0, 0);
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
	let seconds_past = (time_point as f64);
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
