use crate::state::State;
use crate::helpers::algebraic_to_sq;
use crate::moves::{BitMove, encode_move, move_from, move_to, move_promotion_piece, generate_moves};
use crate::pieces::Piece;
use std::str::SplitWhitespace;

pub struct InvalidUciCommandError;

pub fn parse_position_command(command: &str) -> Result<State, InvalidUciCommandError> {
    let mut segments = command.split_whitespace().skip(1);
    let mut state = match segments.next() {
        Some("startpos") => State::start_pos(),
        Some("fen") => {
            let mut fen = String::new();
            for i in 0..6 {
                match segments.next() {
                    Some(s) => fen.push_str(&(" ".to_owned() + s)),
                    None => {
                        return Err(InvalidUciCommandError);
                    }
                }
            }
            fen = fen.trim().to_string();
            match State::from_fen(&fen) {
                Ok(s) => s,
                Err(_) => {
                    return Err(InvalidUciCommandError);
                }
            }
        },
        _ => {
            return Err(InvalidUciCommandError);
        }
    };

    if let Some("moves") = segments.next() {
        loop {
            match segments.next() {
                Some(move_string) => {
                    if move_string.len() < 4 {
                        return Err(InvalidUciCommandError);
                    }

                    let mut move_list = generate_moves(&state);

                    let from = algebraic_to_sq(&move_string[0..2]);
                    let to = algebraic_to_sq(&move_string[2..4]);
                    let promotion_piece = if move_string.len() == 5 {
                        match &move_string[4..5] {
                            "n" => Some(Piece::Knight),
                            "b" => Some(Piece::Bishop),
                            "r" => Some(Piece::Rook),
                            "q" => Some(Piece::Queen),
                            _ => {
                                return Err(InvalidUciCommandError);
                            }
                        }
                    } else { None };

                    while !move_list.is_empty() {
                        let r#move = move_list.pop();
                        if from == move_from(r#move) && to == move_to(r#move) && promotion_piece == move_promotion_piece(r#move) {
                            state.make_move(r#move);
                            break;
                        }
                    }
                },
                None => break
            }
        }
    }

    Ok(state)
}

