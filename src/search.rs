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
    let mut node_counter = 0;
    while !moves.is_empty() {
        let r#move = moves.pop();
        let copy = state.clone();
        if state.make_move(r#move).is_err() {
            continue;
        }
        let score = -1 * negamax(&mut state, 5, -1 * MATE_VALUE, MATE_VALUE, &mut node_counter);
        if score >= best.1 {
            best.0 = r#move;
            best.1 = score;
        }
        *state = copy;
    }

    let absolute_score = match state.to_move {
        Colour::White => best.1,
        Colour::Black => -1 * best.1
    };

    println!("info depth 6 nodes {} cp {}", node_counter, absolute_score);

    (best.0, absolute_score)
}

fn negamax(mut state: &mut State, depth: u8, mut alpha: isize, mut beta: isize, mut node_counter: &mut usize) -> isize {
    *node_counter += 1;

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
        value = max(value, -1*negamax(&mut state, depth-1, -1*beta, -1*alpha, &mut node_counter));
        *state = copy;
        alpha = max(alpha, value);
        if alpha >= beta {
            break;
        }
    }

    if num_legal_moves == 0 {
        let king_sq = get_ls1b(state.pieces[Piece::King as usize] & state.colours[state.to_move as usize]);

        if state.square_attacked(king_sq, !state.to_move) {
            value = -1 * MATE_VALUE;
        }
        else {
            value = 0;
        }
    }
    
    value
}