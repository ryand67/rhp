mod parser;
mod util;

use parser::*;
use std::process::exit;

fn main() {
    let mut parser: Parser = match Parser::new() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    };

    parser.parse();
}
