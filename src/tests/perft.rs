#[cfg(test)]

use crate::perft::perft;
use crate::state::State;

#[test]
fn perft1() {
    let mut state = State::start_pos();
    assert_eq!(perft(&mut state, 5), 4865609);
}

#[test]
fn perft2() {
    let mut state = State::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    assert_eq!(perft(&mut state, 4), 4085603);
}

#[test]
fn perft3() {
    let mut state = State::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
    assert_eq!(perft(&mut state, 6), 11030083);
}

#[test]
fn perft4() {
    let mut state = State::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap();
    assert_eq!(perft(&mut state, 5), 15833292);
}

#[test]
fn perft5() {
    let mut state = State::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ").unwrap();
    assert_eq!(perft(&mut state, 4), 2103487);
}

#[test]
fn perft6() {
    let mut state = State::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ").unwrap();
    assert_eq!(perft(&mut state, 4), 3894594);
}