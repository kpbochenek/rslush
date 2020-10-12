extern crate sdl2;

mod buffer;
mod config;
mod file_assist;
mod file_picker;

use buffer::*;
use config::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Mod;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;
use std::time::Instant;

use file_picker::*;

const STARTX: u32 = 0;
const STARTY: u32 = 0;

#[derive(PartialEq)]
enum InputMode {
    Insert,
    Normal,
}

enum FilePickerAction {
    OpenFile,
    ChangeColorScheme,
}

struct Actions {
    active: bool,
    part: String,
    selected: u32,
    commands: Vec<CommandAction>,
}

enum CommandAction {
    OpenFile,
    ChangeColorScheme,
}

impl CommandAction {
    pub fn name(&self) -> &str {
        match self {
            Self::OpenFile => "Open File",
            Self::ChangeColorScheme => "Change Color Scheme",
        }
    }
}

impl Actions {
    pub fn new() -> Actions {
        Actions {
            active: false,
            part: String::from(""),
            selected: 0,
            commands: vec![CommandAction::OpenFile, CommandAction::ChangeColorScheme],
        }
    }

    pub fn get_items(&self) -> &Vec<CommandAction> {
        &self.commands
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn selected_line(&self) -> u32 {
        self.selected
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn inserted_part(&self) -> &String {
        &self.part
    }
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

    let mut cs = DEFAULT_CS;

    let mut buffer: Buffer = Buffer::new(file_text, file_name.to_string());

    let mut fp_action: FilePickerAction = FilePickerAction::OpenFile;

    let mut second_now = Instant::now();
    let mut fps_tick: u32 = 0;
    let mut fps_draw: String = String::from("?");

    let mut display_from: usize = 0;

    let mut file_explorer = FilePicker::new(".");
    let mut actions = Actions::new();

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
        canvas.set_draw_color(cs.buffer_bg);
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
                    let ctrl = keymod.intersects(Mod::LCTRLMOD) || keymod.intersects(Mod::RCTRLMOD);
                    let shift =
                        keymod.intersects(Mod::LSHIFTMOD) || keymod.intersects(Mod::RSHIFTMOD);
                    // println!("Pressed {:?} ctrl:{} shift:{}", keycode, ctrl, shift);
                    if ctrl && keycode == Some(Keycode::O) {
                        file_explorer.activate();
                        fp_action = FilePickerAction::OpenFile;
                    } else if ctrl && keycode == Some(Keycode::P) {
                        file_explorer.activate();
                        fp_action = FilePickerAction::ChangeColorScheme;
                    } else if ctrl && keycode == Some(Keycode::M) {
                        actions.activate();
                    } else {
                        if file_explorer.is_active() {
                            if ctrl {
                                match keycode {
                                    Some(Keycode::J) => file_explorer.selection_down(),
                                    Some(Keycode::K) => file_explorer.selection_up(),
                                    Some(Keycode::H) => file_explorer.delete_segment(),
                                    Some(Keycode::L) => {
                                        if let Some(filename) = file_explorer.confirm_selection() {
                                            file_explorer.deactivate();
                                            match fp_action {
                                                FilePickerAction::OpenFile => {
                                                    open_file(&mut buffer, &filename)
                                                }
                                                FilePickerAction::ChangeColorScheme => {
                                                    match change_color_scheme(filename) {
                                                        Ok(color_scheme) => cs = color_scheme,
                                                        Err(msg) => display_message(msg),
                                                    }
                                                }
                                            }
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
                                            file_explorer.deactivate();
                                            match fp_action {
                                                FilePickerAction::OpenFile => {
                                                    open_file(&mut buffer, &filename)
                                                }
                                                FilePickerAction::ChangeColorScheme => {
                                                    match change_color_scheme(filename) {
                                                        Ok(color_scheme) => cs = color_scheme,
                                                        Err(msg) => display_message(msg),
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Some(Keycode::Backspace) => file_explorer.delete_character(),
                                    Some(Keycode::Escape) => file_explorer.quit(),
                                    _ => (),
                                }
                            }
                        } else if actions.is_active() {
                            match keycode {
                                Some(Keycode::Escape) => actions.deactivate(),
                                _ => (),
                            }
                        } else if input_mode == InputMode::Insert {
                            if keycode == Some(Keycode::Escape) {
                                input_mode = InputMode::Normal;
                                buffer.move_cursor(Direction::Left);
                            }
                            match keycode {
                                Some(k) => handle_key_ins_mode(k, shift, &mut buffer),
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
                                    input_mode = InputMode::Insert;
                                }
                                Some(Keycode::S) => {
                                    file_assist::save_file(&buffer.file_name, &buffer.lines);
                                    buffer.saved()
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

        let rows_displayed: usize = ((windowy - char_size_y) / char_size_y) as usize;
        let display_to = usize::min(display_from + rows_displayed, buffer.lines.len());
        let mut i: u32 = 0;
        for l in &buffer.lines[display_from..display_to] {
            let lne = format!("{:3}|{}", i + display_from as u32, l);
            let rendering = dejavu.render(&lne);
            let surface = rendering.blended(cs.buffer_fg).unwrap();
            let texture = surface.as_texture(&texture_creator).unwrap();
            canvas
                .copy(
                    &texture,
                    None,
                    Rect::new(
                        STARTX as i32,
                        (STARTY + surface.height() * i) as i32,
                        surface.width(),
                        surface.height(),
                    ),
                )
                .unwrap();
            i += 1;
        }

        if (buffer.cursor.row as usize) < display_from {
            display_from = buffer.cursor.row as usize;
        }
        if buffer.cursor.row as usize > display_to - 1
            && buffer.cursor.row < buffer.lines.len() as u32 - 1
        {
            display_from += (buffer.cursor.row as usize - (display_to - 1)) as usize;
        }

        canvas.set_draw_color(cs.cursor);
        let from = (
            (char_size_x * buffer.cursor.col) as i32 + 4 * char_size_x as i32,
            (char_size_y * buffer.cursor.row - char_size_y * display_from as u32) as i32,
        );
        if input_mode == InputMode::Insert {
            let to = (from.0, from.1 + char_size_y as i32);
            canvas.draw_line(from, to).unwrap();
        } else {
            let rct = Rect::new(from.0, from.1, char_size_x, char_size_y);
            canvas.draw_rect(rct).unwrap();
        }

        // draw modeline
        canvas.set_draw_color(cs.statusline_bg);
        let modeline_from = (0, (windowy - char_size_y) as i32);
        let modeline_to = (windowx as i32, (windowy - char_size_y) as i32);
        canvas.draw_line(modeline_from, modeline_to).unwrap();
        let mut txt = String::from(if input_mode == InputMode::Insert {
            " INSERT"
        } else {
            " NORMAL"
        });
        txt += " | ";
        txt += &format!(
            "{}:{} ({:2}%)",
            buffer.cursor.row,
            buffer.cursor.col,
            (buffer.cursor.row * 100 / buffer.lines.len() as u32)
        );
        if buffer.modified {
            txt += " *M* ";
        }
        txt += &format!(" [{}] ", buffer.file_name);
        let rendering = dejavu.render(&txt);
        let surface = rendering.blended(cs.statusline_fg).unwrap();
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
        let surface = rendering.blended(cs.statusline_fg).unwrap();
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

        if file_explorer.is_active() {
            let draw_area: Rect = Rect::new(
                10,
                windowy as i32 / 2,
                windowx - 20,
                windowy / 2 - 2 * char_size_y,
            );
            let items_space_y = draw_area.height() - char_size_y;
            let items_count = items_space_y / char_size_y;

            // draw area
            canvas.set_draw_color(cs.filepicker_bg);
            canvas.fill_rect(draw_area).unwrap();
            canvas.set_draw_color(cs.filepicker_border);
            canvas.draw_rect(draw_area).unwrap();

            // draw prompt
            let stats = file_explorer.selection_stats();
            let prompt = format!(
                "[{}/{}] ({})> {}",
                stats.1,
                stats.0,
                file_explorer.current_directory_name(),
                file_explorer.inserted_part()
            );
            let rendering = dejavu.render(&prompt);
            let surface = rendering.blended(cs.filepicker_fg).unwrap();
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
                if id == file_explorer.selected_line() {
                    canvas.set_draw_color(cs.filepicker_selection);
                    canvas
                        .fill_rect(Rect::new(10, i, draw_area.width(), char_size_y))
                        .unwrap();
                }
                let rendering = dejavu.render(e.name.as_str());
                let surface = rendering.blended(cs.filepicker_fg).unwrap();
                let texture = surface.as_texture(&texture_creator).unwrap();
                let r = Rect::new(15, i, surface.width(), surface.height());
                canvas.copy(&texture, None, r).unwrap();

                i -= char_size_y as i32;
                id += 1;
            }
        }
        if actions.is_active() {
            let draw_area: Rect = Rect::new(
                20,
                windowy as i32 / 2,
                windowx - 40,
                windowy / 2 - 2 * char_size_y,
            );
            canvas.set_draw_color(cs.actions_bg);
            canvas.fill_rect(draw_area).unwrap();
            canvas.set_draw_color(cs.actions_border);
            canvas.draw_rect(draw_area).unwrap();

            // draw prompt
            let prompt = format!(" Run: {}", actions.inserted_part());
            let rendering = dejavu.render(&prompt);
            let surface = rendering.blended(cs.filepicker_fg).unwrap();
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
            for e in actions.get_items() {
                if id == actions.selected_line() {
                    canvas.set_draw_color(cs.actions_selection);
                    canvas
                        .fill_rect(Rect::new(10, i, draw_area.width(), char_size_y))
                        .unwrap();
                }
                let rendering = dejavu.render(e.name());
                let surface = rendering.blended(cs.actions_fg).unwrap();
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

fn open_file(buffer: &mut Buffer, filename: &String) {
    buffer.update(file_assist::open_file(&filename), filename);
}

fn change_color_scheme(filename: String) -> Result<ColorScheme, String> {
    ColorScheme::read_from_file(filename)
}

fn display_message(msg: String) {
    println!("Display Message! {} ", msg);
}

fn shft(no_shift: char, shift: char, is_shift: bool) -> char {
    if is_shift {
        shift
    } else {
        no_shift
    }
}

fn handle_key_ins_mode(keycode: Keycode, shift: bool, buffer: &mut Buffer) {
    match keycode {
        // numbers
        Keycode::Num1 => buffer.insert_char(shft('1', '!', shift)),
        Keycode::Num2 => buffer.insert_char(shft('2', '@', shift)),
        Keycode::Num3 => buffer.insert_char(shft('3', '#', shift)),
        Keycode::Num4 => buffer.insert_char(shft('4', '$', shift)),
        Keycode::Num5 => buffer.insert_char(shft('5', '%', shift)),
        Keycode::Num6 => buffer.insert_char(shft('6', '^', shift)),
        Keycode::Num7 => buffer.insert_char(shft('7', '&', shift)),
        Keycode::Num8 => buffer.insert_char(shft('8', '*', shift)),
        Keycode::Num9 => buffer.insert_char(shft('9', '(', shift)),
        Keycode::Num0 => buffer.insert_char(shft('0', ')', shift)),
        Keycode::Minus => buffer.insert_char(shft('-', '_', shift)),
        Keycode::Equals => buffer.insert_char(shft('=', '+', shift)),

        // top row
        Keycode::Q => buffer.insert_char(shft('q', 'Q', shift)),
        Keycode::W => buffer.insert_char(shft('w', 'W', shift)),
        Keycode::E => buffer.insert_char(shft('e', 'E', shift)),
        Keycode::R => buffer.insert_char(shft('r', 'R', shift)),
        Keycode::T => buffer.insert_char(shft('t', 'T', shift)),
        Keycode::Y => buffer.insert_char(shft('y', 'Y', shift)),
        Keycode::U => buffer.insert_char(shft('u', 'U', shift)),
        Keycode::I => buffer.insert_char(shft('i', 'I', shift)),
        Keycode::O => buffer.insert_char(shft('o', 'O', shift)),
        Keycode::P => buffer.insert_char(shft('p', 'P', shift)),
        Keycode::LeftBracket => buffer.insert_char(shft('[', '{', shift)),
        Keycode::RightBracket => buffer.insert_char(shft(']', '}', shift)),
        Keycode::Backslash => buffer.insert_char(shft('\\', '|', shift)),

        // middle row
        Keycode::A => buffer.insert_char(shft('a', 'A', shift)),
        Keycode::S => buffer.insert_char(shft('s', 'S', shift)),
        Keycode::D => buffer.insert_char(shft('d', 'D', shift)),
        Keycode::F => buffer.insert_char(shft('f', 'F', shift)),
        Keycode::G => buffer.insert_char(shft('g', 'G', shift)),
        Keycode::H => buffer.insert_char(shft('h', 'H', shift)),
        Keycode::J => buffer.insert_char(shft('j', 'J', shift)),
        Keycode::K => buffer.insert_char(shft('k', 'K', shift)),
        Keycode::L => buffer.insert_char(shft('l', 'L', shift)),
        Keycode::Semicolon => buffer.insert_char(shft(';', ':', shift)),
        Keycode::Quote => buffer.insert_char(shft('\'', '"', shift)),

        // bottom row
        Keycode::Z => buffer.insert_char(shft('z', 'Z', shift)),
        Keycode::X => buffer.insert_char(shft('x', 'X', shift)),
        Keycode::C => buffer.insert_char(shft('c', 'C', shift)),
        Keycode::V => buffer.insert_char(shft('v', 'V', shift)),
        Keycode::B => buffer.insert_char(shft('b', 'B', shift)),
        Keycode::N => buffer.insert_char(shft('n', 'N', shift)),
        Keycode::M => buffer.insert_char(shft('m', 'M', shift)),
        Keycode::Comma => buffer.insert_char(shft(',', '<', shift)),
        Keycode::Period => buffer.insert_char(shft('.', '>', shift)),
        Keycode::Slash => buffer.insert_char(shft('/', '?', shift)),

        // other
        Keycode::Space => buffer.insert_char(' '),

        Keycode::Return => {
            buffer.enter_newline();
            buffer.move_cursor(Direction::Down);
            buffer.move_cursor_beginning_line();
        }

        Keycode::Backspace => {
            if buffer.cursor.col > 0 {
                buffer.move_cursor(Direction::Left);
                buffer.delete_current_character();
            }
        }
        _ => (),
    }
}
