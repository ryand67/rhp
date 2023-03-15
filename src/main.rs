mod parser;
mod util;

use parser::*;

fn main() {
    let parser = match Parser::new() {
        Ok(_) => todo!(),
        Err(e) => eprintln!("Error: {e}"),
    };
}
