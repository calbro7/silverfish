use crate::state::State;
use crate::eval::eval;
use more_asserts::assert_lt;

#[test]
fn test_material_differences() {
    // An array of positions in order of how heavily they favour white, hence the evals should form a decreasing sequence
    let positions = [
        // White up a queen
        State::from_fen("r1b1r1k1/ppp1bppp/2np1n2/1B2p3/4P3/3P1N1P/PPP2PP1/RNBQR1K1 w - - 1 8").unwrap(),
        // White up a knight+bishop
        State::from_fen("r2qr1k1/ppp1bppp/2np4/1B2p3/4P3/3P1N1P/PPP2PP1/RNBQR1K1 w - - 1 8").unwrap(),
        // White up a rook
        State::from_fen("2bqr1k1/ppp1bppp/2np1n2/1B2p3/4P3/3P1N1P/PPP2PP1/RNBQR1K1 w - - 1 8").unwrap(),
        // White up a bishop
        State::from_fen("r2qr1k1/ppp1bppp/2np1n2/1B2p3/4P3/3P1N1P/PPP2PP1/RNBQR1K1 w - - 1 8").unwrap(),
        // White up a pawn
        State::from_fen("r1bqr1k1/pp2bppp/2np1n2/1B2p3/4P3/3P1N1P/PPP2PP1/RNBQR1K1 w - - 1 8").unwrap(),
        // Even
        State::from_fen("r1bqr1k1/ppp1bppp/2np1n2/1B2p3/4P3/3P1N1P/PPP2PP1/RNBQR1K1 w - - 1 8").unwrap(),
        // Black up a pawn
        State::from_fen("r1bqr1k1/ppp1bppp/2np1n2/1B2p3/4P3/3P1N1P/PP3PP1/RNBQR1K1 w - - 1 8").unwrap(),
        // Black up a bishop
        State::from_fen("r1bqr1k1/ppp1bppp/2np1n2/1B2p3/4P3/3P1N1P/PPP2PP1/RN1QR1K1 w - - 1 8").unwrap(),
        // Black up a rook
        State::from_fen("r1bqr1k1/ppp1bppp/2np1n2/1B2p3/4P3/3P1N1P/PPP2PP1/1NBQR1K1 w - - 1 8").unwrap(),
        // Black up a knight+bishop
        State::from_fen("r1bqr1k1/ppp1bppp/2np1n2/1B2p3/4P3/3P1N1P/PPP2PP1/R2QR1K1 w - - 1 8").unwrap(),
        // Black up a queen
        State::from_fen("r1bqr1k1/ppp1bppp/2np1n2/1B2p3/4P3/3P1N1P/PPP2PP1/RNB1R1K1 w - - 1 8").unwrap(),
    ];

    let mut previous_evaluation = isize::MAX;
    for position in &positions {
        let eval = eval(position);
        assert_lt!(eval, previous_evaluation);
        previous_evaluation = eval;
    }
}