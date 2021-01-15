use crate::state::State;
use crate::helpers::{algebraic_to_sq};
use crate::moves::{move_from, move_to, move_promotion_piece, generate_moves, move_to_algebraic, move_string_is_valid};
use crate::colours::Colour;
use crate::pieces::Piece;
use crate::perft::perft;
use crate::eval::eval;
use crate::search::{Search, Message};
use std::process::exit;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};

pub struct UciHandler {
    state: State,
    out: Arc<Mutex<dyn std::io::Write + std::marker::Send>>,
    transmitter: Option<Sender<Message>>
}

impl UciHandler {
    pub fn new(out: Arc<Mutex<dyn std::io::Write + std::marker::Send>>) -> Self {
        Self {
            state: State::start_pos(),
            out: out,
            transmitter: None
        }
    }

    pub fn command(&mut self, command: &str) {
        if command.starts_with("isready") {
            self.isready();
        }
        else if command.starts_with("ucinewgame") {
            self.ucinewgame();
        }
        else if command.starts_with("position") {
            self.position(command);
        }
        else if command.starts_with("go") {
            self.go(command);
        }
        else if command.starts_with("stop") {
            self.stop();
        }
        else if command.starts_with("uci") {
            self.uci();
        }
        else if command.starts_with("quit") {
            self.quit();
        }
        else if command.starts_with("perft") {
            self.perft(command);
        }
        else if command.starts_with("eval") {
            self.eval();
        }
        else if command.starts_with("print") {
            self.print();
        }
    }

    fn isready(&mut self) {
        writeln!(self.out.lock().unwrap(), "readyok").unwrap();
    }

    fn ucinewgame(&mut self,) {
        self.state = State::start_pos();
    }

    fn position(&mut self, command: &str) {
        let mut segments = command.split_whitespace().skip(1);
        let mut state = match segments.next() {
            Some("startpos") => State::start_pos(),
            Some("fen") => {
                let mut fen = String::new();
                for _ in 0..6 {
                    match segments.next() {
                        Some(s) => fen.push_str(&(" ".to_owned() + s)),
                        None => {
                            return;
                        }
                    }
                }
                fen = fen.trim().to_string();
                match State::from_fen(&fen) {
                    Ok(s) => s,
                    Err(_) => {
                        return;
                    }
                }
            },
            _ => {
                return;
            }
        };
    
        if let Some("moves") = segments.next() {
            'moveparsing: loop {
                match segments.next() {
                    Some(move_string) if move_string_is_valid(move_string) => {
                        let mut move_list = generate_moves(&state);
    
                        let from = algebraic_to_sq(&move_string[0..2]);
                        let to = algebraic_to_sq(&move_string[2..4]);
                        let promotion_piece = if move_string.len() == 5 {
                            match &move_string[4..5] {
                                "n" => Some(Piece::Knight),
                                "b" => Some(Piece::Bishop),
                                "r" => Some(Piece::Rook),
                                "q" => Some(Piece::Queen),
                                _ => {
                                    return;
                                }
                            }
                        } else { None };
    
                        while let Some(r#move) = move_list.next() {
                            if from == move_from(r#move) && to == move_to(r#move) && promotion_piece == move_promotion_piece(r#move) {
                                // If this is a legal move, proceed to parse the next move. Otherwise, stop parsing the moves altogether
                                match state.make_move(r#move) {
                                    Ok(_) => {
                                        break;
                                    },
                                    Err(_) => {
                                        break 'moveparsing;
                                    }
                                }
                            }
                        }
                    },
                    _ => break
                }
            }
        }
        
        self.state = state;
    }

    fn go(&mut self, command: &str) {
        let mut searcher = Search::new(self.state);

        let mut segments = command.split_whitespace().skip(1);
        loop {
            match segments.next() {
                Some("depth") => {
                    searcher.set_depth(segments.next().unwrap().parse().unwrap());
                },
                Some("movetime") => {
                    searcher.set_search_duration(Some(Duration::from_millis(segments.next().unwrap().parse().unwrap())));
                },
                Some("wtime") => {
                    searcher.set_time(Colour::White, Some(Duration::from_millis(segments.next().unwrap().parse().unwrap())));
                },
                Some("btime") => {
                    searcher.set_time(Colour::Black, Some(Duration::from_millis(segments.next().unwrap().parse().unwrap())));
                },
                _ => {
                    break;
                }
            }
        }

        let (uci_transmitter, search_receiver) = channel();
        let (search_transmitter, uci_receiver) = channel();
        self.transmitter = Some(uci_transmitter);
        searcher.set_channels(Some((search_transmitter, search_receiver)));

        let out1 = self.out.clone();
        std::thread::spawn(move || {
            let bestmove = searcher.go();
            writeln!(out1.lock().unwrap(), "bestmove {}", move_to_algebraic(bestmove.0)).unwrap();
        });

        let out2 = self.out.clone();
        std::thread::spawn(move || {
            loop {
                match uci_receiver.recv().unwrap() {
                    Message::Info(depth, nodes, bestmove, cp) => writeln!(out2.lock().unwrap(), "info depth {} nodes {} bestmove {} cp {}", depth, nodes, move_to_algebraic(bestmove), cp).unwrap(),
                    Message::Done => {
                        break;
                    },
                    _ => {}
                }
            }
        });
    }

    fn stop(&mut self) {
        self.transmitter.as_ref().unwrap().send(Message::Stop).unwrap();
    }

    fn uci(&mut self) {
        writeln!(self.out.lock().unwrap(), "id name silverfish").unwrap();
        writeln!(self.out.lock().unwrap(), "uciok").unwrap();
    }

    fn quit(&self) {
        exit(0);
    }

    fn perft(&mut self, command: &str) {
        let start = std::time::Instant::now();
        let depth: u8 = command.split_whitespace().skip(1).next().unwrap().parse().unwrap();
        
        // We wish to find all legal moves, sorted by (from, to) (with promotion piece in desc order, if applicable)
        let mut moves = generate_moves(&self.state);
        let mut legal_moves = Vec::new();
        while let Some(r#move) = moves.next() {
            let copy = self.state.clone();
            if self.state.make_move(r#move).is_err() {
                continue;
            }
            legal_moves.push(r#move);
            self.state = copy;
        }
        legal_moves.sort_by(|m1,m2| match move_promotion_piece(*m1) {
            Some(p) => (move_from(*m1), move_to(*m1), -(p as isize)).cmp(&(move_from(*m2), move_to(*m2), -(move_promotion_piece(*m2).unwrap() as isize))),
            None => (move_from(*m1), move_to(*m1)).cmp(&(move_from(*m2), move_to(*m2)))
        });
        
        let mut total = 0;
        for r#move in legal_moves {
            let copy = self.state.clone();
            self.state.make_move(r#move).unwrap();
            let n = perft(&mut self.state, depth-1);
            self.state = copy;
            total += n;
            writeln!(self.out.lock().unwrap(), "{}: {}", move_to_algebraic(r#move), n).unwrap();
        }

        writeln!(self.out.lock().unwrap(), "Total: {} ({:.3?})", total, start.elapsed()).unwrap();
    }

    fn eval(&mut self) {
        writeln!(self.out.lock().unwrap(), "{}", eval(&self.state)).unwrap();
    }

    fn print(&mut self) {
        writeln!(self.out.lock().unwrap(), "{}", self.state).unwrap();
    }
}