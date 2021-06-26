use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Widget {
    fn update_event(&mut self, event: Event);
    fn update(&mut self, dt: f32);

    // tmp.
    fn get_current_string(&self) -> String;
}

pub trait DrawableWidget: Widget {
    fn draw(&mut self, canvas: &mut Canvas<Window>, x: u32, y: u32);
}
