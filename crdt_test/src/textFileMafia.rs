use std::fmt::Debug;
use std::fs::File;
use std::io::Write;

trait MafiaAuditor {
    fn edit_file(&mut self);
    fn refresh_file(&mut self);
    fn new(file_path: &str) -> Self;
}

pub struct TextFileMafia {
    pub file_from_text_as_blocks: Vec<String>,
    pub file_path: String,
    pub file: File,
}

impl MafiaAuditor for TextFileMafia {
    fn new(file_path: &str) -> Self {
        Self {
            file_from_text_as_blocks: Vec::new(),
            file_path: file_path.to_string(),
            file: read_file(file_path),
        }
    }
}

fn read_file(file_path: &str) -> File {
    let file_opening_result = File::open(file_path);
    if let Ok(file) = file_opening_result {
        return file;
    } else {
        create_file_if_it_doesnt_exist(file_path)
    }
}

fn create_file_if_it_doesnt_exist(file_path: &str) -> File {
    let result = File::create(file_path);
    if result.is_ok() {
        read_file(file_path)
    } else {
        panic!("Failed to create file: {:?}", result);
    }
}

fn convert_text_to_list_of_strings(text_from_file: &str) {}
