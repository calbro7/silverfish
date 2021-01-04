use crate::state::State;
use crate::bitboards::{RANK_1, RANK_4, RANK_5, RANK_8, pop_ls1b, get_bit};
use crate::colours::Colour;
use crate::pieces::Piece;
use crate::attacks::{PAWN_ATTACKS, KNIGHT_ATTACKS, bishop_attacks, rook_attacks, queen_attacks, KING_ATTACKS};
use crate::castling::{CastleType, decode_castling};
use crate::helpers::sq_to_algebraic;
use regex::Regex;

pub type BitMove = u32;

pub fn encode_move(from: usize, to: usize, piece: Piece, promotion_piece: Option<Piece>, is_capture: bool, is_double_push: bool, is_ep: bool, is_castle: bool) -> BitMove {
    let mut encoded = 0u32;

    encoded |= from as u32;
    encoded |= (to << 6) as u32;
    encoded |= (piece as u32) << 12;
    let promotion_bits = match promotion_piece {
        Some(piece) => piece as u32,
        None => u32::MAX
    } & 7;
    encoded |= promotion_bits << 15;
    encoded |= (is_capture as u32) << 18;
    encoded |= (is_double_push as u32) << 19;
    encoded |= (is_ep as u32) << 20;
    encoded |= (is_castle as u32) << 21;

    encoded
}

