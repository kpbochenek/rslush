extern crate sdl2;

mod buffer;
mod file_assist;

use buffer::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Mod;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Instant;
use std::{path::PathBuf, time::Duration};

use std::fs;

const BLACK: Color = Color::RGB(0, 0, 0);
const WHITE: Color = Color::RGB(255, 255, 255);
const BLUE: Color = Color::RGB(30, 144, 255);
const BROWN: Color = Color::RGB(205, 133, 63);
const GRAY: Color = Color::RGB(169, 169, 169);
const DIM_GRAY: Color = Color::RGB(105, 105, 105);
const SLATE: Color = Color::RGB(47, 79, 79);

const STARTX: u32 = 0;
const STARTY: u32 = 0;

#[derive(PartialEq)]
enum InputMode {
    Insert,
    Normal,
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Slush", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let file_name = "./src/example.kis";
    let file_text = file_assist::open_file(file_name);

    let mut buffer: Buffer = Buffer::new(file_text, file_name.to_string());

    let mut second_now = Instant::now();
    let mut fps_tick: u32 = 0;
    let mut fps_draw: String = String::from("?");

    let mut file_explorer = FileExplorer::new(".");

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
        let tnow = Instant::now();
        fps_tick += 1;
        if tnow.duration_since(second_now).as_millis() >= 1000 {
            fps_draw = format!("FPS {}", fps_tick);
            second_now = tnow;
            fps_tick = 0;
        }
        let (windowx, windowy) = canvas.window().size();
        canvas.set_draw_color(BLACK);
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Backquote),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode, keymod, ..
                } => {
                    let ctrl = keymod.intersects(Mod::LCTRLMOD);
                    let shift = keymod.intersects(Mod::LSHIFTMOD);
                    // println!("Pressed {:?} ctrl:{} shift:{}", keycode, ctrl, shift);
                    if ctrl && keycode == Some(Keycode::O) {
                        file_explorer.activate();
                    } else {
                        if file_explorer.active {
                            if ctrl {
                                match keycode {
                                    Some(Keycode::J) => file_explorer.selection_down(),
                                    Some(Keycode::K) => file_explorer.selection_up(),
                                    Some(Keycode::H) => file_explorer.delete_segment(),
                                    Some(Keycode::L) => {
                                        if let Some(filename) = file_explorer.confirm_selection() {
                                            buffer.update(
                                                file_assist::open_file(&filename),
                                                filename,
                                            );
                                            file_explorer.deactivate();
                                        }
                                    }
                                    _ => (),
                                }
                            } else {
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

                                    Some(Keycode::Up) => file_explorer.selection_up(),
                                    Some(Keycode::Down) => file_explorer.selection_down(),

                                    Some(Keycode::Return) => {
                                        if let Some(filename) = file_explorer.confirm_selection() {
                                            buffer.update(
                                                file_assist::open_file(&filename),
                                                filename,
                                            );
                                            file_explorer.deactivate();
                                        }
                                    }
                                    Some(Keycode::Backspace) => file_explorer.delete_character(),
                                    Some(Keycode::Escape) => file_explorer.quit(),
                                    _ => (),
                                }
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
                                _ => (),
                            }
                        } else {
                            //NormalMode
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
                                }
                                Some(Keycode::X) => {
                                    buffer.delete_current_character();
                                }
                                _ => (),
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
                canvas
                    .copy(
                        &texture,
                        None,
                        sdl2::rect::Rect::new(
                            STARTX as i32,
                            (STARTY + surface.height() * i) as i32,
                            surface.width(),
                            surface.height(),
                        ),
                    )
                    .unwrap();
            }
            i += 1;
        }

        canvas.set_draw_color(BLUE);
        let from = (
            (char_size_x * buffer.cursor.col) as i32,
            (char_size_y * buffer.cursor.row) as i32,
        );
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
        let mut txt = String::from(if input_mode == InputMode::Insert {
            " INS"
        } else {
            " MOD"
        });
        txt += " | ";
        txt += &format!(
            "{}:{} ({:2}%)",
            buffer.cursor.row,
            buffer.cursor.col,
            (buffer.cursor.row * 100 / buffer.lines.len() as u32)
        );
        txt += &format!(" [{}] ", buffer.file_name);
        let rendering = dejavu.render(&txt);
        let surface = rendering.blended(WHITE).unwrap();
        let texture = surface.as_texture(&texture_creator).unwrap();
        canvas
            .copy(
                &texture,
                None,
                Rect::new(
                    modeline_from.0,
                    modeline_from.1,
                    surface.width(),
                    surface.height(),
                ),
            )
            .unwrap();

        let rendering = dejavu.render(&fps_draw);
        let surface = rendering.blended(WHITE).unwrap();
        let texture = surface.as_texture(&texture_creator).unwrap();
        canvas
            .copy(
                &texture,
                None,
                Rect::new(
                    windowx as i32 - 60,
                    modeline_from.1,
                    surface.width(),
                    surface.height(),
                ),
            )
            .unwrap();

        if file_explorer.active {
            let draw_area: Rect = Rect::new(
                10,
                windowy as i32 / 2,
                windowx - 20,
                windowy / 2 - 2 * char_size_y,
            );
            let items_space_y = draw_area.height() - char_size_y;
            let items_count = items_space_y / char_size_y;

            // draw area
            canvas.set_draw_color(DIM_GRAY);
            canvas.fill_rect(draw_area).unwrap();
            canvas.set_draw_color(GRAY);
            canvas.draw_rect(draw_area).unwrap();

            // draw prompt
            let prompt = format!(
                "[{}/{}] ({})> {}",
                &file_explorer.filtered.len(),
                &file_explorer.proposals.len(),
                &file_explorer.current_directory.name,
                &file_explorer.part
            );
            let rendering = dejavu.render(&prompt);
            let surface = rendering.blended(WHITE).unwrap();
            let texture = surface.as_texture(&texture_creator).unwrap();
            let mut r = Rect::new(
                draw_area.x(),
                draw_area.y() + draw_area.height() as i32 - char_size_y as i32,
                surface.width(),
                surface.height(),
            );
            canvas.copy(&texture, None, r).unwrap();
            r.set_width(draw_area.width());
            canvas.draw_rect(r).unwrap();

            // draw items
            let mut i: i32 = r.y - char_size_y as i32;

            let mut id = 0;
            for e in file_explorer.get_items(items_count as usize) {
                if id == file_explorer.selected {
                    canvas.set_draw_color(SLATE);
                    canvas
                        .fill_rect(Rect::new(10, i, draw_area.width(), char_size_y))
                        .unwrap();
                }
                let rendering = dejavu.render(e.name.as_str());
                let surface = rendering.blended(WHITE).unwrap();
                let texture = surface.as_texture(&texture_creator).unwrap();
                let r = Rect::new(15, i, surface.width(), surface.height());
                canvas.copy(&texture, None, r).unwrap();

                i -= char_size_y as i32;
                id += 1;
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

#[derive(Clone)]
struct FileEntity {
    name: String,
    path: PathBuf,
}

impl FileEntity {
    // fn new(name: String, path: PathBuf) -> FileEntity {
    //     FileEntity { name, path }
    // }

    fn from_path(path: PathBuf) -> FileEntity {
        let name = String::from(path.to_str().unwrap());
        FileEntity { name, path }
    }
}

struct FileExplorer {
    current_directory: FileEntity,
    active: bool,
    selected: u32,
    part: String,
    proposals: Vec<FileEntity>,
    filtered: Vec<FileEntity>,
}

impl FileExplorer {
    pub fn new(current: &str) -> FileExplorer {
        let path: PathBuf = fs::canonicalize(PathBuf::from(current)).unwrap();
        FileExplorer {
            current_directory: FileEntity::from_path(path),
            selected: 0,
            active: false,
            part: String::from(""),
            filtered: Vec::new(),
            proposals: Vec::new(),
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.change_dir(self.current_directory.path.clone());
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.proposals = Vec::new();
        self.filtered = Vec::new();
    }

    fn change_dir(&mut self, new_dir: PathBuf) {
        self.current_directory = FileEntity::from_path(new_dir);
        self.part = String::from("");
	self.selected = 0;
        self.refresh_proposals_filtered();
    }

    pub fn insert_character(&mut self, c: &str) {
        self.part += c;
        self.refilter();
    }

    fn refresh_proposals_filtered(&mut self) {
        self.proposals = self
            .current_directory
            .path
            .to_str()
            .map(|d| FileExplorer::list_files(d))
            .unwrap_or(Vec::new());
        self.filtered = self.proposals.iter().map(|e| e.clone()).collect();
    }

    fn refilter(&mut self) {
        self.filtered = self
            .proposals
            .iter()
            .filter(|s| self.part.is_empty() || s.name.contains(&self.part))
            .cloned()
            .collect();
        if self.selected > self.filtered.len() as u32 {
            self.selected = 0;
        }
    }

    pub fn get_items(&mut self, size: usize) -> Vec<FileEntity> {
        if size < self.filtered.len() {
            self.filtered.split_at(size).0.to_vec()
        } else {
            self.filtered.to_vec()
        }
    }

    pub fn selection_up(&mut self) {
        if self.filtered.len() as u32 > self.selected + 1 {
            self.selected += 1;
        }
    }

    pub fn selection_down(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn delete_segment(&mut self) {
        let mut new_dir = self.current_directory.path.clone();
        new_dir.pop();
        self.change_dir(new_dir);
    }

    pub fn delete_character(&mut self) {
        if !self.part.is_empty() {
            self.part.pop();
            self.refilter();
        }
    }

    pub fn confirm_selection(&mut self) -> Option<String> {
        let cwd = self.filtered.get(self.selected as usize).map(|e| e.clone());
        match cwd {
            Some(file_entry) => {
                let path = file_entry.path.as_path();
                if path.is_file() {
                    println!("Opening file {:?}", path.to_str());
                    path.to_str().map(|s| s.to_string())
                } else if path.is_dir() {
                    println!("Goto directory {:?}", path.to_str());
                    self.change_dir(file_entry.path.clone());
                    None
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn quit(&mut self) {
        self.active = false;
    }

    // FileEntity related methods

    fn list_files(directory: &str) -> Vec<FileEntity> {
        match fs::read_dir(directory) {
            Ok(dir_entry) => {
                let c: Vec<fs::DirEntry> = dir_entry.filter_map(|e| e.ok()).collect();
                c.iter().map(|x| FileEntity::from_path(x.path())).collect()
            }
            Err(_) => Vec::new(),
        }
    }
}
