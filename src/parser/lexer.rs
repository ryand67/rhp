use crate::util::open_file;
use std::{io::Read, path::PathBuf};

type CharPos = (char, usize);

pub struct Lexer {
    files: Vec<PathBuf>,
    curr: CharPos,
    content: String,
}

impl Lexer {
    pub fn new(files: Vec<PathBuf>) -> Self {
        return Self {
            files,
            curr: ('\0', 0),
            content: String::default(),
        };
    }

    pub fn lex_all_files(&mut self) {
        for file in self.files.clone() {
            match self.lex_file(&file) {
                Ok(_) => (),
                Err(e) => println!("Error opening file: {:?}", e),
            }
        }
    }

    fn set_content(&mut self, content: String) {
        self.content = content;
    }

    fn lex_file(&mut self, path: &PathBuf) -> Result<(), String> {
        let mut reader = match open_file(path) {
            Ok(r) => r,
            Err(e) => {
                return Err(format!("{}", e.to_string()));
            }
        };

        let mut buf = String::new();

        reader.read_to_string(&mut buf).unwrap();
        self.set_content(buf);

        self.lex_content();

        return Ok(());
    }

    fn lex_content(&mut self) {
        let chars: Vec<char> = self.content.chars().collect();
        self.curr = (chars[0], 0);

        chars.into_iter().enumerate().for_each(|(i, c)| {
            todo!();
        });
    }
}
