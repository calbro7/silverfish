use crate::state::State;
use crate::colours::Colour;
use crate::eval::relative_eval;
use crate::moves::{generate_moves, BitMove, move_is_capture, move_from, move_to, move_to_algebraic};
use std::cmp::max;

pub fn search(mut state: &mut State) -> (BitMove, isize) {
    let mut best: (BitMove, isize) = (0, -10000);
    let mut moves = generate_moves(&state);
    while !moves.is_empty() {
        let r#move = moves.pop();
        let copy = state.clone();
        if state.make_move(r#move).is_err() {
            continue;
        }
        let score = -1 * negamax(&mut state, 3);
        if score >= best.1 {
            best.0 = r#move;
            best.1 = score;
        }
        *state = copy;
    }

    match state.to_move {
        Colour::White => best,
        Colour::Black => (best.0, -1 * best.1)
    }
}

fn negamax(mut state: &mut State, depth: u8) -> isize {
    if depth == 0 {
        return relative_eval(state);
    }

    let mut value = -10000isize;
    let mut moves = generate_moves(&state);
    while !moves.is_empty() {
        let r#move = moves.pop();
        let copy = state.clone();
        if state.make_move(r#move).is_err() {
            continue;
        }
        value = max(value, -1*negamax(&mut state, depth-1));
        *state = copy;
    }

    value
}