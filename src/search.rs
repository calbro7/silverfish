use crate::state::State;
use crate::colours::Colour;
use crate::pieces::Piece;
use crate::bitboards::get_ls1b;
use crate::eval::relative_eval;
use crate::moves::{generate_moves, BitMove};

const MATE_VALUE: isize = 10000;

pub fn search<W: std::io::Write>(mut state: &mut State, depth: Option<u8>, out: Option<&mut W>) -> (BitMove, isize) {
    let default_depth = 5u8;
    let mut best: (BitMove, isize) = (0, -MATE_VALUE);
    let mut moves = generate_moves(&state);
    let mut node_counter = 0;
    while !moves.is_empty() {
        let r#move = moves.pop();
        let copy = state.clone();
        if state.make_move(r#move).is_err() {
            continue;
        }
        let score = -negamax(&mut state, depth.unwrap_or(default_depth), -MATE_VALUE, MATE_VALUE, 0, &mut node_counter);
        if score >= best.1 {
            best.0 = r#move;
            best.1 = score;
        }
        *state = copy;
    }

    let absolute_score = match state.to_move {
        Colour::White => best.1,
        Colour::Black => -best.1
    };

    if out.is_some() {
        writeln!(out.unwrap(), "info depth {} nodes {} cp {}", depth.unwrap_or(default_depth), node_counter, absolute_score).unwrap();
    }

    (best.0, absolute_score)
}

fn negamax(mut state: &mut State, depth: u8, mut alpha: isize, beta: isize, ply: u8, mut node_counter: &mut usize) -> isize {
    *node_counter += 1;

    if depth == 0 {
        return relative_eval(state);
    }

    let mut moves = generate_moves(&state);
    let mut num_legal_moves = 0;
    while !moves.is_empty() {
        let r#move = moves.pop();
        let copy = state.clone();
        if state.make_move(r#move).is_err() {
            continue;
        }
        num_legal_moves += 1;
        let score = -negamax(&mut state, depth-1, -beta, -alpha, ply+1, &mut node_counter);
        *state = copy;
        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }

    if num_legal_moves == 0 {
        let king_sq = get_ls1b(state.pieces[Piece::King as usize] & state.colours[state.to_move as usize]);

        if state.square_attacked(king_sq, !state.to_move) {
            (-MATE_VALUE) + (ply as isize)
        }
        else {
            0
        }
    }
    else {
        alpha
    }
}