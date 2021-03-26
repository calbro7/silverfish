use crate::state::State;
use crate::search::Search;
use crate::moves::move_to_algebraic;

#[test]
fn mates_in_1() {
    let state = State::from_fen("r1bq1rk1/pp1nbppp/2n1p3/3pP2Q/2pP4/2P4N/PPBN1PPP/R1B1K2R w KQ - 6 10").unwrap();
    let mut search = Search::new(state, &None);
    search.set_depth(6);
    let result = search.go();
    assert_eq!(move_to_algebraic(result.0), "h5h7");
}

#[test]
fn mates_in_3() {
    let state = State::from_fen("r5k1/2p2ppp/2q5/7b/2r5/4R1PP/2P1QP2/4R1K1 w - - 0 32").unwrap();
    let mut search = Search::new(state, &None);
    search.set_depth(6);
    let result = search.go();
    assert_eq!(move_to_algebraic(result.0), "e3e8");
}

#[test]
fn defends_mate_in_3() {
    let state = State::from_fen("r5k1/2p2ppp/2q5/8/2r5/4R1PP/2P1QP2/4R1K1 b - - 0 1").unwrap();
    let mut search = Search::new(state, &None);
    search.set_depth(6);
    let result = search.go();
    assert!(vec!["h7h6".to_string(), "g7g6".to_string(), "g8f8".to_string(), "a8f8".to_string()].contains(&move_to_algebraic(result.0)));
}

#[test]
fn tactic_to_win_knight() {
    let state = State::from_fen("r1n4k/P1rq1pb1/1Qp1p2p/3pP1p1/3P4/5NP1/R2B1P1P/R5K1 w - - 6 32").unwrap();
    let mut search = Search::new(state, &None);
    search.set_depth(6);
    let result = search.go();
    assert_eq!(move_to_algebraic(result.0), "b6b8");
}