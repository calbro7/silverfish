mod helpers;
mod bitboards;
mod colours;
mod attacks;
mod castling;
mod state;
mod pieces;
mod moves;
mod uci;
mod errors;
mod perft;
mod eval;
mod search;
mod tests;
mod zobrist;
use text_io::read;
use std::sync::{Arc, Mutex};

fn main() {
    let mut uci = uci::UciHandler::new(Arc::new(Mutex::new(std::io::stdout())));

    loop {
        let input: String = read!("{}\n");
        uci.command(&input);
    }
}