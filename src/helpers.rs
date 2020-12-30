pub fn rank_file_to_sq(rank: usize, file: usize) -> usize {
    (rank * 8) + file
}

pub fn sq_rank(sq: usize) -> usize {
    sq / 8
}

pub fn sq_file(sq: usize) -> usize {
    sq % 8
}

pub fn mirror_sq(sq: usize) -> usize {
    rank_file_to_sq(7 - sq_rank(sq), sq_file(sq))
}

pub fn algebraic_to_sq(algebraic: &str) -> usize {
    let mut chars = algebraic.chars();

    let file = match chars.next().unwrap() {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => panic!("Invalid square")
    };
    let rank = match chars.next().unwrap() {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        _ => panic!("Invalid square")
    };

    rank_file_to_sq(rank, file)
}

pub fn sq_to_algebraic(sq: usize) -> String {
    let rank_char = match sq_rank(sq) {
        0 => '1',
        1 => '2',
        2 => '3',
        3 => '4',
        4 => '5',
        5 => '6',
        6 => '7',
        7 => '8',
        _ => panic!("Invalid square")
    };
    let file_char = match sq_file(sq) {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => panic!("Invalid square")
    };

    format!("{}{}", file_char, rank_char)
}