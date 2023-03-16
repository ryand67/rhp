use crate::util::open_file;
use std::{io::BufRead, path::PathBuf};

pub struct Lexer {
    files: Vec<PathBuf>,
}

impl Lexer {
    pub fn new(files: Vec<PathBuf>) -> Self {
        return Self { files };
    }

    pub fn lex_all_files(&mut self) {
        for file in &self.files {
            match self.lex_file(&file) {
                Ok(_) => (),
                Err(e) => println!("Error opening file: {:?}", e),
            }
        }
    }

    fn lex_file(&self, path: &PathBuf) -> Result<(), String> {
        let mut reader = match open_file(path) {
            Ok(r) => r,
            Err(e) => {
                return Err(format!("{}", e.to_string()));
            }
        };

        let mut line = String::new();
        while reader.read_line(&mut line).unwrap_or(0) != 0 {
            todo!();
        }

        return Ok(());
    }
}
