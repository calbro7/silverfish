use std::env;
use std::fs;
use std::path::Path;

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

fn main () {
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

    fs::write(dest_path, &contents).unwrap();
}