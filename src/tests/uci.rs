#[cfg(test)]

use crate::uci::UciHandler;
use crate::state::State;

#[test]
fn isready() {
    let mut output = Vec::new();
    let mut uci = UciHandler::new(&mut output);
    uci.command("isready");
    assert_eq!(String::from_utf8(output).unwrap(), "readyok\n");
}

#[test]
fn ucinewgame() {
    let mut output = Vec::new();
    let mut uci = UciHandler::new(&mut output);
    uci.command("ucinewgame");
    uci.command("print");
    assert_eq!(String::from_utf8(output).unwrap(), format!("{}\n", State::start_pos()));
}

#[test]
fn position_from_fen() {
    let mut output = Vec::new();
    let mut uci = UciHandler::new(&mut output);
    uci.command("position fen r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 2 3");
    uci.command("print");
    assert_eq!(String::from_utf8(output).unwrap(), format!("{}\n", State::from_fen("r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 2 3").unwrap()));
}

#[test]
fn position_from_fen_with_moves() {
    let mut output = Vec::new();
    let mut uci = UciHandler::new(&mut output);
    uci.command("position fen r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 2 3 moves f1b5");
    uci.command("print");
    assert_eq!(String::from_utf8(output).unwrap(), format!("{}\n", State::from_fen("r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 3 3").unwrap()));
}

#[test]
fn position_startpos() {
    let mut output = Vec::new();
    let mut uci = UciHandler::new(&mut output);
    // By default the state will be the starting position, so to test the startpos command we need to set it to something else first
    uci.command("position fen r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 2 3 moves f1b5");
    uci.command("position startpos");
    uci.command("print");
    assert_eq!(String::from_utf8(output).unwrap(), format!("{}\n", State::start_pos()));
}

#[test]
fn position_startpos_with_moves() {
    let mut output = Vec::new();
    let mut uci = UciHandler::new(&mut output);
    uci.command("position startpos moves e2e4 e7e5 g1f3 b8c6 f1b5");
    uci.command("print");
    assert_eq!(String::from_utf8(output).unwrap(), format!("{}\n", State::from_fen("r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 3 3").unwrap()));
}

#[test]
fn go() {
    let mut output = Vec::new();
    let mut uci = UciHandler::new(&mut output);
    // The best move will of course depend on the searching and evaluation, but we choose a position for which there is objectively only one standout move (capturing the hanging queen)
    uci.command("position fen rnb1kbnr/pppp1ppp/8/4p1qQ/4P3/8/PPPP1PPP/RNB1KBNR w KQkq - 2 3");
    uci.command("go");
    assert!(String::from_utf8(output).unwrap().ends_with("bestmove h5g5\n"));
}

#[test]
fn uci() {
    let mut output = Vec::new();
    let mut uci = UciHandler::new(&mut output);
    uci.command("uci");
    assert_eq!(String::from_utf8(output).unwrap(), "id name silverfish\nuciok\n");
}

#[test]
fn perft() {
    let mut output = Vec::new();
    let mut uci = UciHandler::new(&mut output);
    uci.command("position fen k4b1K/4P3/8/8/8/8/8/8 w - - 0 1");
    uci.command("perft 5");
    assert_eq!(String::from_utf8(output).unwrap(), "e7e8q: 7721\ne7e8r: 4301\ne7e8b: 10851\ne7e8n: 5349\ne7f8q: 4562\ne7f8r: 2506\ne7f8b: 1994\ne7f8n: 1022\nh8h7: 4745\nh8g8: 4616\nTotal: 47667\n");
}

#[test]
fn eval() {
    let mut output = Vec::new();
    let mut uci = UciHandler::new(&mut output);
    uci.command("eval");
    assert_eq!(String::from_utf8(output).unwrap(), "0\n");
}

#[test]
fn print() {
    let mut output = Vec::new();
    let mut uci = UciHandler::new(&mut output);
    uci.command("print");
    assert_eq!(String::from_utf8(output).unwrap(), format!("{}\n", State::start_pos()));
}