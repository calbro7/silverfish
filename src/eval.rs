use crate::bitboards::{count_bits, pop_ls1b};
use crate::colours::Colour;
use crate::pieces::Piece;
use crate::state::State;
use crate::helpers::mirror_sq;

const SQ_VALS: [[isize; 64]; 6] = [
    [
         0,  0,  0,  0,  0,  0,  0,  0,
         5, 10, 10,-20,-20, 10, 10,  5,
         5, -5,-10,  0,  0,-10, -5,  5,
         0,  0,  0, 20, 20,  0,  0,  0,
         5,  5, 10, 25, 25, 10,  5,  5,
        10, 10, 20, 30, 30, 20, 10, 10,
        50, 50, 50, 50, 50, 50, 50, 50,
         0,  0,  0,  0,  0,  0,  0,  0
    ],
    [
        -50,-40,-30,-30,-30,-30,-40,-50,
        -40,-20,  0,  5,  5,  0,-20,-40,
        -30,  5, 10, 15, 15, 10,  5,-30,
        -30,  0, 15, 20, 20, 15,  0,-30,
        -30,  5, 15, 20, 20, 15,  5,-30,
        -30,  0, 10, 15, 15, 10,  0,-30,
        -40,-20,  0,  0,  0,  0,-20,-40,
        -50,-40,-30,-30,-30,-30,-40,-50
    ],
    [
        -20,-10,-10,-10,-10,-10,-10,-20,
        -10,  5,  0,  0,  0,  0,  5,-10,
        -10, 10, 10, 10, 10, 10, 10,-10,
        -10,  0, 10, 10, 10, 10,  0,-10,
        -10,  5,  5, 10, 10,  5,  5,-10,
        -10,  0,  5, 10, 10,  5,  0,-10,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -20,-10,-10,-10,-10,-10,-10,-20
    ],
    [
         0,  0,  0,  5,  5,  0,  0,  0,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
         5, 10, 10, 10, 10, 10, 10,  5,
         0,  0,  0,  0,  0,  0,  0,  0
    ],
    [
        -20,-10,-10, -5, -5,-10,-10,-20,
        -10,  0,  5,  0,  0,  5,  0,-10,
        -10,  5,  5,  5,  5,  5,  5,-10,
          0,  0,  5,  5,  5,  5,  0,  0,
         -5,  0,  5,  5,  5,  5,  0, -5,
        -10,  0,  5,  5,  5,  5,  0,-10,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -20,-10,-10, -5, -5,-10,-10,-20
    ],
    [
         20, 30, 10,  0,  0, 10, 30, 20,
         20, 20,  0,  0,  0,  0, 20, 20,
        -10,-20,-20,-20,-20,-20,-20,-10,
        -20,-30,-30,-40,-40,-30,-30,-20,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30
    ]
];

fn piece_val(piece: &Piece, colour: &Colour) -> isize {
    (match piece {
        Piece::Pawn => 100,
        Piece::Knight => 320,
        Piece::Bishop => 330,
        Piece::Rook => 500,
        Piece::Queen => 900,
        Piece::King => 20000
    }) * (match colour {
        Colour::White => 1,
        Colour::Black => -1
    })
}

fn piece_sq_val(piece: &Piece, colour: &Colour, sq: usize) -> isize {
    match colour {
        Colour::White => SQ_VALS[*piece as usize][sq],
        Colour::Black => SQ_VALS[*piece as usize][mirror_sq(sq)] * -1
    }
}

pub fn eval(state: &State) -> isize {
    let mut score = 0isize;

    for piece in &[Piece::Pawn, Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen, Piece::King] {
        for colour in &[Colour::White, Colour::Black] {
            let mut bb = state.pieces[*piece as usize] & state.colours[*colour as usize];
            score += piece_val(piece, colour) * count_bits(bb) as isize;
            while bb !=0 {
                score += piece_sq_val(piece, colour, pop_ls1b(&mut bb));
            }
        }
    }
    
    score
}

pub fn relative_eval(state: &State) -> isize {
    match state.to_move {
        Colour::White => eval(&state),
        Colour::Black => -1 * eval(&state)
    }
}