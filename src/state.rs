use crate::pieces::Piece;
use crate::colours::Colour;
use crate::bitboards::{get_bit, set_bit, clear_bit, get_ls1b};
use crate::helpers::{rank_file_to_sq, sq_file, sq_to_algebraic, algebraic_to_sq};
use crate::castling::{CastleType, decode_castling};
use crate::attacks::{PAWN_ATTACKS,KNIGHT_ATTACKS,bishop_attacks,rook_attacks,queen_attacks,KING_ATTACKS};
use crate::moves::{BitMove, move_from, move_to, move_piece, move_is_capture, move_promotion_piece, move_is_double_push, move_is_ep, move_is_castle};
use crate::zobrist;
use crate::errors::{InvalidFenError, IllegalMoveError};
use std::fmt;

#[derive(Clone, Copy)]
pub struct History {
    hashes: [u64; 255],
    length: usize
}

impl History {
    pub fn new() -> Self {
        Self {
            hashes: [0; 255],
            length: 0
        }
    }

    pub fn push(&mut self, hash: u64) {
        self.hashes[self.length] = hash;
        self.length += 1;
    }

    pub fn clear(&mut self) {
        self.hashes = [0; 255];
        self.length = 0;
    }
}

#[derive(Clone, Copy)]
pub struct State {
    pub pieces: [u64; 6],
    pub colours: [u64; 2],
    pub occupancy: u64,
    pub to_move: Colour,
    pub ep_target: Option<usize>,
    pub castling: u8,
    pub halfmove_clock: u8,
    hash: u64,
    pub history: History
}

impl State {
    pub fn new() -> Self {
        Self {
            pieces: [0; 6],
            colours: [0; 2],
            occupancy: 0,
            to_move: Colour::White,
            ep_target: None,
            castling: 0,
            halfmove_clock: 0,
            hash: 0,
            history: History::new()
        }
    }

    pub fn start_pos() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn from_fen(fen: &str) -> Result<Self, InvalidFenError> {
        let fen_segments: Vec<&str> = fen.split(' ').collect();

        let mut state = Self::new();

        let mut sq = 56usize;
        for c in fen_segments[0].chars() {
            if c == '/' {
                sq -= 16;
            }
            else if c.is_digit(10) {
                sq += c.to_digit(10).unwrap() as usize;
            }
            else {
                let piece = match c.to_ascii_lowercase() {
                    'p' => Piece::Pawn,
                    'n' => Piece::Knight,
                    'b' => Piece::Bishop,
                    'r' => Piece::Rook,
                    'q' => Piece::Queen,
                    'k' => Piece::King,
                    _ => {
                        return Err(InvalidFenError {
                            fen: fen.to_string()
                        })
                    }
                };
                let colour = if c == c.to_ascii_uppercase() {Colour::White} else {Colour::Black};

                state.pieces[piece as usize] = set_bit(state.pieces[piece as usize], sq);
                state.colours[colour as usize] = set_bit(state.colours[colour as usize], sq);
                state.hash ^= zobrist::PIECES[colour as usize][piece as usize][sq];

                sq += 1;
            }
        }

        state.to_move = match fen_segments[1] {
            "w" => {
                state.hash ^= zobrist::WHITE_MOVE;

                Colour::White
            },
            "b" => Colour::Black,
            _ => {
                return Err(InvalidFenError {
                    fen: fen.to_string()
                })
            }
        };

        if fen_segments[2].contains('K') {
            state.castling |= CastleType::WhiteKingside as u8
        }
        if fen_segments[2].contains('Q') {
            state.castling |= CastleType::WhiteQueenside as u8
        }
        if fen_segments[2].contains('k') {
            state.castling |= CastleType::BlackKingside as u8
        }
        if fen_segments[2].contains('q') {
            state.castling |= CastleType::BlackQueenside as u8
        }
        state.hash ^= zobrist::CASTLING[state.castling as usize];

        state.ep_target = match fen_segments[3] {
            "-" => None,
            sq => {
                let sq = algebraic_to_sq(sq);
                state.hash ^= zobrist::EP_FILE[sq_file(sq)];

                Some(sq)
            }
        };

        state.halfmove_clock = fen_segments[4].parse().unwrap();

        state.occupancy = state.colours[Colour::White as usize] | state.colours[Colour::Black as usize];

        Ok(state)
    }

