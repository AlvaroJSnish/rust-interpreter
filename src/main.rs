use std::io;

pub mod lexer;
pub mod repl;
mod token;

use crate::repl::start;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    start(&mut stdin, &mut stdout);
}
