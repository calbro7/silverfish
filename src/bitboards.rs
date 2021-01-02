use crate::helpers::rank_file_to_sq;

pub fn set_bit(bb: u64, sq: usize) -> u64 {
    bb | (1 << sq)
}

pub fn clear_bit(bb: u64, sq: usize) -> u64 {
    bb & !(1 << sq)
}

pub fn get_bit(bb: u64, sq: usize) -> bool {
    bb & (1 << sq) != 0
}

pub fn count_bits(bb: u64) -> usize {
    bb.count_ones() as usize
}

pub fn get_ls1b(bb: u64) -> usize {
    bb.trailing_zeros() as usize
}

pub fn pop_ls1b(bb: &mut u64) -> usize {
    let sq = get_ls1b(*bb);
    *bb = clear_bit(*bb, sq);
    sq
}

pub fn from_sq(sq: usize) -> u64 {
    1 << sq
}

pub const RANK_1: u64 = 0b0000000000000000000000000000000000000000000000000000000011111111;
pub const RANK_4: u64 = 0b0000000000000000000000000000000011111111000000000000000000000000;
pub const RANK_5: u64 = 0b0000000000000000000000001111111100000000000000000000000000000000;
pub const RANK_8: u64 = 0b1111111100000000000000000000000000000000000000000000000000000000;

#[allow(dead_code)]
pub fn print_bb(bb: u64) {
    let mut output = String::new();

    for rank in (0..8).rev() {
        output.push('|');
        for file in 0..8 {
            let sq = rank_file_to_sq(rank, file);
            output.push(if get_bit(bb, sq) {'1'} else {'0'});
            output.push('|');
        }
        output.push('\n');
    }

    println!("{}", output)
}