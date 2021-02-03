use std::env;
use std::fs;
use std::path::Path;
use rand::random;

const NOT_A_FILE: u64 = 0b1111111011111110111111101111111011111110111111101111111011111110;
const NOT_H_FILE: u64 = 0b0111111101111111011111110111111101111111011111110111111101111111;
const NOT_AB_FILE: u64 = 0b1111110011111100111111001111110011111100111111001111110011111100;
const NOT_GH_FILE: u64 = 0b0011111100111111001111110011111100111111001111110011111100111111;

fn white_pawn_attacks(sq: usize) -> u64 {
    let sq_bb = 1 << sq;
    ((sq_bb << 7) & NOT_H_FILE) | ((sq_bb << 9) & NOT_A_FILE)
}

fn black_pawn_attacks(sq: usize) -> u64 {
    let sq_bb = 1 << sq;
    ((sq_bb >> 9) & NOT_H_FILE) | ((sq_bb >> 7) & NOT_A_FILE)
}

fn knight_attacks(sq: usize) -> u64 {
    let sq_bb = 1 << sq;
    ((sq_bb << 17) & NOT_A_FILE)
        | ((sq_bb << 10) & NOT_AB_FILE)
        | ((sq_bb >> 6) & NOT_AB_FILE)
        | ((sq_bb >> 15) & NOT_A_FILE)
        | ((sq_bb >> 17) & NOT_H_FILE)
        | ((sq_bb >> 10) & NOT_GH_FILE)
        | ((sq_bb << 6) & NOT_GH_FILE)
        | ((sq_bb << 15) & NOT_H_FILE)
}

fn king_attacks(sq: usize) -> u64 {
    let sq_bb = 1 << sq;
    (sq_bb << 8)
        | ((sq_bb << 9) & NOT_A_FILE)
        | ((sq_bb << 1) & NOT_A_FILE)
        | ((sq_bb >> 7) & NOT_A_FILE)
        | (sq_bb >> 8)
        | ((sq_bb >> 9) & NOT_H_FILE)
        | ((sq_bb >> 1) & NOT_H_FILE)
        | ((sq_bb << 7) & NOT_H_FILE)
}

fn north_ray(sq: usize) -> u64 {
    let mut bb = 1 << sq;
    for _ in 0..7 {
        bb |= bb << 8;
    }
    bb & !(1 << sq)
}

fn east_ray(sq: usize) -> u64 {
    let mut bb = 1 << sq;
    for _ in 0..7 {
        bb |= (bb << 1) & NOT_A_FILE
    }
    bb & !(1 << sq)
}

fn south_ray(sq: usize) -> u64 {
    let mut bb = 1 << sq;
    for _ in 0..7 {
        bb |= bb >> 8;
    }
    bb & !(1 << sq)
}

fn west_ray(sq: usize) -> u64 {
    let mut bb = 1 << sq;
    for _ in 0..7 {
        bb |= (bb >> 1) & NOT_H_FILE;
    }
    bb & !(1 << sq)
}

fn northeast_ray(sq: usize) -> u64 {
    let mut bb = 0u64;
    let sq_rank = sq / 8;
    let sq_file = sq % 8;

    let mut r = sq_rank;
    let mut f = sq_file;
    while r < 7 && f < 7 {
        r += 1;
        f += 1;
        bb |= 1 << (r*8 + f);
    }

    bb
}

fn southeast_ray(sq: usize) -> u64 {
    let mut bb = 0u64;
    let sq_rank = sq / 8;
    let sq_file = sq % 8;

    let mut r = sq_rank;
    let mut f = sq_file;
    while r > 0 && f < 7 {
        r -= 1;
        f += 1;
        bb |= 1 << (r*8 + f);
    }

    bb
}

fn southwest_ray(sq: usize) -> u64 {
    let mut bb = 0u64;
    let sq_rank = sq / 8;
    let sq_file = sq % 8;

    let mut r = sq_rank;
    let mut f = sq_file;
    while r > 0 && f > 0 {
        r -= 1;
        f -= 1;
        bb |= 1 << (r*8 + f);
    }

    bb
}

