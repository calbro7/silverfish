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
use std::env;
use getopts::Options;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("t", "syzygy", "Tablebase directory", "Tablebase");
    let tb_directory = match opts.parse(&args[1..]) {
        Ok(matches) => matches.opt_str("t"),
        Err(_) => panic!()
    };

    let mut uci = uci::UciHandler::new(tb_directory, Arc::new(Mutex::new(std::io::stdout())));

    loop {
        let input: String = read!("{}\n");
        uci.command(&input);
    }
}