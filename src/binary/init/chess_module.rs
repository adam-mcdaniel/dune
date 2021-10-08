use dune::{Environment, Error, Expression};

pub fn chess_fn(args: Vec<Expression>, env: &mut Environment) -> Result<Expression, Error> {
    let mut won = false;

    let player_color = match &args[0] {
        Expression::String(color) | Expression::Symbol(color) if color == "white" => {
            chess_engine::WHITE
        }
        Expression::String(color) | Expression::Symbol(color) if color == "black" => {
            chess_engine::BLACK
        }
        _ => {
            return Err(Error::CustomError(
                "call chess with a color, like \"black\" or \"white\"".to_string(),
            ))
        }
    };

    let mut board = chess_engine::Board::default();
    let mut history = vec![];
    loop {
        use chess_engine::Evaluate;
        use std::convert::TryFrom;

        let m = if player_color != board.get_turn_color() {
            println!("Waiting for CPU to annihilate your position...");
            board.get_best_next_move(4).0
        } else {
            println!("Your move!\n{}", board);
            let mut rl = crate::new_editor(env);
            let mut s = crate::readline("Enter move: ", &mut rl);
            s = s.trim().to_string();

            if s.is_empty() {
                eprintln!("That's not a move!");
                continue;
            } else if s == "q" || s == "quit" || s == "exit" {
                println!("Bye!");
                break;
            } else {
                match chess_engine::Move::try_from(s) {
                    Ok(m) => m,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                }
            }
        };

        match board.play_move(m) {
            chess_engine::GameResult::Continuing(next_board) => {
                board = next_board;
                history.push(m);
            }

            chess_engine::GameResult::Victory(winner) => {
                println!("{}", board);
                println!("Checkmate! {} loses. {} is victorious!", !winner, winner);

                won = player_color == winner;
                break;
            }

            chess_engine::GameResult::IllegalMove(x) => {
                eprintln!("{} is an illegal move.", x);
            }

            chess_engine::GameResult::Stalemate => {
                println!("Drawn game.");
                break;
            }
        }
    }

    Ok(Expression::Boolean(won))
}

pub const HELP: &str = "a fun builtin function for playing chess!";
