use sdl2::rect::Rect;

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

use crate::utils::spaced_internals;

/// Manages the state of input provided by the user as a collection of strings. 
/// provides some font handling and drawing to the screen.
pub struct Console<'ttf, 'a> {
    current_string: String,
    buffer: Vec<String>,

    surface: Option<Surface<'a>>,
    font: Font<'ttf, 'a>,

    // width of the console frame in pixels.
    p_widget: u32,
    console_width: u32,
    console_height: u32,

}

impl<'ttf, 'a> Console<'ttf, 'a> {
    pub fn new(font_path: PathBuf, ttf_c: &'ttf Sdl2TtfContext) -> Self {
        Self {
            current_string: String::new(),
            buffer: Vec::new(),
            surface: None,
            font: ttf_c.load_font(font_path, 128).unwrap(),
            p_widget: 30,
            console_width: 300,
            console_height: 400,
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
                keycode: Some(t), repeat: false, .. } => {
                match t {
                    Keycode::Space => {
                        self.current_string += " ";
                    },
                    Keycode::Backspace => {
                        self.current_string.pop();
                    }
                    Keycode::KpEnter | Keycode::Return => {
                        self.buffer.push(self.current_string.clone());
                        self.current_string.clear();
                    },
                    _=> (),
                };
                match t as i32 {
                    97..=122 => {
                        self.current_string += &format!("{}", t);
                    },                        
                    _ => { println!("hello"); },
                };
                println!("Current string: {}", self.current_string);
            },

            Event::KeyUp { keycode: Some(_t),
                           repeat: false, .. } => {
                
            },
            _ => (),
        }
    }
}

impl<'ttf, 'a> DrawableWidget for Console<'ttf, 'a> {
    fn draw(&mut self, canvas: &mut Canvas<Window>, _x: u32, _y: u32) {

        let background_rec = Rect::new(0, 0,
                                       self.console_width, self.console_height);

        canvas.set_draw_color(Color::RGB(34, 39, 46));

        canvas.draw_rect(background_rec).unwrap();



        let texture_creator = canvas.texture_creator();
        let mut console_texture = texture_creator.create_texture(None, sdl2::render::TextureAccess::Target, self.console_width, self.console_height).unwrap();


        



        canvas.with_texture_canvas(&mut console_texture,
                                   |user_context| {

                                       // draw the backbuffer.
                                       user_context.set_draw_color(Color::RGBA(0, 200, 0, 255));
                                       user_context.fill_rect(Rect::new(0, 0, self.console_width, self.console_height));

                                       let intervals = spaced_internals(30, self.buffer.len() as u32);
                                       
                                       for (index, i) in self.buffer.iter().enumerate() {
                                           println!("{}", i);
                                           let s = self.font.render(i).blended(Color::RGBA(255,
                                                                                           0, 0, 255)).map_err(|e| e.to_string()).unwrap();
                                           println!("{}: s: {}, {}", index, s.width(), s.height());
                                           let k = texture_creator.create_texture_from_surface(&s).unwrap();
                                           // let q = k.query();
                                           let target_widget = i.len() as u32 * self.p_widget;
                                           
                                           // println!("{}: Text info: {:#?}, {:#?}", index, q.width, q.height);
                                           let interva = intervals[index];
                                           let rec = Rect::new(0, interva.0 as i32,
                                                               target_widget,
                                                               30);
                                           user_context.copy(&k, None, Some(rec)).unwrap();
                                       }
                                   }).expect("Failed to draw console backbuffer");

        let temp_s = self.get_current_string();
        if temp_s.len() != 0 {
            // important that surface is member variable of
            // class, can get segfaults on mac os x platform if not,
            // guessing that there is some lifetime item that is being
            // violated. 



            let target_widget = self.current_string.len() as u32 * self.p_widget;
            let target_rect = sdl2::rect::Rect::new(0, 0, target_widget, 30);

            canvas.copy(&console_texture, None, Some(background_rec)).unwrap();

            self.surface = Some(self.font.render(&self.get_current_string())
                .blended(Color::RGBA(255, 0, 0, 255))
                .map_err(|e| e.to_string()).unwrap());

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
