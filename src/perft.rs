use crate::state::State;
use crate::moves::generate_moves;

pub fn perft(state: &mut State, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut count = 0u64;
    let mut moves = generate_moves(&state);
    while let Some(r#move) = moves.next() {
        let copy = state.clone();
        if state.make_move(r#move).is_err() {
            continue;
        }
        count += perft(state, depth - 1);
        *state = copy;
    }

    count
}