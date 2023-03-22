use crate::parser::token::*;
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

        let mut line_buf = String::new();
        let mut mid_parse = true;
        let mut tag_buf = String::new();

        while reader.read_line(&mut line_buf).unwrap_or(0) != 0 {
            if line_buf.ends_with("\n") {
                line_buf = line_buf.replace("\n", "");
            }

            if mid_parse
                && line_buf.trim().starts_with("<")
                && line_buf.ends_with(">")
                && !line_buf.ends_with("/>")
            {
                tag_buf.push_str(&line_buf);
            }
        }

        return Ok(());
    }

    fn gather_content(&self, content: String) -> String {
        todo!();
    }
}
