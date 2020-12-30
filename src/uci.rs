use crate::state::State;
use crate::helpers::{algebraic_to_sq, sq_to_algebraic};
use crate::moves::{BitMove, encode_move, move_from, move_to, move_promotion_piece, generate_moves};
use crate::pieces::Piece;
use crate::colours::Colour;
use crate::perft::perft;
use crate::eval::eval;
use std::str::SplitWhitespace;
use std::process::exit;
use std::io::{Write};

pub struct UciHandler<W: Write> {
    state: State,
    out: W
}

impl<W> UciHandler<W> where W: Write {
    pub fn new(out: W) -> Self {
        Self {
            state: State::start_pos(),
            out: out
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
    }

    fn isready(&mut self) {
        writeln!(&mut self.out, "readyok");
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
                for i in 0..6 {
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
            loop {
                match segments.next() {
                    Some(move_string) => {
                        if move_string.len() < 4 {
                            return;
                        }
    
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
    
                        while !move_list.is_empty() {
                            let r#move = move_list.pop();
                            if from == move_from(r#move) && to == move_to(r#move) && promotion_piece == move_promotion_piece(r#move) {
                                state.make_move(r#move);
                                break;
                            }
                        }
                    },
                    None => break
                }
            }
        }
        
        self.state = state;
    }

    fn go(&mut self, command: &str) {
        let mut moves = generate_moves(&self.state);
        while !moves.is_empty() {
            let r#move = moves.pop();
            match self.state.make_move(r#move) {
                Ok(_) => {
                    writeln!(&mut self.out, "bestmove {}{}", sq_to_algebraic(move_from(r#move)), sq_to_algebraic(move_to(r#move)));
                    break;
                },
                Err(_) => {}
            }
        }
    }

    fn uci(&mut self) {
        writeln!(&mut self.out, "id name silverfish");
        writeln!(&mut self.out, "uciok");
    }

    fn quit(&self) {
        exit(0);
    }

    fn perft(&mut self, command: &str) {
        let depth: u8 = command.split_whitespace().skip(1).next().unwrap().parse().unwrap();
        writeln!(&mut self.out, "{}", perft(&mut self.state, depth, true));
    }

    fn eval(&mut self) {
        writeln!(&mut self.out, "{}", eval(&self.state) * match self.state.to_move {
            Colour::White => 1,
            Colour::Black => -1
        });
    }
}