    pub fn square_attacked(&self, sq: usize, colour: Colour) -> bool {
        let colour_bb = self.colours[colour as usize];
        
        (PAWN_ATTACKS[sq][!colour as usize] & self.pieces[Piece::Pawn as usize] & colour_bb != 0)
        | (KNIGHT_ATTACKS[sq] & self.pieces[Piece::Knight as usize] & colour_bb != 0)
        | (bishop_attacks(sq, self.occupancy) & self.pieces[Piece::Bishop as usize] & colour_bb != 0)
        | (rook_attacks(sq, self.occupancy) & self.pieces[Piece::Rook as usize] & colour_bb != 0)
        | (queen_attacks(sq, self.occupancy) & self.pieces[Piece::Queen as usize] & colour_bb != 0)
        | (KING_ATTACKS[sq] & self.pieces[Piece::King as usize] & colour_bb != 0)
    }

    pub fn is_in_check(&self, colour: Colour) -> bool {
        let king_sq = get_ls1b(self.colours[colour as usize] & self.pieces[Piece::King as usize]).unwrap();

        self.square_attacked(king_sq, !colour)
    }

    pub fn make_move(&mut self, r#move: BitMove) -> Result<(), IllegalMoveError> {
        let copy = self.clone();

        let from = move_from(r#move);
        let to = move_to(r#move);
        let piece = move_piece(r#move);
        let promotion_piece = move_promotion_piece(r#move);
        let is_capture = move_is_capture(r#move);
        let is_double_push = move_is_double_push(r#move);
        let is_ep = move_is_ep(r#move);
        let is_castle = move_is_castle(r#move);

        self.hash ^= zobrist::PIECES[self.to_move as usize][piece as usize][from];
        if let Some(sq) = self.ep_target {
            self.hash ^= zobrist::EP_FILE[sq_file(sq)];
        }
        
        if is_capture {
            for piece in &[Piece::Pawn, Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen, Piece::King] {
                if get_bit(self.pieces[*piece as usize], to) {
                    self.hash ^= zobrist::PIECES[!self.to_move as usize][*piece as usize][to];
                    self.pieces[*piece as usize] = clear_bit(self.pieces[*piece as usize], to);
                    break;
                }
            }
            self.colours[!self.to_move as usize] = clear_bit(self.colours[!self.to_move as usize], to);
        }

        self.pieces[piece as usize] = clear_bit(self.pieces[piece as usize], from);
        if let Some(promotion_piece) = promotion_piece {
            self.hash ^= zobrist::PIECES[self.to_move as usize][promotion_piece as usize][to];
            self.pieces[promotion_piece as usize] = set_bit(self.pieces[promotion_piece as usize], to);
        }
        else {
            self.hash ^= zobrist::PIECES[self.to_move as usize][piece as usize][to];
            self.pieces[piece as usize] = set_bit(self.pieces[piece as usize], to);
        }

        self.colours[self.to_move as usize] = clear_bit(self.colours[self.to_move as usize], from);
        self.colours[self.to_move as usize] = set_bit(self.colours[self.to_move as usize], to);

        if is_ep {
            let captured_pawn_sq = match self.to_move {
                Colour::White => self.ep_target.unwrap() - 8,
                Colour::Black => self.ep_target.unwrap() + 8
            };
            self.hash ^= zobrist::PIECES[!self.to_move as usize][Piece::Pawn as usize][captured_pawn_sq];
            self.hash ^= zobrist::EP_FILE[sq_file(to)];
            self.pieces[Piece::Pawn as usize] = clear_bit(self.pieces[Piece::Pawn as usize], captured_pawn_sq);
            self.colours[!self.to_move as usize] = clear_bit(self.colours[!self.to_move as usize], captured_pawn_sq);
        }
        
        self.ep_target = match is_double_push {
            true => match self.to_move {
                Colour::White => Some(to - 8),
                Colour::Black => Some(to + 8)
            },
            false => None
        };
        if let Some(sq) = self.ep_target {
            self.hash ^= zobrist::EP_FILE[sq_file(sq)];
        }

        if is_castle {
            match to {
                6 => {
                    self.pieces[Piece::Rook as usize] = clear_bit(self.pieces[Piece::Rook as usize], 7);
                    self.pieces[Piece::Rook as usize] = set_bit(self.pieces[Piece::Rook as usize], 5);
                    self.colours[Colour::White as usize] = clear_bit(self.colours[Colour::White as usize], 7);
                    self.colours[Colour::White as usize] = set_bit(self.colours[Colour::White as usize], 5);
                },
                2 => {
                    self.pieces[Piece::Rook as usize] = clear_bit(self.pieces[Piece::Rook as usize], 0);
                    self.pieces[Piece::Rook as usize] = set_bit(self.pieces[Piece::Rook as usize], 3);
                    self.colours[Colour::White as usize] = clear_bit(self.colours[Colour::White as usize], 0);
                    self.colours[Colour::White as usize] = set_bit(self.colours[Colour::White as usize], 3);
                },
                62 => {
                    self.pieces[Piece::Rook as usize] = clear_bit(self.pieces[Piece::Rook as usize], 63);
                    self.pieces[Piece::Rook as usize] = set_bit(self.pieces[Piece::Rook as usize], 61);
                    self.colours[Colour::Black as usize] = clear_bit(self.colours[Colour::Black as usize], 63);
                    self.colours[Colour::Black as usize] = set_bit(self.colours[Colour::Black as usize], 61);
                },
                58 => {
                    self.pieces[Piece::Rook as usize] = clear_bit(self.pieces[Piece::Rook as usize], 56);
                    self.pieces[Piece::Rook as usize] = set_bit(self.pieces[Piece::Rook as usize], 59);
                    self.colours[Colour::Black as usize] = clear_bit(self.colours[Colour::Black as usize], 56);
                    self.colours[Colour::Black as usize] = set_bit(self.colours[Colour::Black as usize], 59);
                },
                _ => panic!("Invalid castle move")
            }
        }
        self.hash ^= zobrist::CASTLING[self.castling as usize];
        if from == 0 || to == 0 {
            self.castling &= !(CastleType::WhiteQueenside as u8);
        }
        if from == 7 || to == 7 {
            self.castling &= !(CastleType::WhiteKingside as u8);
        }
        if from == 4 || to == 4 {
            self.castling &= !(CastleType::WhiteQueenside as u8) & !(CastleType::WhiteKingside as u8);
        }
        if from == 56 || to == 56 {
            self.castling &= !(CastleType::BlackQueenside as u8);
        }
        if from == 63 || to == 63 {
            self.castling &= !(CastleType::BlackKingside as u8);
        }
        if from == 60 || to == 60 {
            self.castling &= !(CastleType::BlackQueenside as u8) & !(CastleType::BlackKingside as u8);
        }
        self.hash ^= zobrist::CASTLING[self.castling as usize];

        self.occupancy = self.colours[Colour::White as usize] | self.colours[Colour::Black as usize];

        if is_capture || piece == Piece::Pawn {
            self.halfmove_clock = 0;
        }
        else {
            self.halfmove_clock += 1;
        }

        if self.is_in_check(self.to_move) {
            *self = copy;
            return Err(IllegalMoveError);
        }

        self.to_move = !self.to_move;
        self.hash ^= zobrist::WHITE_MOVE;
        if piece == Piece::Pawn || is_castle || is_capture {
            self.history.clear();
        }
        else {
            self.history.push(copy.hash);
        }

        Ok(())
    }

    pub fn is_repetition(&self) -> bool {
        if self.history.length == 0 {
            return false;
        }

        for i in (0..self.history.length-1).rev().step_by(2) {
            if self.history.hashes[i] == self.hash {
                return true;
            }
        }

        return false;
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        for rank in (0..8).rev() {
            output.push('|');
            for file in 0..8 {
                let sq = rank_file_to_sq(rank, file);
                let mut sq_occupied = false;
                for piece in &[Piece::Pawn, Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen, Piece::King] {
                    if get_bit(self.pieces[*piece as usize], sq) {
                        let mut piece_char = match piece {
                            Piece::Pawn => 'p',
                            Piece::Knight => 'n',
                            Piece::Bishop => 'b',
                            Piece::Rook => 'r',
                            Piece::Queen => 'q',
                            Piece::King => 'k'
                        };
                        if get_bit(self.colours[Colour::White as usize], sq) {
                            piece_char = piece_char.to_ascii_uppercase();
                        }

                        output.push(piece_char);
                        sq_occupied = true;
                    }
                }
                if !sq_occupied {
                    output.push(match self.ep_target {
                        Some(sq2) if sq == sq2 => 'x',
                        _ => '-'
                    });
                }
                output.push('|');
            }
            output.push('\n');
        }

        output.push_str(&format!("To move: {}", match self.to_move {
            Colour::White => "White",
            Colour::Black => "Black"
        }));
        output.push('\n');

        output.push_str("Castling: ");
        if self.castling == 0 {
            output.push('-');
        }
        else {
            if decode_castling(self.castling, CastleType::WhiteKingside) {
                output.push('K')
            }
            if decode_castling(self.castling, CastleType::WhiteQueenside) {
                output.push('Q')
            }
            if decode_castling(self.castling, CastleType::BlackKingside) {
                output.push('k')
            }
            if decode_castling(self.castling, CastleType::BlackQueenside) {
                output.push('q')
            }
        }
        output.push('\n');

        output.push_str(&format!("EP target: {}", match self.ep_target {
            Some(sq) => sq_to_algebraic(sq),
            None => "-".to_string()
        }));
        output.push('\n');

        write!(f, "{}", output)
    }
}