use crate::uci::UciHandler;
use crate::state::State;
use regex::Regex;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

#[test]
fn isready() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let mut uci = UciHandler::new(output.clone());

    uci.command("isready");

    assert_eq!(String::from_utf8(output.lock().unwrap().to_vec()).unwrap(), "readyok\n");
}

#[test]
fn ucinewgame() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let mut uci = UciHandler::new(output.clone());

    uci.command("ucinewgame");
    uci.command("print");

    assert_eq!(String::from_utf8(output.lock().unwrap().to_vec()).unwrap(), format!("{}\n", State::start_pos()));
}

#[test]
fn position_from_fen() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let mut uci = UciHandler::new(output.clone());

    uci.command("position fen r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 2 3");
    uci.command("print");

    assert_eq!(String::from_utf8(output.lock().unwrap().to_vec()).unwrap(), format!("{}\n", State::from_fen("r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 2 3").unwrap()));
}

#[test]
fn position_from_fen_with_moves() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let mut uci = UciHandler::new(output.clone());

    uci.command("position fen r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 2 3 moves f1b5");
    uci.command("print");
    
    assert_eq!(String::from_utf8(output.lock().unwrap().to_vec()).unwrap(), format!("{}\n", State::from_fen("r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 3 3").unwrap()));
}

#[test]
fn position_startpos() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let mut uci = UciHandler::new(output.clone());

    // By default the state will be the starting position, so to test the startpos command we need to set it to something else first
    uci.command("position fen r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 2 3 moves f1b5");
    uci.command("position startpos");
    uci.command("print");

    assert_eq!(String::from_utf8(output.lock().unwrap().to_vec()).unwrap(), format!("{}\n", State::start_pos()));
}

#[test]
fn position_startpos_with_moves() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let mut uci = UciHandler::new(output.clone());

    uci.command("position startpos moves e2e4 e7e5 g1f3 b8c6 f1b5");
    uci.command("print");

    assert_eq!(String::from_utf8(output.lock().unwrap().to_vec()).unwrap(), format!("{}\n", State::from_fen("r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 3 3").unwrap()));
}

#[test]
fn position_with_invalid_moves() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let mut uci = UciHandler::new(output.clone());

    uci.command("position startpos moves e2e4 e7e5 not valid");
    uci.command("print");

    assert_eq!(String::from_utf8(output.lock().unwrap().to_vec()).unwrap(), format!("{}\n", State::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2").unwrap()));
}

#[test]
fn go_with_movetime() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let mut uci = UciHandler::new(output.clone());

    // The best move will of course depend on the searching and evaluation, but we choose a position for which there is objectively only one standout move (capturing the hanging queen)
    uci.command("position fen rnb1kbnr/pppp1ppp/8/4p1qQ/4P3/8/PPPP1PPP/RNB1KBNR w KQkq - 2 3");
    uci.command("go movetime 3000");

    // The searching happens in its own thread, so we must manually wait an appropriate length of time for it to finish
    // (technically there's no guarantee this wait will be long enough, but it should be)
    sleep(Duration::from_secs(5));

    assert!(String::from_utf8(output.lock().unwrap().to_vec()).unwrap().contains("bestmove h5g5"));
}

#[test]
fn go_with_wtime() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let mut uci = UciHandler::new(output.clone());

    // The best move will of course depend on the searching and evaluation, but we choose a position for which there is objectively only one standout move (capturing the hanging queen)
    uci.command("position fen rnb1kbnr/pppp1ppp/8/4p1qQ/4P3/8/PPPP1PPP/RNB1KBNR w KQkq - 2 3");
    uci.command("go wtime 10000");

    // The searching happens in its own thread, so we must manually wait an appropriate length of time for it to finish
    // (technically there's no guarantee this wait will be long enough, but it should be)
    sleep(Duration::from_secs(5));

    assert!(String::from_utf8(output.lock().unwrap().to_vec()).unwrap().contains("bestmove h5g5"));
}

#[test]
fn go_with_btime() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let mut uci = UciHandler::new(output.clone());

    // The best move will of course depend on the searching and evaluation, but we choose a position for which there is objectively only one standout move (capturing the hanging queen)
    uci.command("position fen rnb1kbnr/pppp1ppp/8/4p1qQ/4P3/7P/PPPP1PP1/RNB1KBNR b KQkq - 0 3");
    uci.command("go btime 10000");

    // The searching happens in its own thread, so we must manually wait an appropriate length of time for it to finish
    // (technically there's no guarantee this wait will be long enough, but it should be)
    sleep(Duration::from_secs(5));

    assert!(String::from_utf8(output.lock().unwrap().to_vec()).unwrap().contains("bestmove g5h5"));
}

#[test]
fn go_with_depth() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let mut uci = UciHandler::new(output.clone());

    // The best move will of course depend on the searching and evaluation, but we choose a position for which there is objectively only one standout move (capturing the hanging queen)
    uci.command("position fen rnb1kbnr/pppp1ppp/8/4p1qQ/4P3/8/PPPP1PPP/RNB1KBNR w KQkq - 2 3");
    uci.command("go depth 4");

    // The searching happens in its own thread, so we must manually wait an appropriate length of time for it to finish
    // (technically there's no guarantee this wait will be long enough, but it should be)
    sleep(Duration::from_secs(5));

    let output_str = String::from_utf8(output.lock().unwrap().to_vec()).unwrap();
    assert!(output_str.contains("depth 4"));
    assert!(!output_str.contains("depth 5"));
    assert!(output_str.contains("bestmove h5g5"));
}

#[test]
fn uci() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let mut uci = UciHandler::new(output.clone());

    uci.command("uci");

    assert_eq!(String::from_utf8(output.lock().unwrap().to_vec()).unwrap(), "id name silverfish\nuciok\n");
}

#[test]
fn perft() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let mut uci = UciHandler::new(output.clone());

    uci.command("position fen k4b1K/4P3/8/8/8/8/8/8 w - - 0 1");
    uci.command("perft 5");

    let output_str = String::from_utf8(output.lock().unwrap().to_vec()).unwrap();
    assert!(Regex::new(r"^e7e8q: 7721\ne7e8r: 4301\ne7e8b: 10851\ne7e8n: 5349\ne7f8q: 4562\ne7f8r: 2506\ne7f8b: 1994\ne7f8n: 1022\nh8h7: 4745\nh8g8: 4616\nTotal: 47667 (.*)\n$").unwrap().is_match(&output_str));
}

#[test]
fn eval() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let mut uci = UciHandler::new(output.clone());

    uci.command("eval");

    assert_eq!(String::from_utf8(output.lock().unwrap().to_vec()).unwrap(), "0\n");
}

#[test]
fn print() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let mut uci = UciHandler::new(output.clone());

    uci.command("print");

    assert_eq!(String::from_utf8(output.lock().unwrap().to_vec()).unwrap(), format!("{}\n", State::start_pos()));
}