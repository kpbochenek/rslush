use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

use crate::config::ColorScheme;

pub struct ItemPicker {
    active: bool,
    items: Vec<Item>,
    filtered: Vec<Item>,
    pub selected_line: usize,
    pub accepted: Option<Item>,
    filter_part: String,
}

#[derive(Clone)]
pub struct Item {
    pub id: usize,
    pub name: String,
    description: Option<String>,
}

impl Item {
    pub fn new(id: usize, name: String) -> Item {
        Item {
            id,
            name,
            description: None,
        }
    }
}

impl ItemPicker {
    pub fn try_handle_key(&mut self, keycode: &Keycode, shift: bool, ctrl: bool) -> bool {
        let mut handled = true;
        if ctrl {
            match &keycode {
                Keycode::J => self.selection_down(),
                Keycode::K => self.selection_up(),
                Keycode::H => self.delete_char(),
                Keycode::L => self.accept_selection(),
                _ => handled = false,
            }
        } else {
            match &keycode {
                Keycode::Num1 => self.insert_char(shft('1', '!', shift)),
                Keycode::Num2 => self.insert_char(shft('2', '@', shift)),
                Keycode::Num3 => self.insert_char(shft('3', '#', shift)),
                Keycode::Num4 => self.insert_char(shft('4', '$', shift)),
                Keycode::Num5 => self.insert_char(shft('5', '%', shift)),
                Keycode::Num6 => self.insert_char(shft('6', '^', shift)),
                Keycode::Num7 => self.insert_char(shft('7', '&', shift)),
                Keycode::Num8 => self.insert_char(shft('8', '*', shift)),
                Keycode::Num9 => self.insert_char(shft('9', '(', shift)),
                Keycode::Num0 => self.insert_char(shft('0', ')', shift)),
                Keycode::Minus => self.insert_char(shft('-', '_', shift)),
                Keycode::Equals => self.insert_char(shft('=', '+', shift)),

                // top row
                Keycode::Q => self.insert_char(shft('q', 'Q', shift)),
                Keycode::W => self.insert_char(shft('w', 'W', shift)),
                Keycode::E => self.insert_char(shft('e', 'E', shift)),
                Keycode::R => self.insert_char(shft('r', 'R', shift)),
                Keycode::T => self.insert_char(shft('t', 'T', shift)),
                Keycode::Y => self.insert_char(shft('y', 'Y', shift)),
                Keycode::U => self.insert_char(shft('u', 'U', shift)),
                Keycode::I => self.insert_char(shft('i', 'I', shift)),
                Keycode::O => self.insert_char(shft('o', 'O', shift)),
                Keycode::P => self.insert_char(shft('p', 'P', shift)),
                Keycode::LeftBracket => self.insert_char(shft('[', '{', shift)),
                Keycode::RightBracket => self.insert_char(shft(']', '}', shift)),
                Keycode::Backslash => self.insert_char(shft('\\', '|', shift)),

                // middle row
                Keycode::A => self.insert_char(shft('a', 'A', shift)),
                Keycode::S => self.insert_char(shft('s', 'S', shift)),
                Keycode::D => self.insert_char(shft('d', 'D', shift)),
                Keycode::F => self.insert_char(shft('f', 'F', shift)),
                Keycode::G => self.insert_char(shft('g', 'G', shift)),
                Keycode::H => self.insert_char(shft('h', 'H', shift)),
                Keycode::J => self.insert_char(shft('j', 'J', shift)),
                Keycode::K => self.insert_char(shft('k', 'K', shift)),
                Keycode::L => self.insert_char(shft('l', 'L', shift)),
                Keycode::Semicolon => self.insert_char(shft(';', ':', shift)),
                Keycode::Quote => self.insert_char(shft('\'', '"', shift)),

                // bottom row
                Keycode::Z => self.insert_char(shft('z', 'Z', shift)),
                Keycode::X => self.insert_char(shft('x', 'X', shift)),
                Keycode::C => self.insert_char(shft('c', 'C', shift)),
                Keycode::V => self.insert_char(shft('v', 'V', shift)),
                Keycode::B => self.insert_char(shft('b', 'B', shift)),
                Keycode::N => self.insert_char(shft('n', 'N', shift)),
                Keycode::M => self.insert_char(shft('m', 'M', shift)),
                Keycode::Comma => self.insert_char(shft(',', '<', shift)),
                Keycode::Period => self.insert_char(shft('.', '>', shift)),
                Keycode::Slash => self.insert_char(shft('/', '?', shift)),

                // other
                Keycode::Space => self.insert_char(' '),

                Keycode::Return => self.accept_selection(),
                Keycode::Backspace => self.delete_char(),
                Keycode::Up => self.selection_up(),
                Keycode::Down => self.selection_down(),
                Keycode::Escape => self.deactivate(),

                _ => handled = false,
            }
        }
        handled
    }
}

