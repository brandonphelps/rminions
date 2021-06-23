
use sdl2::event::Event;

pub trait Widget {
    fn update_event(&mut self, event: Event);
    fn update(&mut self, dt: f32);

    // tmp.
    fn get_current_string(&self) -> String;
}

pub trait DrawableWidget {
    fn draw(&self, x: u32, y: u32);
}



