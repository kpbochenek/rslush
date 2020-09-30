
pub struct Buffer {
    pub file_name: String,
    pub lines: Vec<String>,
    pub cursor: Cursor,
}

pub struct Cursor {
    pub row: u32,
    pub col: u32
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Buffer {
    pub fn new(text: String, file_name: String) -> Buffer {
        let lines = text.lines().map(|l| { l.to_string() }).collect();
        Buffer { file_name, lines, cursor: Cursor { row: 0, col: 0 } }
    }

    pub fn update(&mut self, text: String, file_name: String) {
        self.lines = text.lines().map(|l| { l.to_string() }).collect();
        self.file_name = file_name;
        self.cursor = Cursor { row: 0, col: 0 };
    }

    pub fn move_cursor(&mut self, dir: Direction) {
        match dir {
            Direction::Left => if self.cursor.col > 0 { self.cursor.col -= 1 },
            Direction::Right => if self.cursor.col < self.lines[self.cursor.row as usize].len() as u32 { self.cursor.col += 1 },
            Direction::Up => if self.cursor.row > 0 { self.cursor.row -= 1 },
            Direction::Down => if self.cursor.row < self.lines.len() as u32 - 1 { self.cursor.row += 1 }
        }
        let max_col = self.lines[self.cursor.row as usize].len() as u32;
        if self.cursor.col > max_col {
           self.cursor.col = max_col;
        }
    }

    pub fn insert_newline_below(&mut self) {
        self.lines.insert(self.cursor.row as usize + 1, String::from(""));
    }

    pub fn delete_line(&mut self) {
        if self.lines.len() > 1 {
            self.lines.remove(self.cursor.row as usize);
        }
    }

    pub fn insert_character(&mut self, c: char) {
        println!("Update row={},column={}={}", self.cursor.row, self.cursor.col, c);
        let line = self.lines.get_mut(self.cursor.row as usize).unwrap();
        let (l, r) = line.split_at(self.cursor.col as usize);
        self.lines[self.cursor.row as usize] = format!("{}{}{}", l, c, r);
        self.move_cursor(Direction::Right);
    }

    pub fn delete_current_character(&mut self) {
        if self.cursor.col < self.lines[self.cursor.row as usize].len() as u32 {
            let line = self.lines.get_mut(self.cursor.row as usize).unwrap();
            let (l, r) = line.split_at(self.cursor.col as usize);
            self.lines[self.cursor.row as usize] = format!("{}{}", l, &r[1..]);
        }
    }
}
