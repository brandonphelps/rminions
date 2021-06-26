use sdl2::rect::Rect;

use crate::widget::DrawableWidget;
use crate::widget::Widget;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use std::path::PathBuf;

use crate::utils::spaced_internals;

/// Manages the state of input provided by the user as a collection of strings.
/// provides some font handling and drawing to the screen.
pub struct Console<'ttf, 'a, 'callback> {
    current_string: String,
    buffer: Vec<String>,

    surface: Option<Surface<'a>>,
    font: Font<'ttf, 'a>,

    // width of a single character in pixels.
    p_widget: u32,
    // heigh to f asingle character in pixels.
    char_height: u16,

    // width of the console frame in pixels.
    console_width: u32,
    // height of the console frame in pixels.
    console_height: u32,

    // need some sort of callback hook for when event should occur. 
    /// callback function if defined 
    enter_callback: &'callback dyn Fn(String) -> (),
}

impl<'ttf, 'a, 'callback> Console<'ttf, 'a, 'callback> {
    pub fn new(font_path: PathBuf,
               ttf_c: &'ttf Sdl2TtfContext,
               enter_callback: &'callback dyn Fn(String) -> ()) -> Self {


        Self {
            current_string: String::new(),
            buffer: Vec::new(),
            surface: None,
            font: ttf_c.load_font(font_path, 128).unwrap(),
            p_widget: 30,
            char_height: 30,
            console_width: 300,
            console_height: 400,
            enter_callback: enter_callback,
        }
    }
}

impl<'ttf, 'a, 'callback> Widget for Console<'ttf, 'a, 'callback> {
    fn get_current_string(&self) -> String {
        self.current_string.clone()
    }

    fn update(&mut self, _: f32) {
        todo!()
    }

    fn update_event(&mut self, event: sdl2::event::Event) {
        let mut handled_string = None;
        match event {
            Event::KeyDown {
                keycode: Some(t),
                repeat: false,
                ..
            } => {
                match t {
                    Keycode::Space => {
                        self.current_string += " ";
                    }
                    Keycode::Backspace => {
                        self.current_string.pop();
                    }
                    Keycode::KpEnter | Keycode::Return => {
                        self.buffer.push(self.current_string.clone());
                        handled_string = Some(self.current_string.clone());
                        self.current_string.clear();
                    }
                    _ => (),
                };
                match t as i32 {
                    // a-z
                    97..=122 => {
                        self.current_string += &format!("{}", t);
                    }
                    _ => {
                        // println!("hello");
                    }
                };
            }

            Event::KeyUp {
                keycode: Some(_t),
                repeat: false,
                ..
            } => {}
            _ => (),
        };

        match handled_string {
            Some(t) => {
                (self.enter_callback)(t)
            },
            None => ()
        }
    }
}

impl<'ttf, 'a, 'callback> DrawableWidget for Console<'ttf, 'a, 'callback> {
    fn draw(&mut self, canvas: &mut Canvas<Window>, _x: u32, _y: u32) {
        let background_rec = Rect::new(0, 0, self.console_width, self.console_height);

        canvas.set_draw_color(Color::RGB(34, 39, 46));

        let texture_creator = canvas.texture_creator();
        let mut console_texture = texture_creator
            .create_texture(
                None,
                sdl2::render::TextureAccess::Target,
                self.console_width,
                self.console_height,
            )
            .unwrap();

        // performs background and buffer drawing.
        canvas
            .with_texture_canvas(&mut console_texture, |user_context| {
                // draw the backbuffer.
                user_context.set_draw_color(Color::RGBA(0, 200, 0, 255));
                user_context.fill_rect(Rect::new(0, 0, self.console_width, self.console_height));

                for (index, i) in self.buffer.iter().enumerate() {
                    let s = self
                        .font
                        .render(i)
                        .blended(Color::RGBA(255, 0, 0, 255))
                        .map_err(|e| e.to_string())
                        .unwrap();
                    let k = texture_creator.create_texture_from_surface(&s).unwrap();
                    // let q = k.query();
                    let target_widget = i.len() as u32 * self.p_widget;
                    let rec = Rect::new(
                        0,
                        self.char_height as i32 * index as i32,
                        target_widget,
                        self.char_height as u32,
                    );
                    user_context.copy(&k, None, Some(rec)).unwrap();
                }
            })
            .expect("Failed to draw console backbuffer");
        canvas
            .copy(&console_texture, None, Some(background_rec))
            .unwrap();

        // draw drawing of the current string / user provided input and
        // the prompt icon (todo) add prompt icon.
        if self.get_current_string().len() != 0 {
            // important that surface is member variable of
            // class, can get segfaults on mac os x platform if not,
            // guessing that there is some lifetime item that is being
            // violated.

            let target_widget = self.current_string.len() as u32 * self.p_widget;
            let target_rect = sdl2::rect::Rect::new(0, 0, target_widget, 30);

            self.surface = Some(
                self.font
                    .render(&self.get_current_string())
                    .blended(Color::RGBA(255, 0, 0, 255))
                    .map_err(|e| e.to_string())
                    .unwrap(),
            );

            match self.surface {
                Some(ref s) => {
                    let s_texture = texture_creator
                        .create_texture_from_surface(&s)
                        .map_err(|e| e.to_string())
                        .unwrap();
                    canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));
                    canvas.copy(&s_texture, None, Some(target_rect)).unwrap();
                }
                None => (),
            }
        }
    }
}
