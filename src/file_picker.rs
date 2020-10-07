use std::fs;
use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq)]
pub struct FileEntity {
    pub name: String,
    pub path: PathBuf,
}

impl PartialOrd for FileEntity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FileEntity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name
            .to_lowercase()
            .cmp(&other.name.to_lowercase())
            .reverse()
    }
}

impl FileEntity {
    fn from_path(path: PathBuf) -> FileEntity {
        let mut name = String::from(path.file_name().unwrap().to_str().unwrap());
        if path.is_dir() {
            name.push('/');
        }
        FileEntity { name, path }
    }
}

pub struct FilePicker {
    pub current_directory: FileEntity,
    active: bool,
    selected: u32,
    part: String,
    proposals: Vec<FileEntity>,
    filtered: Vec<FileEntity>,
}

impl FilePicker {
    pub fn new(current: &str) -> FilePicker {
        let path: PathBuf = fs::canonicalize(PathBuf::from(current)).unwrap();
        FilePicker {
            current_directory: FileEntity::from_path(path),
            selected: 0,
            active: false,
            part: String::from(""),
            filtered: Vec::new(),
            proposals: Vec::new(),
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn inserted_part(&self) -> &String {
        &self.part
    }

    pub fn current_directory_name(&self) -> &str {
        self.current_directory
            .path
            .to_str()
            .unwrap_or(&self.current_directory.name)
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.change_dir(self.current_directory.path.clone());
    }

    pub fn selected_line(&self) -> u32 {
        self.selected
    }

    pub fn selection_stats(&self) -> (usize, usize) {
        (self.proposals.len(), self.filtered.len())
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
            .map(|d| FilePicker::list_files(d))
            .unwrap_or(Vec::new());
        self.proposals.sort();
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

    pub fn get_items(&self, size: usize) -> Vec<FileEntity> {
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

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
