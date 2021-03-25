use crate::moves::{BitMove};
use crate::state::State;
use crate::pieces::Piece;
use crate::moves::{generate_moves, move_from, move_to, move_promotion_piece};
use crate::helpers::algebraic_to_sq;
use std::collections::HashMap;
use lazy_static::lazy_static;
use serde_yaml;

fn parse_mapping(state: &mut State, mapping: serde_yaml::Mapping, book: &mut HashMap<u64, Vec<BitMove>>) {
    let copy = state.clone();

    for v in mapping.into_iter() {
        let moves = generate_moves(&state);

        match v.0 {
            serde_yaml::Value::String(s) => {
                let from = algebraic_to_sq(&s[0..2]);
                let to = algebraic_to_sq(&s[2..4]);
                let promotion_piece = if s.len() == 5 {
                    match &s[4..5] {
                        "n" => Some(Piece::Knight),
                        "b" => Some(Piece::Bishop),
                        "r" => Some(Piece::Rook),
                        "q" => Some(Piece::Queen),
                        _ => {
                            return;
                        }
                    }
                } else { None };

                let mut found = false;
                for r#move in moves {
                    if from == move_from(r#move) && to == move_to(r#move) && promotion_piece == move_promotion_piece(r#move) {
                        found = true;

                        let entry = book.entry(state.hash).or_insert(vec![]);
                        if !entry.contains(&r#move) {
                            entry.push(r#move);
                        }

                        state.make_move(r#move).unwrap();
                        match v.1 {
                            serde_yaml::Value::Mapping(m) => {
                                parse_mapping(state, m, book);
                            },
                            serde_yaml::Value::Null => {},
                            _ => panic!()
                        }
                        *state = copy;
                        break;
                    }
                }
                if !found {
                    panic!("Illegal move {} in\n{}", s, state)
                }
            },
            _ => panic!()
        };
    }
}

lazy_static! {
    pub static ref BOOK: HashMap<u64, Vec<BitMove>> = {
        let mut state = State::start_pos();
        let mut book: HashMap<u64, Vec<BitMove>> = HashMap::new();

        let f = include_str!("book.yml");
        let data: serde_yaml::Mapping = serde_yaml::from_str(f).unwrap();
        parse_mapping(&mut state, data, &mut book);
        book
    };
}