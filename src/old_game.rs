// old came stuff just moving it out of the way.
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
