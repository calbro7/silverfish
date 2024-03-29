use crate::state::State;
use crate::colours::Colour;
use crate::pieces::Piece;
use crate::bitboards::{get_bit, count_bits};
use crate::eval::relative_eval;
use crate::moves::{generate_moves, BitMove, move_is_capture, move_is_ep, move_piece, move_from, move_to, MoveList, move_to_algebraic, encode_move};
use crate::book::BOOK;
use rand::{thread_rng, Rng};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, Instant};
use std::sync::mpsc::{Sender, Receiver};
use shakmaty::{CastlingMode, Chess, Role};
use shakmaty::fen::Fen;
use shakmaty_syzygy::{Tablebase, Dtz};

const MATE_VALUE: isize = 10000;
const MAX_PLY: usize = 64;

pub enum Message {
    Info(usize, usize, usize, usize, Duration, BitMove, isize, Line), // depth, nodes, tt hits, tb_hits, duration, bestmove, eval, pv
    Done,
    Stop
}

#[derive(Copy, Clone)]
pub struct Line {
    pub length: usize,
    pub moves: [BitMove; MAX_PLY]
}

impl Line {
    pub fn new() -> Self {
        Self {
            length: 0,
            moves: [0; 64]
        }
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for i in 0..self.length {
            output.push_str(&format!("{} ", move_to_algebraic(self.moves[i])));
        }
        write!(f, "{}", output)
    }
}

struct TtEntry {
    score: isize,
    depth: usize,
    flag: i8
}

pub struct Search {
    state: State,
    depth: usize,
    depth_searched: usize,
    times: [Option<Duration>; 2],
    search_start: Instant,
    search_duration: Option<Duration>,
    search_active: bool,
    node_counter: usize,
    best: (BitMove, isize),
    killers: [[BitMove; 2]; MAX_PLY],
    history: [[[usize; MAX_PLY]; MAX_PLY]; 2],
    tt_table: HashMap<u64, TtEntry>,
    tt_hits: usize,
    tablebase: Option<Tablebase<Chess>>,
    tb_hits: usize,
    previous_pv: Line,
    channels: Option<(Sender<Message>, Receiver<Message>)>
}

impl Search {
    pub fn new(state: State, tb_directory: &Option<String>) -> Self {
        let tablebase = match tb_directory {
            Some(dir) => {
                let mut tb = Tablebase::new();
                tb.add_directory(dir).unwrap();
                Some(tb)
            },
            None => None
        };

        Self {
            state,
            depth: usize::MAX,
            depth_searched: 0,
            times: [None; 2],
            search_start: Instant::now(),
            search_duration: None,
            search_active: false,
            node_counter: 0,
            best: (0, -MATE_VALUE),
            killers: [[0; 2]; MAX_PLY],
            history: [[[0; MAX_PLY]; MAX_PLY]; 2],
            tt_table: HashMap::new(),
            tt_hits: 0,
            tablebase: tablebase,
            tb_hits: 0,
            previous_pv: Line::new(),
            channels: None
        }
    }

    pub fn set_depth(&mut self, depth: usize) {
        self.depth = depth;
    }
    pub fn set_time(&mut self, colour: Colour, time: Option<Duration>) {
        self.times[colour as usize] = time;
    }
    pub fn set_search_duration(&mut self, duration: Option<Duration>) {
        self.search_duration = duration;
    }
    pub fn set_channels(&mut self, channels: Option<(Sender<Message>, Receiver<Message>)>) {
        self.channels = channels;
    }

    pub fn go(mut self) -> (BitMove, isize) {
        if self.state.fullmove_number <= 6 {
            if let Some(result) = BOOK.get(&self.state.hash) {
                let mut rng = thread_rng();
                self.best = (*rng.choose(&result).unwrap(), 0);
            }
        }

        if self.best.0 == 0 {
            self.search_start = Instant::now();
            if self.search_duration.is_none() {
                if let Some(duration) = self.times[self.state.to_move as usize] {
                    self.search_duration = Some(duration / 20);
                }
            }
            self.search_active = true;
            
            for depth in 1..=self.depth {
                let mut pv = Line::new();
                self.negamax(-MATE_VALUE, MATE_VALUE, depth, 0, &mut pv, true);
                self.previous_pv = pv;
    
                if !self.search_active && depth > 1 {
                    break;
                }
    
                self.depth_searched = depth;
    
                if let Some(channels) = &mut self.channels {
                    channels.0.send(Message::Info(depth, self.node_counter, self.tt_hits, self.tb_hits, Instant::now().duration_since(self.search_start), self.best.0, self.best.1, self.previous_pv)).unwrap();
                }
    
                // Slightly hacky way of detecting if we've found mate, because then we don't need to search at any higher depths
                if self.best.1 >= MATE_VALUE - 100 {
                    break;
                }
            }
        }
        
        if let Some(channels) = self.channels {
            channels.0.send(Message::Done).unwrap();
        }
        (self.best.0, match self.state.to_move {
            Colour::White => self.best.1,
            Colour::Black => -self.best.1
        })
    }

