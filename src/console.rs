use sdl2::surface::Surface;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::render::Canvas;
use crate::widget::DrawableWidget;
use std::path::PathBuf;
use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;
use crate::widget::Widget;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

/// Manages the state of input provided by the user as a collection of strings. 
/// provides some font handling and drawing to the screen.
pub struct Console<'ttf, 'a> {
    current_string: String,
    buffer: Vec<String>,

    surface: Option<Surface<'a>>,
    font: Font<'ttf, 'a>,

    // width of the console frame in pixels.
    p_widget: u32,

}

impl<'ttf, 'a> Console<'ttf, 'a> {
    pub fn new(font_path: PathBuf, ttf_c: &'ttf Sdl2TtfContext) -> Self {
        Self {
            current_string: String::new(),
            buffer: Vec::new(),
            surface: None,
            font: ttf_c.load_font(font_path, 128).unwrap(),
            p_widget: 300,
        }
    }
}

impl<'ttf, 'a> Widget for Console<'ttf, 'a> {
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
                    Keycode::KpEnter | Keycode::Return => {
                        self.current_string.clear();
                    },
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

impl<'ttf, 'a> DrawableWidget for Console<'ttf, 'a> {
    fn draw(&mut self, canvas: &mut Canvas<Window>, x: u32, y: u32) {
        let temp_s = self.get_current_string();
        if temp_s.len() != 0 {
            // important that surface is member variable of
            // class, can get segfaults on mac os x platform if not,
            // guessing that there is some lifetime item that is being
            // violated. 
            self.surface = Some(self.font.render(&self.get_current_string())
                .blended(Color::RGBA(255, 0, 0, 255))
                .map_err(|e| e.to_string()).unwrap());

            let texture_creator = canvas.texture_creator();
            let target_rect = sdl2::rect::Rect::new(0, 0, self.p_widget, 30);
            match self.surface {
                Some(ref s) => {
                    let s_texture = texture_creator
                        .create_texture_from_surface(&s).
                        map_err(|e| e.to_string()).unwrap();
                    canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));
                    canvas.copy(&s_texture, None, Some(target_rect)).unwrap();
                },
                None => (),
            }
        }
    }
}
