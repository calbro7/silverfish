use crate::state::State;
use crate::colours::Colour;
use crate::pieces::Piece;
use crate::bitboards::{get_bit};
use crate::eval::relative_eval;
use crate::moves::{generate_moves, BitMove, move_is_capture, move_is_ep, move_piece, move_to, MoveList, move_to_algebraic};
use std::time::{Duration, Instant};

const MATE_VALUE: isize = 10000;

pub struct Search<'a> {
    state: State,
    depth: u8,
    times: [Option<Duration>; 2],
    search_start: Instant,
    search_duration: Option<Duration>,
    node_counter: usize,
    best: (BitMove, isize),
    killers: [[BitMove; 2]; 64],
    out: Option<&'a mut dyn std::io::Write>
}

impl<'a> Search<'a> {
    pub fn new(state: State) -> Self {
        Self {
            state,
            depth: u8::MAX,
            times: [None; 2],
            search_start: Instant::now(),
            search_duration: None,
            node_counter: 0,
            best: (0, -MATE_VALUE),
            killers: [[0; 2]; 64],
            out: None
        }
    }

    pub fn set_depth(&mut self, depth: u8) {
        self.depth = depth;
    }
    pub fn set_time(&mut self, colour: Colour, time: Option<Duration>) {
        self.times[colour as usize] = time;
    }
    pub fn set_search_duration(&mut self, duration: Option<Duration>) {
        self.search_duration = duration;
    }
    pub fn set_out(&mut self, out: Option<&'a mut dyn std::io::Write>) {
        self.out = out;
    }

    pub fn go(mut self) -> (BitMove, isize) {
        self.search_start = Instant::now();
        if self.search_duration.is_none() {
            if let Some(duration) = self.times[self.state.to_move as usize] {
                self.search_duration = Some(Duration::from_nanos(duration.as_nanos() as u64 / 20));
            }
        }
        
        for depth in 1..=self.depth {
            let mut best = (0, -MATE_VALUE);

            let mut moves = generate_moves(&self.state);
            self.sort_moves(&mut moves, [0, 0]);
    
            while let Some(r#move) = moves.next() {
                let copy = self.state.clone();
                if self.state.make_move(r#move).is_err() {
                    continue;
                }
                let score = -self.negamax(-MATE_VALUE, MATE_VALUE, depth-1, 1);
                if score >= best.1 {
                    best.0 = r#move;
                    best.1 = score;
                }
                self.state = copy;
            }

            if let Some(duration) = self.search_duration {
                if depth > 1 && Instant::now().duration_since(self.search_start) > duration {
                    break;
                }
            }

            self.best = best;
    
            if let Some(out) = &mut self.out {
                writeln!(out, "info depth {} nodes {} bestmove {} cp {}", depth, self.node_counter, move_to_algebraic(best.0), match self.state.to_move {
                    Colour::White => self.best.1,
                    Colour::Black => -self.best.1
                }).unwrap();
            }
        }

        (self.best.0, match self.state.to_move {
            Colour::White => self.best.1,
            Colour::Black => -self.best.1
        })
    }

    fn negamax(&mut self, mut alpha: isize, beta: isize, mut depth: u8, current_ply: u8) -> isize {
        if let Some(duration) = self.search_duration {
            if self.node_counter % 2048 == 0 && Instant::now().duration_since(self.search_start) > duration {
                return alpha;
            }
        }

        if depth == 0 {
            return self.quiescence(alpha, beta, current_ply);
        }

        self.node_counter += 1;

        if self.state.is_in_check(self.state.to_move) {
            depth += 1;
        }

        let mut moves = generate_moves(&self.state);
        self.sort_moves(&mut moves, self.killers[current_ply as usize]);
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
                if !move_is_capture(r#move) {
                    self.killers[current_ply as usize][1] = self.killers[current_ply as usize][0];
                    self.killers[current_ply as usize][0] = r#move;
                }

                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }

        if num_legal_moves == 0 {
            if self.state.is_in_check(self.state.to_move) {
                return -MATE_VALUE + (current_ply as isize);
            }
            else {
                return 0;
            }
        }

        alpha
    }

    fn quiescence(&mut self, mut alpha: isize, beta: isize, current_ply: u8) -> isize {
        if let Some(duration) = self.search_duration {
            if self.node_counter % 2048 == 0 && Instant::now().duration_since(self.search_start) > duration {
                return alpha;
            }
        }

        self.node_counter += 1;

        let standing_pat = relative_eval(&self.state);
        if standing_pat >= beta {
            return beta;
        }
        if standing_pat > alpha {
            alpha = standing_pat;
        }

        let mut moves = generate_moves(&self.state);
        self.sort_moves(&mut moves, self.killers[current_ply as usize]);
        while let Some(r#move) = moves.next() {
            if !move_is_capture(r#move) {
                continue;
            }
            let copy = self.state.clone();
            if self.state.make_move(r#move).is_err() {
                continue;
            }
            let score = -self.quiescence(-beta, -alpha, current_ply+1);
            self.state = copy;
            if score >= beta {
                if !move_is_capture(r#move) {
                    self.killers[current_ply as usize][1] = self.killers[current_ply as usize][0];
                    self.killers[current_ply as usize][0] = r#move;
                }
                
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }

    fn sort_moves(&self, move_list: &mut MoveList, killers: [BitMove; 2]) {
        move_list.moves[0..move_list.length].sort_by(|a,b| self.score_move(*b, killers).cmp(&self.score_move(*a, killers)));
    }

    fn score_move(&self, r#move: BitMove, killers: [BitMove; 2]) -> usize {
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
    
            return 6 * (captured_piece as usize) + (5 - move_piece(r#move) as usize) + 10000;
        }
        else if killers[0] == r#move {
            return 9000;
        }
        else if killers[1] == r#move {
            return 8000;
        }
        
        0
    }
}