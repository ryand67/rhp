use std::{fs::File, io::BufReader};

pub struct Lexer {
    files: Vec<BufReader<File>>,
}

impl Lexer {
    pub fn new(files: Vec<BufReader<File>>) -> Self {
        Self { files }
    }

    pub fn lex_all_files(&mut self) {
        for file in &self.files {
            self.lex_file(&file);
        }
    }

    pub fn lex_file(&self, file: &BufReader<File>) {
        todo!();
    }
}
