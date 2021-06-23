use crate::widget::Widget;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

pub struct Console {
    current_string: String,
    buffer: Vec<String>
}

impl Console {
    pub fn new() -> Self {
        Self {
            current_string: String::new(),
            buffer: Vec::new()
        }
    }

}

impl Widget for Console {
    fn get_current_string(&self) -> String {
        self.current_string.clone()
    }

    fn update(&mut self, _: f32) {
        todo!()
    }

    fn update_event(&mut self, event: sdl2::event::Event) {
        match event {
            Event::KeyDown {
                keycode: Some(T), repeat: false, .. } => {
                match T {
                    Keycode::Space => {
                        self.current_string += " ";
                    },
                    Keycode::Backspace => {
                        self.current_string.pop();
                    }
                    _=> (),
                };
                match T as i32 {
                    97..=122 => {
                        self.current_string += &format!("{}", T);
                    },                        
                    _ => { println!("hello"); },
                };
                println!("Current string: {}", self.current_string);
            },
            Event::KeyUp { keycode: Some(T), repeat: false, .. } => {
                
            },
            _ => (),
        }
    }
}
