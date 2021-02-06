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

fn Pos(vec: f64, t: f64, x: &Position) -> Position {
    let delta = vec * t;
    todo!();
}

fn generate_pathing_program(path: &Path) -> Vec<Command> {
    let mut program = Vec::<Command>::new();

    let pos_offset_dist: f32 = 1.0;
    let tile_width = 30.0;
    for p in path.path_points.iter() {
	let mut k = 0.0;
	while k < tile_width {
            program.push(Command::MoveP(Position::new_with_offset(
                p.0,
                p.1,
                k as f32 * pos_offset_dist,
                k as f32 * pos_offset_dist,
            )));
	    k += pos_offset_dist;
        }
    }

    return program;
}

//fn main() -> () {
fn test() -> () {
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



fn main() -> () {
    let mut current_pos: f64 = 0.0 ;
    let mut currentPos = Position::new(0, 0);
    let speed = 0.5;
    for time_point in 0..10 {
	let seconds = (time_point as f64);
	
	

	println!("t: {}, Pos: {}", seconds, current_pos);
	let displ = world_to_display(&Position::new_with_offset(current_pos as u32, 0, current_pos as f32, 0.0), 1, 16, 16);
	println!("D: {} {}", displ.0, displ.1);
	current_pos = pos(speed, (time_point as f64) * 0.1, current_pos);
    }
}
