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
use text_io::read;

fn main() {
    let mut out = std::io::stdout();
    let mut uci = uci::UciHandler::new(&mut out);

    loop {
        let input: String = read!("{}\n");
        uci.command(&input);
    }
}