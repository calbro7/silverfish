use crate::state::State;
use crate::colours::Colour;
use crate::pieces::Piece;
use crate::bitboards::get_ls1b;
use crate::eval::relative_eval;
use crate::moves::{generate_moves, BitMove, move_is_capture, move_from, move_to, move_to_algebraic};
use std::cmp::max;

const MATE_VALUE: isize = 10000;

pub fn search(mut state: &mut State) -> (BitMove, isize) {
    let mut best: (BitMove, isize) = (0, -1 * MATE_VALUE);
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

    let mut value = -1 * MATE_VALUE;
    let mut moves = generate_moves(&state);
    let mut num_legal_moves = 0;
    while !moves.is_empty() {
        let r#move = moves.pop();
        let copy = state.clone();
        if state.make_move(r#move).is_err() {
            continue;
        }
        num_legal_moves += 1;
        value = max(value, -1*negamax(&mut state, depth-1));
        *state = copy;
    }

    if num_legal_moves == 0 {
        let king_sq = get_ls1b(state.pieces[Piece::King as usize] & state.colours[state.to_move as usize]);

        return if state.square_attacked(king_sq, !state.to_move) { -1 * MATE_VALUE } else { 0 };
    }
    
    value
}