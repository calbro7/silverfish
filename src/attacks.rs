use crate::bitboards::{get_ls1b, get_ms1b};

include!(concat!(env!("OUT_DIR"), "/attacks.rs"));

pub fn bishop_attacks(sq: usize, blockers: u64) -> u64 {
    let mut bb = 0u64;

    let northeast = NORTHEAST_RAYS[sq];
    match get_ls1b(blockers & northeast) {
        Some(first_blocker) => {
            bb |= northeast & !NORTHEAST_RAYS[first_blocker];
        },
        None => {
            bb |= northeast;
        }
    }

    let southeast = SOUTHEAST_RAYS[sq];
    match get_ms1b(blockers & southeast) {
        Some(first_blocker) => {
            bb |= southeast & !SOUTHEAST_RAYS[first_blocker];
        },
        None => {
            bb |= southeast;
        }
    }

    let southwest = SOUTHWEST_RAYS[sq];
    match get_ms1b(blockers & southwest) {
        Some(first_blocker) => {
            bb |= southwest & !SOUTHWEST_RAYS[first_blocker];
        },
        None => {
            bb |= southwest;
        }
    }

    let northwest = NORTHWEST_RAYS[sq];
    match get_ls1b(blockers & northwest) {
        Some(first_blocker) => {
            bb |= northwest & !NORTHWEST_RAYS[first_blocker];
        },
        None => {
            bb |= northwest;
        }
    }

    bb
}

pub fn rook_attacks(sq: usize, blockers: u64) -> u64 {
    let mut bb = 0u64;

    let north = NORTH_RAYS[sq];
    match get_ls1b(blockers & north) {
        Some(first_blocker) => {
            bb |= north & !NORTH_RAYS[first_blocker];
        },
        None => {
            bb |= north;
        }
    }

    let east = EAST_RAYS[sq];
    match get_ls1b(blockers & east) {
        Some(first_blocker) => {
            bb |= east & !EAST_RAYS[first_blocker];
        },
        None => {
            bb |= east;
        }
    }

    let south = SOUTH_RAYS[sq];
    match get_ms1b(blockers & south) {
        Some(first_blocker) => {
            bb |= south & !SOUTH_RAYS[first_blocker];
        },
        None => {
            bb |= south;
        }
    }

    let west = WEST_RAYS[sq];
    match get_ms1b(blockers & west) {
        Some(first_blocker) => {
            bb |= west & !WEST_RAYS[first_blocker];
        },
        None => {
            bb |= west;
        }
    }

    bb
}

pub fn queen_attacks(sq: usize, blockers: u64) -> u64 {
    bishop_attacks(sq, blockers) | rook_attacks(sq, blockers)
}