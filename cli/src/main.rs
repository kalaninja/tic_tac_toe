use engine::{Game, Player, TicTacToe};
use std::io;
use itertools::Itertools;
use std::io::Write;

fn game_over(game: &Game) -> bool {
    game.board_is_full() || game.winner() != Player::None
}

fn read_move() -> Option<(u32, u32)> {
    print!("Make your move (row-column) [e.g. 0-0]:");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim()
        .split('-')
        .flat_map(|x| x.parse::<u32>())
        .next_tuple()
}

fn main() {
    let mut game = Game::new();

    loop {
        println!("{}", game);
        if game_over(&game) { break; }

        loop {
            if let Some((row, column)) = read_move() {
                let result = game.make_move(row, column);
                match result {
                    Ok(_) => break,
                    Err(e) => println!("{}", e),
                }
            } else {
                println!("Invalid input");
            }
        }
    }
}
