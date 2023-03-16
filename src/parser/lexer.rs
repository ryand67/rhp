use std::{fs::File, io::BufReader, path::PathBuf};

pub struct Lexer {
    files: Vec<PathBuf>,
}

impl Lexer {
    pub fn new(files: Vec<PathBuf>) -> Self {
        Self { files }
    }

    pub fn lex_all_files(&mut self) {
        for file in &self.files {
            self.lex_file(&file);
        }
    }

    fn lex_file(&self, file: &PathBuf) {
        todo!();
    }
}
