use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use sdl2::rect::Rect;
use std::collections::HashMap;

use sdl2::keyboard::Mod;

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

/// manages the state of keyboard input allowing for checking if
/// a key is still pressed down while another key event occurs.
/// all values are initially false.
pub struct KeyboardState {
    pressed_down_keys: HashMap<Keycode, bool>,
}

impl KeyboardState {
    pub fn new() -> Self {
        Self {
            pressed_down_keys: HashMap::new(),
        }
    }

    /// only accepts KeyUp and KeyDown events, all other events
    /// are ignored and do not do anything
    pub fn update(&mut self, event: Event) {
        // key, is_down, repeat
        let info = match event {
            Event::KeyUp {
                keycode, repeat, ..
            } => (keycode.unwrap(), false, repeat),
            Event::KeyDown {
                keycode, repeat, ..
            } => (keycode.unwrap(), true, repeat),
            _ => {
                return;
            }
        };

        self.pressed_down_keys.insert(info.0, info.1);
    }

    pub fn is_down(&self, key: Keycode) -> bool {
        match self.pressed_down_keys.get(&key) {
            Some(k) => *k,
            None => false,
        }
    }
}

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

    receiver: Arc<Mutex<mpsc::Receiver<String>>>,
    sender: Arc<Mutex<mpsc::Sender<String>>>,
}

impl<'ttf, 'a, 'callback> Console<'ttf, 'a, 'callback> {
    pub fn new(
        font_path: PathBuf,
        ttf_c: &'ttf Sdl2TtfContext,
        receiver: Arc<Mutex<mpsc::Receiver<String>>>,
        sender: Arc<Mutex<mpsc::Sender<String>>>
    ) -> Self {
        Self {
            current_string: String::new(),
            buffer: Vec::new(),
            surface: None,
            font: ttf_c.load_font(font_path, 128).unwrap(),
            p_widget: 20,
            char_height: 20,
            console_width: 700,
            console_height: 600,
            receiver,
            sender,
        }
    }

