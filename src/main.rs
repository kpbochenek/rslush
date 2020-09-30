extern crate sdl2;

mod file_assist;
mod buffer;

use buffer::*;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::{time::Duration, path::PathBuf};
use sdl2::rect::Rect;

use std::{fs};

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

    let file_name = "./src/example.kis";
    let file_text = file_assist::open_file(file_name);

    let mut buffer: Buffer = Buffer::new(file_text, file_name.to_string());

    let mut file_explorer = FileExplorer::new();

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
                Event::KeyDown { keycode, keymod, .. } => {
                    println!("Pressed {:?}", keycode);
                    if keycode == Some(Keycode::O) && keymod.intersects(sdl2::keyboard::Mod::LCTRLMOD) {
                        file_explorer.active = true;
                    } else {
                        if file_explorer.active {
                            match keycode {
                                Some(Keycode::Q) => file_explorer.insert_character("q"),
                                Some(Keycode::W) => file_explorer.insert_character("w"),
                                Some(Keycode::E) => file_explorer.insert_character("e"),
                                Some(Keycode::R) => file_explorer.insert_character("r"),
                                Some(Keycode::T) => file_explorer.insert_character("t"),
                                Some(Keycode::Y) => file_explorer.insert_character("y"),
                                Some(Keycode::U) => file_explorer.insert_character("u"),
                                Some(Keycode::I) => file_explorer.insert_character("i"),
                                Some(Keycode::O) => file_explorer.insert_character("o"),
                                Some(Keycode::P) => file_explorer.insert_character("p"),
                                // middle row
                                Some(Keycode::A) => file_explorer.insert_character("a"),
                                Some(Keycode::S) => file_explorer.insert_character("s"),
                                Some(Keycode::D) => file_explorer.insert_character("d"),
                                Some(Keycode::F) => file_explorer.insert_character("f"),
                                Some(Keycode::G) => file_explorer.insert_character("g"),
                                Some(Keycode::H) => file_explorer.insert_character("h"),
                                Some(Keycode::J) => file_explorer.insert_character("j"),
                                Some(Keycode::K) => file_explorer.insert_character("k"),
                                Some(Keycode::L) => file_explorer.insert_character("l"),
                                // bottom row
                                Some(Keycode::Z) => file_explorer.insert_character("z"),
                                Some(Keycode::X) => file_explorer.insert_character("x"),
                                Some(Keycode::C) => file_explorer.insert_character("c"),
                                Some(Keycode::V) => file_explorer.insert_character("v"),
                                Some(Keycode::B) => file_explorer.insert_character("b"),
                                Some(Keycode::N) => file_explorer.insert_character("n"),
                                Some(Keycode::M) => file_explorer.insert_character("m"),

                                Some(Keycode::Period) => file_explorer.insert_character("."),

                                Some(Keycode::Slash) => {
                                    if let Some(filename) = file_explorer.slash() {
                                        buffer.update(file_assist::open_file(&filename), filename);
                                        file_explorer.active = false;
                                        file_explorer.part = "".to_string();
                                    }
                                },
                                Some(Keycode::Return) => {
                                    if let Some(filename) = file_explorer.confirm_selection() {
                                        buffer.update(file_assist::open_file(&filename), filename);
                                        file_explorer.active = false;
                                        file_explorer.part = "".to_string();
                                    }
                                },
                                Some(Keycode::Backspace) => file_explorer.delete_character(),
                                Some(Keycode::Escape) => file_explorer.quit(),
                                _ => ()
                            }

                        } else if input_mode == InputMode::Insert {
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
        let mut txt = String::from(if input_mode == InputMode::Insert { " INS" } else { " MOD" });
        txt += " | ";
        txt += &format!("{}:{} ({}%)", buffer.cursor.row, buffer.cursor.col, (buffer.cursor.row * 100 / buffer.lines.len() as u32));
        txt += &format!(" [{}] ", buffer.file_name);
        let rendering = dejavu.render(&txt);
        let surface = rendering.blended(WHITE).unwrap();
        let texture = surface.as_texture(&texture_creator).unwrap();
        canvas.copy(&texture, None, sdl2::rect::Rect::new(modeline_from.0 , modeline_from.1, surface.width(), surface.height())).unwrap();

        if file_explorer.active {
            let mut i = 0;
            for e in &file_explorer.proposals {
                let rendering = dejavu.render(e.as_str());
                let surface = rendering.blended(WHITE).unwrap();
                let texture = surface.as_texture(&texture_creator).unwrap();
                let r = sdl2::rect::Rect::new(10, 400 + i, surface.width(), surface.height());
                canvas.copy(&texture, None, r).unwrap();
                i += char_size_y as i32;
            }

            let prompt = format!("[{}] > {}", &file_explorer.current_directory_name , &file_explorer.part);
            let rendering = dejavu.render(&prompt);
            let surface = rendering.blended(WHITE).unwrap();
            let texture = surface.as_texture(&texture_creator).unwrap();
            let r = sdl2::rect::Rect::new(10, 400 + i, surface.width(), surface.height());
            canvas.copy(&texture, None, r).unwrap();
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}


struct FileExplorer {
    current_directory: PathBuf,
    current_directory_name: String,
    active: bool,
    part: String,
    proposals: Vec<String>,
}

impl FileExplorer {
    pub fn new() -> FileExplorer {
        let current_directory: PathBuf = PathBuf::from(".");
        let (name, proposals) = current_directory.to_str().map(|d| (d, FileExplorer::list_files(d))).unwrap_or(("unknown", Vec::new()));
        FileExplorer {
            current_directory: PathBuf::from("."),
            current_directory_name: name.to_string(),
            active: false,
            part: String::from(""),
            proposals,
        }
    }

    pub fn insert_character(&mut self, c: &str) {
        self.part += c;
    }


    pub fn delete_character(&mut self) {
        if !self.part.is_empty() {
        self.part.remove(self.part.len()-1);
        }
    }

    pub fn confirm_selection(&mut self) -> Option<String> {
        let mut cwd = self.current_directory.clone();
        cwd.push(&self.part);
        if cwd.as_path().is_file() {
            println!("Opening file");
            cwd.to_str().map(|s| s.to_string())
        } else if self.part == ".." {
            self.current_directory.pop();
            let (name, proposals) = self.current_directory.to_str().map(|d| (d, FileExplorer::list_files(d))).unwrap_or(("unknown", Vec::new()));
            self.current_directory_name = name.to_string();
            self.proposals = proposals;
            self.part = String::from("");
            None
        } else if cwd.as_path().is_dir() {
            self.current_directory.push(&self.part);
            let (name, proposals) = self.current_directory.to_str().map(|d| (d, FileExplorer::list_files(d))).unwrap_or(("unknown", Vec::new()));
            self.current_directory_name = name.to_string();
            self.proposals = proposals;
            self.part = String::from("");
            None
        } else {
            None
        }
    }

    fn list_files(directory: &str) -> Vec<String> {
        fs::read_dir(directory).unwrap().filter_map(|e| {
            e.unwrap().path().to_str().map(|s| s.to_string())
        }).collect()
    }

    pub fn slash(&mut self) -> Option<String> {
        let mut cwd = self.current_directory.clone();
        cwd.push(&self.part);
        if cwd.as_path().is_file() {
            println!("Opening file");
            cwd.to_str().map(|s| s.to_string())
        } else if self.part == ".." {
            self.current_directory.pop();
            None
        } else if cwd.as_path().is_dir() {
            self.current_directory.push(&self.part);
            self.part = String::from("");
            None
        } else {
            None
        }
    }

    pub fn quit(& mut self) {
        self.active = false;
    }
}
