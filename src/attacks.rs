use crate::bitboards::{set_bit, get_bit};
use crate::helpers::{sq_rank, sq_file, rank_file_to_sq};

include!(concat!(env!("OUT_DIR"), "/attacks.rs"));

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