extern crate sdl2;

mod file_assist;
mod buffer;

use buffer::*;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;

const BLACK: Color = Color::RGB(0, 0, 0);
const WHITE: Color = Color::RGB(255, 255, 255);
const BLUE: Color = Color::RGB(30,144,255);
const BROWN: Color = Color::RGB(205,133,63);

const startx: u32 = 0;
const starty: u32 = 0;

#[derive(PartialEq)]
enum InputMode {
    Insert,
    Normal,
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Slush", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let file_text = file_assist::open_test();
    let lines: Vec<String> = file_text.lines().map(|l| { l.to_string() }).collect();

    let mut buffer: Buffer = Buffer { lines, cursor: Cursor { row: 0, col: 0 } };


    let mut input_mode: InputMode = InputMode::Normal;

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let ttf_context = sdl2::ttf::init().unwrap();
    let mut dejavu = ttf_context.load_font("./src/Roboto.ttf", 13).unwrap();
    let (char_size_x, char_size_y) = dejavu.size_of_char('a').unwrap();

    dejavu.set_style(sdl2::ttf::FontStyle::NORMAL);
    dejavu.set_hinting(sdl2::ttf::Hinting::Mono);
    dejavu.set_kerning(true);

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let (windowx, windowy) = canvas.window().size();
        canvas.set_draw_color(BLACK);
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Backquote), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode, .. } => {
                    println!("Pressed {:?}", keycode);
                    if input_mode == InputMode::Insert {
                        if keycode == Some(Keycode::Escape) {
                            input_mode = InputMode::Normal;
                            buffer.move_cursor(Direction::Left);
                        }
                        match keycode {
                            // top row
                            Some(Keycode::Q) => buffer.insert_character('q'),
                            Some(Keycode::W) => buffer.insert_character('w'),
                            Some(Keycode::E) => buffer.insert_character('e'),
                            Some(Keycode::R) => buffer.insert_character('r'),
                            Some(Keycode::T) => buffer.insert_character('t'),
                            Some(Keycode::Y) => buffer.insert_character('y'),
                            Some(Keycode::U) => buffer.insert_character('u'),
                            Some(Keycode::I) => buffer.insert_character('i'),
                            Some(Keycode::O) => buffer.insert_character('o'),
                            Some(Keycode::P) => buffer.insert_character('p'),
                            // middle row
                            Some(Keycode::A) => buffer.insert_character('a'),
                            Some(Keycode::S) => buffer.insert_character('s'),
                            Some(Keycode::D) => buffer.insert_character('d'),
                            Some(Keycode::F) => buffer.insert_character('f'),
                            Some(Keycode::G) => buffer.insert_character('g'),
                            Some(Keycode::H) => buffer.insert_character('h'),
                            Some(Keycode::J) => buffer.insert_character('j'),
                            Some(Keycode::K) => buffer.insert_character('k'),
                            Some(Keycode::L) => buffer.insert_character('l'),
                            // bottom row
                            Some(Keycode::Z) => buffer.insert_character('z'),
                            Some(Keycode::X) => buffer.insert_character('x'),
                            Some(Keycode::C) => buffer.insert_character('c'),
                            Some(Keycode::V) => buffer.insert_character('v'),
                            Some(Keycode::B) => buffer.insert_character('b'),
                            Some(Keycode::N) => buffer.insert_character('n'),
                            Some(Keycode::M) => buffer.insert_character('m'),
                            // other
                            Some(Keycode::Space) => buffer.insert_character(' '),
                            Some(Keycode::Return) => {
                                buffer.insert_newline_below();
                                buffer.move_cursor(Direction::Down);
                            }

                            Some(Keycode::Backspace) => {
                                if buffer.cursor.col > 0 {
                                    buffer.move_cursor(Direction::Left);
                                    buffer.delete_current_character();
                                }
                            }
                            _ => ()
                        }
                    } else {    //NormalMode
                        match keycode {
                            Some(Keycode::H) => buffer.move_cursor(Direction::Left),
                            Some(Keycode::L) => buffer.move_cursor(Direction::Right),
                            Some(Keycode::J) => buffer.move_cursor(Direction::Down),
                            Some(Keycode::K) => buffer.move_cursor(Direction::Up),

                            Some(Keycode::D) => buffer.delete_line(),
                            Some(Keycode::O) => {
                                buffer.insert_newline_below();
                                buffer.move_cursor(Direction::Down);
                            }

                            Some(Keycode::I) => input_mode = InputMode::Insert,
                            Some(Keycode::A) => {
                                input_mode = InputMode::Insert;
                                buffer.move_cursor(Direction::Right);
                            },
                            Some(Keycode::X) => {
                                buffer.delete_current_character();
                            }
                            _ => ()

                        }

                    }

                }
                _ => {}
            }
        }

        let mut i = 0;
        for l in &buffer.lines {
            if l.len() > 0 {
                let rendering = dejavu.render(l);
                let surface = rendering.blended(WHITE).unwrap();
                let texture = surface.as_texture(&texture_creator).unwrap();
                canvas.copy(&texture, None, sdl2::rect::Rect::new(startx as i32 , (starty + surface.height() * i) as i32, surface.width(), surface.height())).unwrap();
            }
            i += 1;
        };

        canvas.set_draw_color(BLUE);
        let from = ((char_size_x * buffer.cursor.col) as i32, (char_size_y * buffer.cursor.row) as i32);
        if input_mode == InputMode::Insert {
            let to = (from.0, from.1 + char_size_y as i32);
            canvas.draw_line(from, to).unwrap();
        } else {
            let rct = Rect::new(from.0, from.1, char_size_x, char_size_y);
            canvas.draw_rect(rct).unwrap();
        }

        // draw modeline
        canvas.set_draw_color(BROWN);
        let modeline_from = (0, (windowy - char_size_y) as i32);
        let modeline_to = (windowx as i32, (windowy - char_size_y) as i32);
        canvas.draw_line(modeline_from, modeline_to).unwrap();
        let mut txt = String::from(if input_mode == InputMode::Insert { "INS" } else { "MOD" });
        txt += " | ";
        txt += &format!("{}:{} ({}%)", buffer.cursor.row, buffer.cursor.col, (buffer.cursor.row * 100 / buffer.lines.len() as u32));
        let rendering = dejavu.render(&txt);
        let surface = rendering.blended(WHITE).unwrap();
        let texture = surface.as_texture(&texture_creator).unwrap();
        canvas.copy(&texture, None, sdl2::rect::Rect::new(modeline_from.0 , modeline_from.1, surface.width(), surface.height())).unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