pub fn move_from(r#move: BitMove) -> usize {
    (r#move & 63) as usize
}
pub fn move_to(r#move: BitMove) -> usize {
    ((r#move >> 6) & 63) as usize
}
pub fn move_piece(r#move: BitMove) -> Piece {
    match (r#move >> 12) & 7 {
        0 => Piece::Pawn,
        1 => Piece::Knight,
        2 => Piece::Bishop,
        3 => Piece::Rook,
        4 => Piece::Queen,
        5 => Piece::King,
        _ => panic!("Invalid piece")
    }
}
pub fn move_promotion_piece(r#move: BitMove) -> Option<Piece> {
    match (r#move >> 15) & 7 {
        0 => Some(Piece::Pawn),
        1 => Some(Piece::Knight),
        2 => Some(Piece::Bishop),
        3 => Some(Piece::Rook),
        4 => Some(Piece::Queen),
        5 => Some(Piece::King),
        _ => None
    }
}
pub fn move_is_capture(r#move: BitMove) -> bool {
    ((r#move >> 18) & 1) != 0
}
pub fn move_is_double_push(r#move: BitMove) -> bool {
    ((r#move >> 19) & 1) != 0
}
pub fn move_is_ep(r#move: BitMove) -> bool {
    ((r#move >> 20) & 1) != 0
}
pub fn move_is_castle(r#move: BitMove) -> bool {
    ((r#move >> 21) & 1) != 0
}

pub fn move_to_algebraic(r#move: BitMove) -> String {
    let promotion_suffix = match move_promotion_piece(r#move) {
        Some(Piece::Knight) => "n",
        Some(Piece::Bishop) => "b",
        Some(Piece::Rook) => "r",
        Some(Piece::Queen) => "q",
        _ => ""
    };

    format!("{}{}{}", sq_to_algebraic(move_from(r#move)), sq_to_algebraic(move_to(r#move)), promotion_suffix)
}

pub fn move_string_is_valid(r#move: &str) -> bool {
    Regex::new(r"^[abcdefgh][12345678][abcdefgh][12345678][nbrq]?$").unwrap().is_match(r#move)
}

pub struct MoveList {
    moves: [BitMove; 255],
    length: usize,
    current: usize
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            moves: [0; 255],
            length: 0,
            current: 0
        }
    }

    pub fn push(&mut self, r#move: BitMove) {
        self.moves[self.length] = r#move;
        self.length += 1;
    }

    pub fn sort(&mut self, state: &State) {
        self.moves[0..self.length].sort_by(|a,b| score_move(&state, *b).cmp(&score_move(&state, *a)));
    }
}

impl Iterator for MoveList {
    type Item = BitMove;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        match self.moves[self.current - 1] {
            0 => None,
            m => Some(m)
        }
    }
}

pub fn generate_moves(state: &State) -> MoveList {
    let mut moves = MoveList::new();

    let us = state.colours[state.to_move as usize];
    let them = state.colours[!state.to_move as usize];
    let final_rank = match state.to_move {
        Colour::White => RANK_8,
        Colour::Black => RANK_1
    };

    // pawn moves
    let mut pawns = state.pieces[Piece::Pawn as usize] & us;
    let mut push_one = match state.to_move {
        Colour::White => pawns << 8,
        Colour::Black => pawns >> 8
    } & !state.occupancy;
    let mut push_two = match state.to_move {
        Colour::White => (push_one << 8) & RANK_4,
        Colour::Black => (push_one >> 8) & RANK_5
    } & !state.occupancy;

    while push_one != 0 {
        let to = pop_ls1b(&mut push_one);
        let from = match state.to_move {
            Colour::White => to - 8,
            Colour::Black => to + 8
        };

        if get_bit(final_rank, to) {
            for piece in &[Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen] {
                moves.push(encode_move(from, to, Piece::Pawn, Some(*piece), false, false, false, false));
            }
        }
        else {
            moves.push(encode_move(from, to, Piece::Pawn, None, false, false, false, false));
        }
    }
    while push_two != 0 {
        let to = pop_ls1b(&mut push_two);
        let from = match state.to_move {
            Colour::White => to - 16,
            Colour::Black => to + 16
        };
        moves.push(encode_move(from, to, Piece::Pawn, None, false, true, false, false));
    }
    if let Some(ep_target) = state.ep_target {
        let mut ep_capturers = PAWN_ATTACKS[ep_target][!state.to_move as usize] & pawns;
        while ep_capturers != 0 {
            let from = pop_ls1b(&mut ep_capturers);
            moves.push(encode_move(from, ep_target, Piece::Pawn, None, true, false, true, false));
        }
    }
    while pawns != 0 {
        let from = pop_ls1b(&mut pawns);
        let mut attacks = PAWN_ATTACKS[from][state.to_move as usize] & them;
        while attacks != 0 {
            let to = pop_ls1b(&mut attacks);
            if get_bit(final_rank, to) {
                for piece in &[Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen] {
                    moves.push(encode_move(from, to, Piece::Pawn, Some(*piece), true, false, false, false));
                }
            }
            else {
                moves.push(encode_move(from, to, Piece::Pawn, None, true, false, false, false));
            }
        }
    }

    // knight moves
    let mut knights = state.pieces[Piece::Knight as usize] & us;
    while knights != 0 {
        let from = pop_ls1b(&mut knights);
        let mut attacks = KNIGHT_ATTACKS[from] & !us;
        while attacks != 0 {
            let to = pop_ls1b(&mut attacks);
            let is_capture = get_bit(them, to);
            moves.push(encode_move(from, to, Piece::Knight, None, is_capture, false, false, false));
        }
    }

    // bishop moves
    let mut bishops = state.pieces[Piece::Bishop as usize] & us;
    while bishops != 0 {
        let from = pop_ls1b(&mut bishops);
        let mut attacks = bishop_attacks(from, state.occupancy) & !us;
        while attacks != 0 {
            let to = pop_ls1b(&mut attacks);
            let is_capture = get_bit(them, to);
            moves.push(encode_move(from, to, Piece::Bishop, None, is_capture, false, false, false));
        }
    }

    // rook moves
    let mut rooks = state.pieces[Piece::Rook as usize] & us;
    while rooks != 0 {
        let from = pop_ls1b(&mut rooks);
        let mut attacks = rook_attacks(from, state.occupancy) & !us;
        while attacks != 0 {
            let to = pop_ls1b(&mut attacks);
            let is_capture = get_bit(them, to);
            moves.push(encode_move(from, to, Piece::Rook, None, is_capture, false, false, false));
        }
    }

    // queen moves
    let mut queens = state.pieces[Piece::Queen as usize] & us;
    while queens != 0 {
        let from = pop_ls1b(&mut queens);
        let mut attacks = queen_attacks(from, state.occupancy) & !us;
        while attacks != 0 {
            let to = pop_ls1b(&mut attacks);
            let is_capture = get_bit(them, to);
            moves.push(encode_move(from, to, Piece::Queen, None, is_capture, false, false, false));
        }
    }

    // king moves
    let mut kings = state.pieces[Piece::King as usize] & us;
    while kings != 0 {
        let from = pop_ls1b(&mut kings);
        let mut attacks = KING_ATTACKS[from] & !us;
        while attacks != 0 {
            let to = pop_ls1b(&mut attacks);
            let is_capture = get_bit(them, to);
            moves.push(encode_move(from, to, Piece::King, None, is_capture, false, false, false));
        }
    }

    // castle moves
    match state.to_move {
        Colour::White => {
            if decode_castling(state.castling, CastleType::WhiteKingside) && !get_bit(state.occupancy, 5) && !get_bit(state.occupancy, 6) && !state.square_attacked(4, !state.to_move) && !state.square_attacked(5, !state.to_move) {
                moves.push(encode_move(4, 6, Piece::King, None, false, false, false, true));
            }
            if decode_castling(state.castling, CastleType::WhiteQueenside) && !get_bit(state.occupancy, 3) && !get_bit(state.occupancy, 2) && !get_bit(state.occupancy, 1) && !state.square_attacked(4, !state.to_move) && !state.square_attacked(3, !state.to_move) {
                moves.push(encode_move(4, 2, Piece::King, None, false, false, false, true));
            }
        },
        Colour::Black => {
            if decode_castling(state.castling, CastleType::BlackKingside) && !get_bit(state.occupancy, 61) && !get_bit(state.occupancy, 62) && !state.square_attacked(60, !state.to_move) && !state.square_attacked(61, !state.to_move) {
                moves.push(encode_move(60, 62, Piece::King, None, false, false, false, true));
            }
            if decode_castling(state.castling, CastleType::BlackQueenside) && !get_bit(state.occupancy, 59) && !get_bit(state.occupancy, 58) && !get_bit(state.occupancy, 57) && !state.square_attacked(60, !state.to_move) && !state.square_attacked(59, !state.to_move) {
                moves.push(encode_move(60, 58, Piece::King, None, false, false, false, true));
            }
        }
    }

    moves
}

fn score_move(state: &State, r#move: BitMove) -> usize {
    if move_is_capture(r#move) {
        let mut captured_piece = Piece::Pawn;
        if !move_is_ep(r#move) {
            // todo - is it quicker to add pawns to the start of this list? (extra iteration, but most captures will be of pawns)
            for piece in &[Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen, Piece::King] {
                if get_bit(state.pieces[*piece as usize], move_to(r#move)) {
                    captured_piece = *piece;
                    break;
                }
            }
        }

        return 6 * (captured_piece as usize) + (5 - move_piece(r#move) as usize);
    }
    
    0
}