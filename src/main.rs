mod helpers;
mod bitboards;
mod colours;
mod attacks;
mod castling;
mod state;
mod pieces;
mod moves;
mod uci;
mod errors;
mod perft;
use text_io::read;

fn main() {
    let mut state = state::State::new();

    loop {
        let command: String = read!("{}\n");
        
        if command.starts_with("isready") {
            println!("readyok");
        }
        else if command.starts_with("ucinewgame") {
            state = state::State::start_pos();
        }
        else if command.starts_with("position") {
            match uci::parse_position_command(&command) {
                Ok(s) => {
                    state = s;
                },
                _ => {}
            };
        }
        else if command.starts_with("go") {
            let mut moves = moves::generate_moves(&state);
            while !moves.is_empty() {
                let r#move = moves.pop();
                match state.make_move(r#move) {
                    Ok(_) => {
                        println!("bestmove {}{}", helpers::sq_to_algebraic(moves::move_from(r#move)), helpers::sq_to_algebraic(moves::move_to(r#move)));
                        break
                    },
                    Err(_) => {}
                };
            }
        }
        else if command.starts_with("uci") {
            println!("id silverfish");
            println!("uciok");
        }
        else if command.starts_with("quit") {
            break;
        }
    }
}