impl ItemPicker {
    pub fn new() -> ItemPicker {
        ItemPicker {
            active: false,
            items: Vec::new(),
            filtered: Vec::new(),
            selected_line: 0,
            filter_part: String::from(""),
            accepted: None,
        }
    }

    pub fn is_active(&mut self) -> bool {
        self.active
    }
    pub fn activate(&mut self, items: Vec<Item>) {
        self.active = true;
        self.items = items;
        self.filtered = self.items.iter().cloned().collect();
        self.selected_line = 0;
        self.filter_part.clear();
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn get_accepted(&mut self) -> Option<Item> {
        self.accepted.take()
    }

    pub fn inserted_part(&self) -> &String {
        &self.filter_part
    }

    pub fn get_items(&self) -> &Vec<Item> {
        &self.items
    }

    pub fn get_items_filtered(&self) -> &Vec<Item> {
        &self.filtered
    }

    pub fn selection_up(&mut self) {
        if self.filtered.len() > self.selected_line + 1 {
            self.selected_line += 1;
        }
    }

    pub fn selection_down(&mut self) {
        if self.selected_line > 0 {
            self.selected_line -= 1;
        }
    }

    fn refilter(&mut self) {
        self.filtered = self
            .items
            .iter()
            .filter(|s| self.filter_part.is_empty() || s.name.contains(&self.filter_part))
            .cloned()
            .collect();
        if self.filtered.is_empty() || self.selected_line > self.filtered.len() - 1 {
            self.selected_line = 0;
        }
    }

    fn accept_selection(&mut self) {
        match self.filtered.get(self.selected_line) {
            Some(item) => self.accepted = Some(item.clone()),
            None => (),
        }
    }

    fn insert_char(&mut self, c: char) {
        self.filter_part.push(c);
        self.refilter();
    }

    fn delete_char(&mut self) {
        if !self.filter_part.is_empty() {
            self.filter_part.pop();
            self.refilter();
        };
    }
}

pub struct ItemPickerDisplay {}

impl ItemPickerDisplay {
    pub fn display(
        picker: &ItemPicker,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        cs: &ColorScheme,
        draw_area: Rect,
        _char_size_x: u32,
        char_size_y: u32,
        font: &sdl2::ttf::Font,
        texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    ) {
        canvas.set_draw_color(cs.itempicker_bg);
        canvas.fill_rect(draw_area).unwrap();
        canvas.set_draw_color(cs.itempicker_border);
        canvas.draw_rect(draw_area).unwrap();

        let items_len = picker.get_items().len();
        let fit_into_display = (draw_area.height() / char_size_y - 1) as usize;

        // draw prompt
        let prompt = format!(
            "[T:{}] ({}/{}) Filter: {}",
            items_len,
            picker.selected_line + 1,
            usize::min(fit_into_display, items_len),
            picker.inserted_part()
        );
        let rendering = font.render(&prompt);
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

        for (id, e) in picker
            .get_items_filtered()
            .iter()
            .take(fit_into_display)
            .enumerate()
        {
            if id == picker.selected_line {
                canvas.set_draw_color(cs.itempicker_selection);
                canvas
                    .fill_rect(Rect::new(r.x() + 2, i, draw_area.width() - 4, char_size_y))
                    .unwrap();
            }
            let rendering = font.render(&e.name);
            let surface = rendering.blended(cs.itempicker_fg).unwrap();
            let texture = surface.as_texture(&texture_creator).unwrap();
            let rt = Rect::new(r.x() + 5, i, surface.width(), surface.height());
            canvas.copy(&texture, None, rt).unwrap();

            i -= char_size_y as i32;
        }
    }
}

fn shft(no_shift: char, shift: char, is_shift: bool) -> char {
    if is_shift {
        shift
    } else {
        no_shift
    }
}
