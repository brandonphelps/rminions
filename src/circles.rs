#![allow(dead_code)]

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

fn circle_formula(x: u32, y: u32) -> u32 {
    return ((x * x) + (y * y)).into();
}

fn next_x_sq(x_sq_n: u32, y_n: u32) -> u32 {
    x_sq_n - 2 * y_n - 1
}

fn radius_error(x_n: i32, y_n: i32, r_n: i32) -> i32 {
    (x_n * x_n + y_n * y_n - r_n * r_n).abs()
}

pub fn create_circle_texture<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    radius: i32,
) -> Result<Texture<'a>, String> {
    let shifted_points = generate_circle_points(radius);
    // +1 because the center is there.
    let mut circle_texture = texture_creator
        .create_texture_target(None, ((radius * 2) + 1) as u32, ((radius * 2) + 1) as u32)
        .unwrap();
    let text = canvas.with_texture_canvas(&mut circle_texture, |canvas_context| {
        canvas_context.set_draw_color(Color::RGB(255, 0, 0));
        for point in shifted_points.iter() {
            canvas_context
                .draw_point(Point::new(point.0, point.1))
                .unwrap();
        }
    });

    match text {
        Ok(_r) => (),
        Err(r) => {
            println!("create circle texture error!: {}", r);
        }
    };

    return Ok(circle_texture);
}

/// @brief generates a list of 2d points, where a line should be drawn to fill in a circle.
/// format (x1, y1, x2, y2)
fn generate_circle_lines(_radius: i32) -> Vec<(i32, i32, i32, i32)> {
    let points = Vec::new();

    return points;
}

fn generate_circle_points(radius: i32) -> Vec<(i32, i32)> {
    let mut points = Vec::new();
    let mut col = radius;
    // why is this 90? if this was octects wouldn't it be 45?
    let degrees: f64 = 0.90;
    let num_to_go = degrees.sin() * radius as f64;
    for row in 0..num_to_go.round() as i32 {
        let x = radius_error(col - 1, row + 1, radius);
        let x_o = radius_error(col, row + 1, radius);

        points.push((col, row));
        points.push((row, col));
        points.push((-1 * col, -1 * row));
        points.push((-1 * row, -1 * col));
        points.push((-1 * row, col));
        points.push((row, -1 * col));
        points.push((-1 * col, row));
        points.push((col, -1 * row));
        if x < x_o {
            col = col - 1;
        }
    }
    // how to do this in single line?
    let mut shifted_points = Vec::new();
    for i in points.iter() {
        shifted_points.push((radius + i.0, radius + i.1));
    }
    for i in shifted_points.iter() {
        assert!(i.0 >= 0);
        assert!(i.1 >= 0);
    }
    return shifted_points;
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

        let radius = 100;
        let texture_creator: TextureCreator<_> = canvas.texture_creator();
        let circle_texture = create_circle_texture(&mut canvas, &texture_creator, radius).unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 0));
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
