use crate::state::State;
use crate::moves::{generate_moves, move_from, move_to};
use crate::helpers::sq_to_algebraic;

pub fn perft(state: &mut State, depth: u8, print: bool) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut count = 0u64;
    let mut moves = generate_moves(&state);
    while !moves.is_empty() {
        let copy = state.clone();
        let r#move = moves.pop();
        if state.make_move(r#move).is_err() {
            continue;
        }
        let n = perft(state, depth - 1, false);
        count += n;
        if print {
            println!("{} {} {}", sq_to_algebraic(move_from(r#move)), sq_to_algebraic(move_to(r#move)), n);
        }
        *state = copy;
    }

    count
}