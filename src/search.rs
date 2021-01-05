use crate::state::State;
use crate::colours::Colour;
use crate::pieces::Piece;
use crate::bitboards::{get_ls1b, get_bit};
use crate::eval::relative_eval;
use crate::moves::{generate_moves, BitMove, move_is_capture, move_is_ep, move_piece, move_to, MoveList};

const MATE_VALUE: isize = 10000;

pub struct Search<'a> {
    state: State,
    depth: u8,
    node_counter: usize,
    best: (BitMove, isize),
    out: Option<&'a mut dyn std::io::Write>
}

impl<'a> Search<'a> {
    pub fn new(state: State) -> Self {
        Self {
            state,
            depth: 5,
            node_counter: 0,
            best: (0, -MATE_VALUE),
            out: None
        }
    }

    pub fn set_depth(&mut self, depth: u8) {
        self.depth = depth;
    }
    pub fn set_out(&mut self, out: Option<&'a mut dyn std::io::Write>) {
        self.out = out;
    }

    pub fn go(&mut self) -> (BitMove, isize) {
        let mut moves = generate_moves(&self.state);
        self.sort_moves(&mut moves);

        while let Some(r#move) = moves.next() {
            let copy = self.state.clone();
            if self.state.make_move(r#move).is_err() {
                continue;
            }
            let score = -self.negamax(-MATE_VALUE, MATE_VALUE, self.depth-1, 1);
            if score >= self.best.1 {
                self.best.0 = r#move;
                self.best.1 = score;
            }
            self.state = copy;
        }

        let absolute_score = match self.state.to_move {
            Colour::White => self.best.1,
            Colour::Black => -self.best.1
        };

        if let Some(out) = &mut self.out {
            writeln!(out, "info depth {} nodes {} cp {}", self.depth, self.node_counter, absolute_score).unwrap();
        }

        (self.best.0, absolute_score)
    }

    fn negamax(&mut self, mut alpha: isize, beta: isize, depth: u8, current_ply: u8) -> isize {
        if depth == 0 {
            return self.quiescence(alpha, beta);
        }

        self.node_counter += 1;

        let mut moves = generate_moves(&self.state);
        self.sort_moves(&mut moves);
        let mut num_legal_moves = 0;
        while let Some(r#move) = moves.next() {
            let copy = self.state.clone();
            if self.state.make_move(r#move).is_err() {
                continue;
            }
            num_legal_moves += 1;
            let score = -self.negamax(-beta, -alpha, depth-1, current_ply+1);
            self.state = copy;
            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }

        if num_legal_moves == 0 {
            let king_sq = get_ls1b(self.state.pieces[Piece::King as usize] & self.state.colours[self.state.to_move as usize]).unwrap();

            if self.state.square_attacked(king_sq, !self.state.to_move) {
                return -MATE_VALUE + (current_ply as isize);
            }
            else {
                return 0;
            }
        }

        alpha
    }

    fn quiescence(&mut self, mut alpha: isize, beta: isize) -> isize {
        self.node_counter += 1;

        let standing_pat = relative_eval(&self.state);
        if standing_pat >= beta {
            return beta;
        }
        if standing_pat > alpha {
            alpha = standing_pat;
        }

        let mut moves = generate_moves(&self.state);
        self.sort_moves(&mut moves);
        while let Some(r#move) = moves.next() {
            if !move_is_capture(r#move) {
                continue;
            }
            let copy = self.state.clone();
            if self.state.make_move(r#move).is_err() {
                continue;
            }
            let score = -self.quiescence(-beta, -alpha);
            self.state = copy;
            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }

    fn sort_moves(&self, move_list: &mut MoveList) {
        move_list.moves[0..move_list.length].sort_by(|a,b| self.score_move(*b).cmp(&self.score_move(*a)));
    }

    fn score_move(&self, r#move: BitMove) -> usize {
        if move_is_capture(r#move) {
            let mut captured_piece = Piece::Pawn;
            if !move_is_ep(r#move) {
                // todo - is it quicker to add pawns to the start of this list? (extra iteration, but most captures will be of pawns)
                for piece in &[Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen, Piece::King] {
                    if get_bit(self.state.pieces[*piece as usize], move_to(r#move)) {
                        captured_piece = *piece;
                        break;
                    }
                }
            }
    
            return 6 * (captured_piece as usize) + (5 - move_piece(r#move) as usize);
        }
        
        0
    }
}