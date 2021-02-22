use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};


fn circle_formula(x: u32, y: u32) -> u32 {
    return ((x * x) + (y * y)).into();
}

fn next_x_sq(x_sq_n: u32, y_n: u32) -> u32 {
    x_sq_n - 2 * y_n - 1
}


fn radius_error(x_n: i32, y_n: i32, r_n: i32) -> i32 {
    (x_n * x_n + y_n * y_n - r_n * r_n).abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdl2;
    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;

    #[test]
    fn circle_testing() {
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

	let mut points = Vec::new();
	let r = 50;
	let mut col = r;
	// why is this 90? if this was octects wouldn't it be 45? 
	let degrees: f64 = 0.90;
	let mut num_to_go = degrees.sin() * r as f64;
	println!("Num to go: {}", num_to_go);
	for row in 0..num_to_go.round() as i32 {
	    let x = radius_error(col - 1, row + 1, r);
	    let x_o = radius_error(col, row + 1, r);

	    // upper half right octant
	    points.push((col, row));
	    points.push((row, col));
	    points.push((-1 * col, -1 * row));
	    points.push((-1 * row, -1 * col));	    
	    points.push((-1 * row, col));
	    points.push((row, -1 * col));
	    points.push((-1 * col, row));
	    points.push((col, -1 * row));

	    // if 2 * (radius_error(col, row, r) + 1) + 1 > 0 {
	    // 	col = col - 1;
	    // }
	    if x < x_o {
		col = col - 1;
	    }
	}

	// how to do this in single line? 
	let mut shifted_points = Vec::new();
	for i in points.iter() {
	    shifted_points.push((r + i.0, r + i.1));
	}
	println!("Shifted");
	for i in shifted_points.iter() {
	    println!("{},{}", i.0, i.1);
	    assert!(i.0 >= 0);
	    assert!(i.1 >= 0);
	}


	canvas.set_draw_color(Color::RGB(0, 255, 0));

	let center_point = (10, 10);

	let texture_creator = canvas.texture_creator();
	let mut circle_texture = texture_creator.create_texture_target(None, ((r * 2)+1) as u32, ((r * 2)+1) as u32).unwrap();
	canvas.with_texture_canvas(&mut circle_texture,
				   |canvas_context| {
				       canvas_context.set_draw_color(Color::RGB(255, 0, 0));
				       
				       for point in shifted_points.iter() {
					   canvas_context.draw_point(Point::new(point.0, point.1)).unwrap();
				       }
				       
				   }
	);

	println!("copyingtexture");
	canvas.copy(&circle_texture, None, None).unwrap();
	canvas.present();

	// hold the app and wait for user to quit.
	'holding_loop: loop {
            for event in event_pump.poll_iter() {
		match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
			keycode: Some(Keycode::Escape),
			..
                    } => break 'holding_loop,
                    _ => {
			canvas.present();
		    }
		}
            }
	}
	assert_eq!(1, 2);
    }
}