fn northwest_ray(sq: usize) -> u64 {
    let mut bb = 0u64;
    let sq_rank = sq / 8;
    let sq_file = sq % 8;

    let mut r = sq_rank;
    let mut f = sq_file;
    while r < 7 && f > 0 {
        r += 1;
        f -= 1;
        bb |= 1 << (r*8 + f);
    }

    bb
}

fn create_attacks_file () {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("attacks.rs");
    let mut contents = String::new();

    contents.push_str("pub const PAWN_ATTACKS: [[u64; 2]; 64] = [");
    for sq in 0..64 {
        contents.push_str(&format!("[{},{}],", white_pawn_attacks(sq), black_pawn_attacks(sq)));
    }
    contents.push_str("];");

    contents.push_str("pub const KNIGHT_ATTACKS: [u64; 64] = [");
    for sq in 0..64 {
        contents.push_str(&format!("{},", knight_attacks(sq)));
    }
    contents.push_str("];");

    contents.push_str("pub const KING_ATTACKS: [u64; 64] = [");
    for sq in 0..64 {
        contents.push_str(&format!("{},", king_attacks(sq)));
    }
    contents.push_str("];");

    contents.push_str("pub const NORTH_RAYS: [u64; 64] = [");
    for sq in 0..64 {
        contents.push_str(&format!("{},", north_ray(sq)));
    }
    contents.push_str("];");

    contents.push_str("pub const EAST_RAYS: [u64; 64] = [");
    for sq in 0..64 {
        contents.push_str(&format!("{},", east_ray(sq)));
    }
    contents.push_str("];");

    contents.push_str("pub const SOUTH_RAYS: [u64; 64] = [");
    for sq in 0..64 {
        contents.push_str(&format!("{},", south_ray(sq)));
    }
    contents.push_str("];");

    contents.push_str("pub const WEST_RAYS: [u64; 64] = [");
    for sq in 0..64 {
        contents.push_str(&format!("{},", west_ray(sq)));
    }
    contents.push_str("];");

    contents.push_str("pub const NORTHEAST_RAYS: [u64; 64] = [");
    for sq in 0..64 {
        contents.push_str(&format!("{},", northeast_ray(sq)));
    }
    contents.push_str("];");

    contents.push_str("pub const SOUTHEAST_RAYS: [u64; 64] = [");
    for sq in 0..64 {
        contents.push_str(&format!("{},", southeast_ray(sq)));
    }
    contents.push_str("];");

    contents.push_str("pub const SOUTHWEST_RAYS: [u64; 64] = [");
    for sq in 0..64 {
        contents.push_str(&format!("{},", southwest_ray(sq)));
    }
    contents.push_str("];");

    contents.push_str("pub const NORTHWEST_RAYS: [u64; 64] = [");
    for sq in 0..64 {
        contents.push_str(&format!("{},", northwest_ray(sq)));
    }
    contents.push_str("];");

    fs::write(dest_path, &contents).unwrap();
}

fn create_zobrist_file() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("zobrist.rs");
    let mut contents = String::new();

    contents.push_str("pub const PIECES: [[[u64; 64]; 6]; 2] = [");
    for _colour in 0..2 {
        contents.push('[');
        for _piece in 0..6 {
            contents.push('[');
            for _sq in 0..64 {
                let rand: u64 = random();
                contents.push_str(&format!("{},", rand));
            }
            contents.push_str("],");
        }
        contents.push_str("],");
    }
    contents.push_str("];");
    
    contents.push_str("pub const CASTLING: [u64; 16] = [");
    for _ in 0..16 {
        let rand: u64 = random();
        contents.push_str(&format!("{},", rand));
    }
    contents.push_str("];");

    contents.push_str("pub const EP_FILE: [u64; 8] = [");
    for _ in 0..8 {
        let rand: u64 = random();
        contents.push_str(&format!("{},", rand));
    }
    contents.push_str("];");

    let rand: u64 = random();
    contents.push_str(&format!("pub const WHITE_MOVE: u64 = {};", rand));

    fs::write(dest_path, &contents).unwrap();
}

fn main() {
    create_attacks_file();
    create_zobrist_file();
}