    fn negamax(&mut self, mut alpha: isize, mut beta: isize, mut depth: usize, current_ply: usize, mut pline: &mut Line, mut in_pv: bool) -> isize {
        if self.depth_searched > 1 && self.node_counter % 2048 == 0 {
            if let Some(duration) = self.search_duration {
                if Instant::now().duration_since(self.search_start) > duration {
                    self.search_active = false;
                }
            }

            if let Some(channels) = &self.channels {
                match channels.1.try_recv() {
                    Ok(Message::Stop) => {
                        self.search_active = false;
                    },
                    _ => {}
                }
            }

            if !self.search_active {
                return alpha;
            }
        }
        
        if current_ply >= MAX_PLY {
            return relative_eval(&self.state);
        }

        if self.state.is_in_check(self.state.to_move) {
            depth += 1;
        }

        let original_alpha = alpha;

        if let Some(tt_entry) = self.tt_table.get(&self.state.hash) {
            if tt_entry.depth >= depth {
                self.tt_hits += 1;

                if tt_entry.flag == 0 {
                    return tt_entry.score;
                }
                if tt_entry.flag == -1 {
                    alpha = max(alpha, tt_entry.score);
                }
                if tt_entry.flag == 1 {
                    beta = min(beta, tt_entry.score);
                }

                if alpha >= beta {
                    return tt_entry.score;
                }
            }
        }

        if let Some(tablebase) = &self.tablebase {
            if count_bits(self.state.occupancy) <= 5 {
                let pos: Chess = self.state.to_fen().parse::<Fen>().unwrap().position(CastlingMode::Standard).unwrap();
                let tb_result = tablebase.best_move(&pos).unwrap();
    
                let (r#move, score) = match tb_result {
                    Some(tb_result) => (
                        encode_move(
                            tb_result.0.from().unwrap() as usize,
                            tb_result.0.to() as usize,
                            Piece::Pawn,
                            match tb_result.0.promotion() {
                                Some(p) => Some(match p {
                                    Role::Pawn => Piece::Pawn,
                                    Role::Knight => Piece::Knight,
                                    Role::Bishop => Piece::Bishop,
                                    Role::Rook => Piece::Rook,
                                    Role::Queen => Piece::Queen,
                                    Role::King => Piece::King
                                }),
                                None => None
                            },
                            false,
                            false,
                            false,
                            false
                        ),
                        match tb_result.1 {
                            Dtz(x) if x > 0 => -MATE_VALUE + current_ply as isize + x as isize,
                            Dtz(x) if x < 0 => MATE_VALUE - current_ply as isize + x as isize,
                            _ => 0
                        }
                    ),
                    None => if self.state.is_in_check(self.state.to_move) { (0, -MATE_VALUE + current_ply as isize) } else { (0, 0) }
                };

                self.tb_hits += 1;
    
                self.tt_table.insert(self.state.hash, TtEntry {
                    score,
                    depth: usize::MAX,
                    flag: 0
                });
    
                if current_ply == 0 {
                    self.best = (r#move, score);
                    self.search_active = false;
                }
    
                return score;
            }
        }

        if depth == 0 {
            pline.length = 0;

            return self.quiescence(alpha, beta, current_ply);
        }

        self.node_counter += 1;

        if current_ply > 0 && current_ply % 2 == 0 && self.state.is_repetition() {
            return 0;
        }

        let mut line = Line::new();

        let mut moves = generate_moves(&self.state);
        self.sort_moves(&mut moves, current_ply, in_pv);
        let mut num_legal_moves = 0;
        let mut bestmove: BitMove = 0;
        while let Some(r#move) = moves.next() {
            let copy = self.state.clone();
            if self.state.make_move(r#move).is_err() {
                continue;
            }
            num_legal_moves += 1;
            let score = if num_legal_moves == 1 {
                -self.negamax(-beta, -alpha, depth-1, current_ply+1, &mut line, in_pv)
            }
            else {
                let null_window_score = -self.negamax(-alpha-1, -alpha, depth-1, current_ply+1, &mut line, in_pv);
                if alpha < null_window_score && null_window_score < beta {
                    -self.negamax(-beta, -null_window_score, depth-1, current_ply+1, &mut line, in_pv)
                }
                else {
                    null_window_score
                }
            };
            self.state = copy;
            in_pv = false;
            if score >= beta {
                if !move_is_capture(r#move) {
                    self.killers[current_ply][1] = self.killers[current_ply][0];
                    self.killers[current_ply][0] = r#move;
                }

                self.tt_table.insert(self.state.hash, TtEntry {
                    score: beta,
                    depth,
                    flag: -1
                });

                return beta;
            }
            if score > alpha {
                if !move_is_capture(r#move) {
                    self.history[self.state.to_move as usize][move_from(r#move)][move_to(r#move)] += depth;
                }

                pline.moves[0] = r#move;
                for i in 0..line.length {
                    pline.moves[i+1] = line.moves[i];
                }
                pline.length = line.length + 1;

                bestmove = r#move;

                alpha = score;
            }
        }

        if num_legal_moves == 0 {
            if self.state.is_in_check(self.state.to_move) {
                alpha = -MATE_VALUE + (current_ply as isize);
            }
            else {
                alpha = 0;
            }
        }

        if self.search_active && bestmove != 0 && current_ply == 0 {
            self.best = (bestmove, alpha);
        }

        self.tt_table.insert(self.state.hash, TtEntry {
            score: alpha,
            depth,
            flag: if alpha <= original_alpha {
                1
            } else if alpha >= beta {
                -1
            } else {
                0
            }
        });

        alpha
    }

    fn quiescence(&mut self, mut alpha: isize, beta: isize, current_ply: usize) -> isize {
        if let Some(duration) = self.search_duration {
            if self.depth_searched > 1 && self.node_counter % 2048 == 0 && Instant::now().duration_since(self.search_start) > duration {
                self.search_active = false;

                return alpha;
            }
        }

        self.node_counter += 1;

        if current_ply >= MAX_PLY {
            return relative_eval(&self.state);
        }

        if let Some(tablebase) = &self.tablebase {
            if count_bits(self.state.occupancy) <= 5 {
                let pos: Chess = self.state.to_fen().parse::<Fen>().unwrap().position(CastlingMode::Standard).unwrap();
                let tb_result = tablebase.best_move(&pos).unwrap();
    
                let (r#move, score) = match tb_result {
                    Some(tb_result) => (
                        encode_move(
                            tb_result.0.from().unwrap() as usize,
                            tb_result.0.to() as usize,
                            Piece::Pawn,
                            match tb_result.0.promotion() {
                                Some(p) => Some(match p {
                                    Role::Pawn => Piece::Pawn,
                                    Role::Knight => Piece::Knight,
                                    Role::Bishop => Piece::Bishop,
                                    Role::Rook => Piece::Rook,
                                    Role::Queen => Piece::Queen,
                                    Role::King => Piece::King
                                }),
                                None => None
                            },
                            false,
                            false,
                            false,
                            false
                        ),
                        match tb_result.1 {
                            Dtz(x) if x > 0 => -MATE_VALUE + current_ply as isize + x as isize,
                            Dtz(x) if x < 0 => MATE_VALUE - current_ply as isize + x as isize,
                            _ => 0
                        }
                    ),
                    None => if self.state.is_in_check(self.state.to_move) { (0, -MATE_VALUE + current_ply as isize) } else { (0, 0) }
                };

                self.tb_hits += 1;
    
                self.tt_table.insert(self.state.hash, TtEntry {
                    score,
                    depth: usize::MAX,
                    flag: 0
                });
    
                if current_ply == 0 {
                    self.best = (r#move, score);
                    self.search_active = false;
                }
    
                return score;
            }
        }

        let standing_pat = relative_eval(&self.state);
        if standing_pat >= beta {
            return beta;
        }
        if standing_pat > alpha {
            alpha = standing_pat;
        }

        let mut moves = generate_moves(&self.state);
        self.sort_moves(&mut moves, current_ply, false);
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
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }

    fn sort_moves(&self, move_list: &mut MoveList, ply: usize, in_pv: bool) {
        move_list.moves[0..move_list.length].sort_by(|a,b| self.score_move(*b, ply, in_pv).cmp(&self.score_move(*a, ply, in_pv)));
    }

    fn score_move(&self, r#move: BitMove, ply: usize, in_pv: bool) -> usize {
        if in_pv && self.previous_pv.moves[ply] == r#move {
            return 11000;
        }

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
        else if self.killers[ply][0] == r#move {
            return 9000;
        }
        else if self.killers[ply][1] == r#move {
            return 8000;
        }
        
        self.history[self.state.to_move as usize][move_from(r#move)][move_to(r#move)]
    }
}