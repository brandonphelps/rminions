
pub mod asteroids;
pub mod collision;



#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature="gui")]
    #[test]
    fn test_game_init() {

	use sdl2;
	use sdl2::pixels::Color;
	use sdl2::rect::{Point, Rect};

	use sdl2::keyboard::Keycode;
	use sdl2::render::{Canvas, Texture, TextureCreator};
	use sdl2::video::{Window, WindowContext};

	let game_state = asteroids::game_init();
	let game_input = asteroids::GameInput {
	    rotation: 0.5,
	    shoot: true,
	    thrusters: false,
	};

	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let window = video_subsystem
            .window("Window", 800, 600)
            .opengl()
            .build()
            .unwrap();

	fn find_sdl_gl_driver() -> Option<u32> {
	    for (index, item) in sdl2::render::drivers().enumerate() {
		if item.name == "opengl" {
		    return Some(index as u32);
		}
	    }
	    None
	}


	let mut canvas = window
            .into_canvas()
            .index(find_sdl_gl_driver().unwrap())
            .build()
            .unwrap();

	canvas.clear();


	asteroids::game_update(&game_state, 0.1, &game_input, &mut canvas);
    }
}
