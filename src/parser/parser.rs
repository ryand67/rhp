use crate::{parser::Lexer, util::*};

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new() -> Result<Self, String> {
        let reader = get_reader()?;

        let lexer = Lexer::new(reader);

        return Ok(Self { lexer });
    }

    pub fn parse(&mut self) {
        self.lexer.lex_all_files();
    }
}
