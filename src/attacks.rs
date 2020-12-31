use crate::bitboards::{NOT_A_FILE, NOT_H_FILE, NOT_AB_FILE, NOT_GH_FILE, from_sq, set_bit, get_bit};
use crate::colours::Colour;
use crate::helpers::{sq_rank, sq_file, rank_file_to_sq};

pub fn pawn_attacks(sq: usize, colour: Colour) -> u64 {
    let sq_bb = from_sq(sq);

    match colour {
        Colour::White => ((sq_bb << 7) & NOT_H_FILE) | ((sq_bb << 9) & NOT_A_FILE),
        Colour::Black => ((sq_bb >> 9) & NOT_H_FILE) | ((sq_bb >> 7) & NOT_A_FILE)
    }
}

pub fn knight_attacks(sq: usize) -> u64 {
    let sq_bb = from_sq(sq);

    ((sq_bb << 17) & NOT_A_FILE)
    | ((sq_bb << 10) & NOT_AB_FILE)
    | ((sq_bb >> 6) & NOT_AB_FILE)
    | ((sq_bb >> 15) & NOT_A_FILE)
    | ((sq_bb >> 17) & NOT_H_FILE)
    | ((sq_bb >> 10) & NOT_GH_FILE)
    | ((sq_bb << 6) & NOT_GH_FILE)
    | ((sq_bb << 15) & NOT_H_FILE)
}

pub fn king_attacks(sq: usize) -> u64 {
    let sq_bb = from_sq(sq);

    (sq_bb << 8)
    | ((sq_bb << 9) & NOT_A_FILE)
    | ((sq_bb << 1) & NOT_A_FILE)
    | ((sq_bb >> 7) & NOT_A_FILE)
    | (sq_bb >> 8)
    | ((sq_bb >> 9) & NOT_H_FILE)
    | ((sq_bb >> 1) & NOT_H_FILE)
    | ((sq_bb << 7) & NOT_H_FILE)
}

pub fn bishop_attacks(sq: usize, blockers: u64) -> u64 {
    let mut bb = 0u64;

    let mut rank = sq_rank(sq);
    let mut file = sq_file(sq);
    while rank < 7 && file < 7 {
        rank += 1;
        file += 1;
        bb = set_bit(bb, rank_file_to_sq(rank, file));
        if get_bit(blockers, rank_file_to_sq(rank, file)) {
            break
        }
    }

    rank = sq_rank(sq);
    file = sq_file(sq);
    while rank > 0 && file < 7 {
        rank -= 1;
        file += 1;
        bb = set_bit(bb, rank_file_to_sq(rank, file));
        if get_bit(blockers, rank_file_to_sq(rank, file)) {
            break
        }
    }

    rank = sq_rank(sq);
    file = sq_file(sq);
    while rank > 0 && file > 0 {
        rank -= 1;
        file -= 1;
        bb = set_bit(bb, rank_file_to_sq(rank, file));
        if get_bit(blockers, rank_file_to_sq(rank, file)) {
            break
        }
    }

    rank = sq_rank(sq);
    file = sq_file(sq);
    while rank < 7 && file > 0 {
        rank += 1;
        file -= 1;
        bb = set_bit(bb, rank_file_to_sq(rank, file));
        if get_bit(blockers, rank_file_to_sq(rank, file)) {
            break
        }
    }

    bb
}

pub fn rook_attacks(sq: usize, blockers: u64) -> u64 {
    let mut bb = 0u64;

    let mut rank = sq_rank(sq);
    let mut file = sq_file(sq);
    while rank < 7 {
        rank += 1;
        bb = set_bit(bb, rank_file_to_sq(rank, file));
        if get_bit(blockers, rank_file_to_sq(rank, file)) {
            break
        }
    }
    
    rank = sq_rank(sq);
    file = sq_file(sq);
    while file < 7 {
        file += 1;
        bb = set_bit(bb, rank_file_to_sq(rank, file));
        if get_bit(blockers, rank_file_to_sq(rank, file)) {
            break
        }
    }
    
    rank = sq_rank(sq);
    file = sq_file(sq);
    while rank > 0 {
        rank -= 1;
        bb = set_bit(bb, rank_file_to_sq(rank, file));
        if get_bit(blockers, rank_file_to_sq(rank, file)) {
            break
        }
    }
    
    rank = sq_rank(sq);
    file = sq_file(sq);
    while file > 0 {
        file -= 1;
        bb = set_bit(bb, rank_file_to_sq(rank, file));
        if get_bit(blockers, rank_file_to_sq(rank, file)) {
            break
        }
    }

    bb
}

pub fn queen_attacks(sq: usize, blockers: u64) -> u64 {
    bishop_attacks(sq, blockers) | rook_attacks(sq, blockers)
}