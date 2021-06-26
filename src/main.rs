#![allow(dead_code)]

mod circles;
mod collision;
mod console;
mod entity_manager;
mod game_state;
mod utils;
mod widget;

use rlua::Lua;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
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

fn lua_entry() -> rlua::Result<()> {
    let lua = Lua::new();
    lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();
        globals.set("string_var", "hello")?;
        globals.set("int_var", 42)?;
        Ok(())
    })?;

    lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();
        println!("{}:", globals.get::<_, String>("string_var")?);
        println!("{}", globals.get::<_, u8>("int_var")?);

        assert_eq!(globals.get::<_, String>("string_var")?, "hello");
        Ok(())
    })?;

    lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();

        let p = lua_ctx
            .load(
                r#"
                     global = 'foo'..'bar'
                    "#,
            )
            .exec();
        match p {
            Ok(r) => Ok(r),
            Err(r) => {
                println!("{}", r);
                Err(r)
            }
        }?;

        println!("hello");

        assert_eq!(globals.get::<_, String>("global")?, "foobar");

        let table = lua_ctx.create_table()?;
        table.set(1, "one")?;
        table.set(2, "two")?;
        table.set(3, "three")?;
        assert_eq!(table.len()?, 3);

        globals.set("array_table", table)?;

        lua_ctx
            .load(
                r#"
for k, v in pairs(array_table) do
  print(k, v)
end
"#,
            )
            .exec()?;

        let print: rlua::Function = globals.get("print")?;
        print.call::<_, ()>("hello from rust")?;

        let check_equal = lua_ctx
            .create_function(|_, (list1, list2): (Vec<String>, Vec<String>)| Ok(list1 == list2))?;

        globals.set("check_equal", check_equal)?;

        assert_eq!(
            lua_ctx
                .load(r#"check_equal({"a", "b", "c"}, {"d", "e", "f"})"#)
                .eval::<bool>()?,
            false
        );

        print.call::<_, ()>("hello from rust")?;

        Ok(())
    })?;

    lua.context(|lua_ctx| {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let p = lua_ctx.load(&line.unwrap()).exec();

            match p {
                Ok(_r) => (),
                Err(r) => {
                    println!("{}", r)
                }
            }
        }
        Ok(())
    })?;

    Ok(())
}

fn main() -> () {
    // lua_entry();
    // return;

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

    let _current_state = game_state::game_load();
    let _game_input = game_state::GameInput::default();

    let frame_per_second_target = 60;
    let _milliseconds_per_frame = 1000.0 / frame_per_second_target as f32;

    // each programed unit gets a target
    // first entity is the actor, second is the "target"
    // let mut programed_units = Vec::<(Entity, Entity)>::new();
    let _programmed_units = HashMap::<Entity, Entity>::new();

    let _frame = 0;
    let _max_frame = 20000;

    let _current_target_entity: Option<Entity> = None;

    // 'running: while frame < max_frame {
    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit { .. }
    //             | Event::KeyDown {
    //                 keycode: Some(Keycode::Escape),
    //                 ..
    //             } => {
    //                 break 'running;
    //             }
    //             _ => {}
    //         };
    //     }

    //     // User code section for controls purposes.
    //     // set values get user input.

    //     if frame == 2 {
    //         game_input.create_unit = true;
    //     }

    //     // if the current target entity runes out of mins set it to none so we can find a new target.
    //     match current_target_entity {
    //         Some(current_ent) => match current_state.get_mineable_count(&current_ent) {
    //             None => {
    //                 current_target_entity = None;
    //             }
    //             Some(amount) => {
    //                 if amount == 0 {
    //                     strip_empties(&mut programmed_units, &current_ent);
    //                     current_target_entity = None;

    //                     println!("Mine is empty");
    //                 }
    //             }
    //         },
    //         None => {}
    //     }

    //     // no current target.
    //     if !current_target_entity.is_some() {
    //         for e in current_state.get_mineable_nodes() {
    //             // don't consider hive.
    //             if *e != Entity(1) {
    //                 match current_state.get_mineable_count(e) {
    //                     Some(amount) => {
    //                         if amount > 0 {
    //                             println!("Setting Entity to: {}", e.0);
    //                             current_target_entity = Some(*e);
    //                             break;
    //                         }
    //                     }
    //                     None => {}
    //                 }
    //             }
    //         }
    //     }

    //     if current_target_entity.is_some() {
    //         let mine_pos = current_state.get_entity_position(&current_target_entity.unwrap());

    //         for e in current_state.get_programable_units() {
    //             if !programmed_units.contains_key(e) {
    //                 println!(
    //                     "Setting actor {} target to: {}",
    //                     e.0,
    //                     current_target_entity.unwrap().0
    //                 );
    //                 let prog = program_harvest_unit(e, &current_target_entity.unwrap(), &mine_pos);

    //                 game_input
    //                     .user_commands
    //                     .push(UserCommand::LoadProgram(*e, prog));

    //                 programmed_units.insert(*e, current_target_entity.unwrap().clone());
    //             }
    //         }
    //     }

    //     // game input is finished perform server updating and such.
    //     let start = Instant::now();
    //     current_state = game_state::game_update(current_state, 0.1, &game_input);
    //     game_state::game_sdl2_render(&current_state, &mut canvas);
    //     // how expensive is this?
    //     canvas.present();
    //     let end = start.elapsed();
    //     if end.as_millis() as f32 > milliseconds_per_frame {
    //         println!("Missed timing window on frame: {}", frame);
    //     }

    //     // todo: get a consistant sleep time aiming for 60 fps.
    //     // (also recalcualte to be seconds per frame calc).
    //     let ten_millis = time::Duration::from_millis(10);

    //     thread::sleep(ten_millis);

    //     // println!("game state {}\n{}", frame, current_state.string());

    //     // clear out input to a defaulted state.
    //     game_input = game_state::GameInput::default();
    //     frame += 1;
    // }

    // need some sort of stateful item for what has focus.
    // need to then pass the event to w/e item has current focuse
    // then each item has a sort of "back out" option.

    // w/e widget has focus is the current "top" widget.
    // widget_stack.push(Box::new(Console::new()));

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("lazy.ttf");

    fn hello(s: String) {
        println!("hello: {}", s);
    }

    let mut widget_stack = Vec::<Box<dyn widget::DrawableWidget>>::new();
    let temp: Box<dyn widget::DrawableWidget> = Box::new(Console::new(p, &ttf_context, &hello));
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
                    timestamp,
                    window_id: _,
                    keycode,
                    scancode: _,
                    keymod,
                    repeat,
                } => {
                    println!(
                        "Up timestamp: {}, repeat: {}, keycode: {}, keymode: {}",
                        timestamp,
                        repeat,
                        keycode.unwrap(),
                        keymod
                    );

                    match keycode {
                        Some(Keycode::Backquote) => {
                            // todo: display / push the console onto the stack.
                        }
                        _ => (),
                    };
                }
                Event::KeyDown {
                    timestamp,
                    window_id: _,
                    keycode,
                    scancode: _,
                    keymod,
                    repeat,
                } => {
                    println!(
                        "Down timestamp: {}, repeat: {}, keycode: {}, keymode: {}",
                        timestamp,
                        repeat,
                        keycode.unwrap(),
                        keymod
                    );
                    match keycode {
                        Some(Keycode::Space) => {
                            canvas.clear();
                        }
                        _ => (),
                    }
                }

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
}