    /// inserts an entry into the back buffer for rendering.
    pub fn insert_text(&mut self, _value: String) {
        todo!()
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
        match event {
            Event::KeyDown {
                keycode: Some(t),
                repeat: false,
                ..
            } => {
                // character input.
                let charac = get_character_from_event(&event);
                match charac {
                    Some(c) => self.current_string.push(c),
                    None => (),
                };

                // control handling.
                match t {
                    Keycode::Backspace => {
                        self.current_string.pop();
                    }
                    Keycode::KpEnter | Keycode::Return => {
                        self.buffer.push(self.current_string.clone());
                        {
                            self.sender.lock().unwrap().send(self.current_string.clone()).unwrap();
                        }
                        self.current_string.clear();
                    }
                    _ => (),
                };
            }

            Event::KeyUp {
                keycode: Some(_t),
                repeat: false,
                ..
            } => {}
            _ => (),
        };

        {
            match self.receiver.lock().unwrap().try_recv() {
                Ok(r) => {
                    println!("Got a message back: {}", r);
                    self.buffer.push(r);
                },
                Err(r) => {
                    println!("Got recieve error, maybe jus tokay?: {:#?}", r);
                },
            }
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
                user_context
                    .fill_rect(Rect::new(0, 0, self.console_width, self.console_height))
                    .expect("Failed to draw background for console");

                for (index, i) in self.buffer.iter().enumerate() {
                    if i.len() == 0 {
                        continue
                    }
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
            let target_rect = sdl2::rect::Rect::new(
                0,
                (self.console_height - self.p_widget) as i32,
                target_widget,
                self.p_widget,
            );

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

/// Returns the correponding keyboard char for the provided
/// event.
/// If the event is not a representable character, (i.e mouse event).
/// Then None is returned.
fn get_character_from_event(event: &Event) -> Option<char> {
    match event {
        Event::KeyDown {
            keycode, keymod, ..
        }
        | Event::KeyUp {
            keycode, keymod, ..
        } => {
            let p = *keymod;
            let is_upper = if (p & Mod::LSHIFTMOD) == Mod::LSHIFTMOD
                || (p & Mod::RSHIFTMOD) == Mod::RSHIFTMOD
                || (p & Mod::CAPSMOD) == Mod::CAPSMOD
            {
                true
            } else {
                false
            };
            match keycode {
                Some(key) => match key {
                    Keycode::A => {
                        if is_upper {
                            Some('A')
                        } else {
                            println!("{:#?}", event);
                            Some('a')
                        }
                    }
                    Keycode::B => {
                        if is_upper {
                            Some('B')
                        } else {
                            Some('b')
                        }
                    }
                    Keycode::C => {
                        if is_upper {
                            Some('C')
                        } else {
                            Some('c')
                        }
                    }
                    Keycode::D => {
                        if is_upper {
                            Some('D')
                        } else {
                            Some('d')
                        }
                    }
                    Keycode::E => {
                        if is_upper {
                            Some('E')
                        } else {
                            Some('e')
                        }
                    }
                    Keycode::F => {
                        if is_upper {
                            Some('F')
                        } else {
                            Some('f')
                        }
                    }
                    Keycode::H => {
                        if is_upper {
                            Some('H')
                        } else {
                            Some('h')
                        }
                    }
                    Keycode::I => {
                        if is_upper {
                            Some('I')
                        } else {
                            Some('i')
                        }
                    }
                    Keycode::J => {
                        if is_upper {
                            Some('J')
                        } else {
                            Some('j')
                        }
                    }
                    Keycode::K => {
                        if is_upper {
                            Some('K')
                        } else {
                            Some('k')
                        }
                    }
                    Keycode::L => {
                        if is_upper {
                            Some('L')
                        } else {
                            Some('l')
                        }
                    }
                    Keycode::O => {
                        if is_upper {
                            Some('O')
                        } else {
                            Some('o')
                        }
                    }
                    Keycode::S => {
                        if is_upper {
                            Some('S')
                        } else {
                            Some('s')
                        }
                    },
                    Keycode::G => {
                        if is_upper {
                            Some('G')
                        } else {
                            Some('g')
                        }
                    },
                    Keycode::P => {
                        if is_upper {
                            Some('P')
                        } else {
                            Some('p')
                        }
                    }
                    Keycode::T => {
                        if is_upper {
                            Some('T')
                        } else {
                            Some('t')
                        }
                    }
                    Keycode::R => {
                        if is_upper {
                            Some('R')
                        } else {
                            Some('r')
                        }
                    }
                    Keycode::W => {
                        if is_upper {
                            Some('W')
                        } else {
                            Some('w')
                        }
                    }
                    Keycode::N => {
                        if is_upper {
                            Some('N')
                        } else {
                            Some('n')
                        }
                    }
                    Keycode::U => {
                        if is_upper {
                            Some('U')
                        } else {
                            Some('u')
                        }
                    }
                    Keycode::Q => {
                        if is_upper {
                            Some('Q')
                        } else {
                            Some('q')
                        }
                    }
                    Keycode::Comma => {
                        if is_upper {
                            Some('<')
                        } else {
                            Some(',')
                        }
                    }
                    Keycode::Quote => {
                        if is_upper {
                            Some('"')
                        } else {
                            Some('\'')
                        }
                    }
                    Keycode::Num0 => {
                        if is_upper {
                            Some(')')
                        } else {
                            Some('0')
                        }
                    }
                    Keycode::Num1 => {
                        if is_upper {
                            Some('!')
                        } else {
                            Some('1')
                        }
                    }
                    Keycode::Num2 => {
                        if is_upper {
                            Some('@')
                        } else {
                            Some('2')
                        }
                    }
                    Keycode::Num3 => {
                        if is_upper {
                            Some('#')
                        } else {
                            Some('3')
                        }
                    }
                    Keycode::Num4 => {
                        if is_upper {
                            Some('$')
                        } else {
                            Some('4')
                        }
                    }
                    Keycode::Num5 => {
                        if is_upper {
                            Some('%')
                        } else {
                            Some('5')
                        }
                    }
                    Keycode::Num6 => {
                        if is_upper {
                            Some('^')
                        } else {
                            Some('6')
                        }
                    }
                    Keycode::Num7 => {
                        if is_upper {
                            Some('&')
                        } else {
                            Some('7')
                        }
                    }
                    Keycode::Num8 => {
                        if is_upper {
                            Some('*')
                        } else {
                            Some('8')
                        }
                    }
                    Keycode::Num9 => {
                        if is_upper {
                            Some('(')
                        } else {
                            Some('9')
                        }
                    }
                    Keycode::Equals => {
                        if is_upper {
                            Some('+')
                        } else {
                            Some('=')
                        }
                    }
                    Keycode::Minus => {
                        if is_upper {
                            Some('_')
                        } else {
                            Some('-')
                        }
                    }
                    Keycode::Space => Some(' '),
                    Keycode::Backspace => None,
                    Keycode::Tab => None,
                    Keycode::LAlt => None,
                    Keycode::RAlt => None,
                    Keycode::LCtrl => None,
                    Keycode::RCtrl => None,
                    Keycode::Return => None,
                    Keycode::LShift => None,
                    Keycode::Escape => None,
                    Keycode::RShift => None,
                    _ => {
                        todo!("haven't imple {:#?}", keycode)
                    }
                },
                None => None,
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_state() {
        let mut keyboard_s = KeyboardState::new();

        keyboard_s.update(Event::KeyUp {
            timestamp: 0,
            window_id: 1,
            keycode: Some(Keycode::KpEnter),
            scancode: None,
            keymod: sdl2::keyboard::Mod::NOMOD,
            repeat: false,
        });
    }

    #[test]
    fn test_keyboard_char() {
        let event = Event::KeyUp {
            timestamp: 0,
            window_id: 1,
            keycode: Some(Keycode::P),
            scancode: None,
            keymod: sdl2::keyboard::Mod::NOMOD,
            repeat: false,
        };

        assert_eq!(get_character_from_event(&event), 'p');

        let event2 = Event::KeyUp {
            timestamp: 0,
            window_id: 1,
            keycode: Some(Keycode::P),
            scancode: None,
            keymod: sdl2::keyboard::Mod::LSHIFTMOD,
            repeat: false,
        };
        assert_eq!(get_character_from_event(&event), 'P');
    }
}
