mod parser;
mod util;

use parser::*;

fn main() {
    match parse() {
        Ok(_) => todo!(),
        Err(e) => eprintln!("Error: {e}"),
    }
}
