#![allow(dead_code)]

mod circles;
mod collision;
mod console;
mod entity_manager;
mod game_state;
mod utils;
mod vidlid_db;
mod widget;
mod lua_worker;

use std::process::Command as pCommand;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use rlua::Error;
use std::io::BufRead;

use rlua::Lua;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::HashMap;

use std::path::PathBuf;

use sdl2;
use sdl2::event::Event;
// use sdl2::EventPump;

use crate::console::Console;



use sdl2::keyboard::Keycode;
//use sdl2::render::{Canvas, Texture, TextureCreator};
//use sdl2::video::{Window, WindowContext};

use entity_manager::Entity;
use game_state::{Command, Position};
use utils::Path;

use std::sync::Arc;
use std::sync::Mutex;

// todo: create gui implementation if a user wanted to play the game themselves.

fn strip_empties(x: &mut HashMap<Entity, Entity>, value: &Entity) {
    let tmp = x.clone();
    let empties = tmp.iter().filter(|&(_, &v)| v.0 == value.0).map(|(k, _)| k);

    for k in empties {
        x.remove(k);
    }
}

#[allow(dead_code)]
fn generate_pathing_program(path: &Path) -> Vec<Command> {
    let mut program = Vec::<Command>::new();

    let _pos_offset_dist: f32 = 1.0;
    let _speed = 0.5; // meters per second
    let _tile_width = 16;

    for p in path.path_points.iter() {
        let current_pos = Position::new(p.0, p.1);
        program.push(Command::MoveP(current_pos));
    }

    return program;
}

fn program_harvest_unit(
    _entity: &Entity,
    target_entity: &Entity,
    target_pos: &Position,
) -> Vec<Command> {
    // should be like get programable units.
    let mut prog = Vec::new();
    prog.push(Command::MoveD(Position::new(
        target_pos.get_x(),
        target_pos.get_y(),
    )));
    prog.push(Command::Harvest(target_entity.clone()));
    prog.push(Command::MoveD(Position::new(0, 0)));

    // entity 1 is hive.
    prog.push(Command::Deposit(Entity(1)));
    return prog;
}


fn main() -> () {
    let (tx_one, rx_one) = mpsc::channel();
    let (tx_two, rx_two) = mpsc::channel();

    let arc_tx = Arc::new(Mutex::new(tx_two));
    let arc_rx = Arc::new(Mutex::new(rx_one));
    let arc_tx_one = Arc::new(Mutex::new(tx_one));
    let arc_rx_two = Arc::new(Mutex::new(rx_two));

    // spin up a lua worker that will process lua calls in the background so we
    // can keep doing drawing and other processing. 
    let lua_worker = lua_worker::LuaWorker::new(Arc::clone(&arc_rx),
                                                Arc::clone(&arc_tx));

    println!("Finished sending messages waiting for responses");
    thread::sleep(Duration::from_secs(1));

    let lua = Lua::new();

    // sdl video stuff.
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window_width = 800;
    let window_height = 600;

    let window = video_subsystem
        .window("Window", window_width, window_height)
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



    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("lazy.ttf");
    
    fn hello(lua: &Lua, s: String) -> Option<String> {
        let mut res = None;
        lua.context(|lua_ctx| {
            let _g = lua_ctx.globals();
            let p = lua_ctx.load(&s).eval::<rlua::MultiValue>();
            match p {
                Ok(r) => {
                    println!(
                        "{}",
                        r.iter()
                            .map(|value| format!("{:?}", value))
                            .collect::<Vec<_>>()
                            .join("\t")
                    );

                    println!("{:#?}", r);
                    for j in r.iter() {
                        match *j {
                            rlua::Value::Nil => {
                                res = Some("nil".into())
                            },
                            rlua::Value::Boolean(t) => {
                                res = Some(format!("{}", t))
                            },
                            rlua::Value::Integer(t) => {
                                res = Some(format!("{}", t))
                            },
                            _ => {
                                res = Some("to string undefined for".into());
                            }
                        }
                    }
                },
                Err(r) => {
                    res = match r {
                        Error::SyntaxError {
                            message,
                            incomplete_input,
                        } => Some(format!("Syntax error: {} {}", incomplete_input, message)),
                        Error::FromLuaConversionError { .. } => Some("nil".into()),
                        _ => Some(format!("unknown error: {}", r.to_string())),
                    }
                }
            };
        });
        res
    }

    let _output = if cfg!(target_os = "windows") {
        pCommand::new("cmd")
            .args(&["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        pCommand::new("open")
            .args(&["-a", "firefox", "https://www.google.com"])
            .output()
            .expect("failed to execute process")
    };

    let _temp = |value| hello(&lua, value);

    let mut widget_stack = Vec::<Box<dyn widget::DrawableWidget>>::new();
    let temp: Box<dyn widget::DrawableWidget> = Box::new(Console::new(p,
                                                                      &ttf_context,
                                                                      Arc::clone(&arc_rx_two),
                                                                      Arc::clone(&arc_tx_one),
    ));
    widget_stack.push(temp);

    // hold the app and wait for user to quit.
    'holding_loop: loop {
        canvas.clear();

        let background_rec = Rect::new(0, 0, window_width, window_height);
        canvas.set_draw_color(Color::RGB(124, 99, 151));

        canvas.fill_rect(background_rec).unwrap();

        // Draw for current top layer widget.
        match widget_stack.get_mut(0) {
            Some(ref mut widget) => {
                widget.draw(&mut canvas, 0, 0);
                canvas.present();
            }
            None => (),
        }

        match widget_stack.get_mut(0) {
            Some(ref mut widget) => {
                widget.update(1.0);
            },
            None => (),
        };

        // event processing which is sent directly to the top layer widget.
        for event in event_pump.poll_iter() {
            match widget_stack.get_mut(0) {
                Some(ref mut widget) => {
                    widget.update_event(event.clone());
                }
                None => (),
            }

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'holding_loop,
                Event::KeyUp {
                    timestamp: _,
                    window_id: _,
                    keycode,
                    scancode: _,
                    keymod: _,
                    repeat: _,
                } => {
                    match keycode {
                        Some(Keycode::Backquote) => {
                            // todo: display / push the console onto the stack.
                        }
                        _ => (),
                    };
                }
                Event::KeyDown {
                    timestamp: _,
                    window_id: _,
                    keycode,
                    scancode: _,
                    keymod: _,
                    repeat: _,
                } => match keycode {
                    Some(Keycode::Space) => {
                        canvas.clear();
                    }
                    _ => (),
                },

                Event::MultiGesture { .. } => {
                    println!("Got a multigesture");
                }
                Event::MouseButtonDown { .. } => (),
                _ => {
                    // println!("Got a random key press");
                }
            }
        }
    }


    if let Some(thread) = lua_worker.thread {
        println!("Waiting on lua thread to finish, as it hsould");
        arc_tx_one.lock().unwrap().send("l_exit".into()).unwrap();
        thread.join().unwrap();
    }
}
