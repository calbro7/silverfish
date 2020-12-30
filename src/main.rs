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
mod tests;
use text_io::read;

fn main() {
    let mut uci = uci::UciHandler::new(std::io::stdout());

    loop {
        let input: String = read!("{}\n");
        uci.command(&input);
    }
}