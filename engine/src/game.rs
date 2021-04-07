use std::fmt;

use crate::error::Error;
use crate::player::Player;
use crate::tic_tac_toe::TicTacToe;
use crate::guard::Guard;

#[derive(Clone)]
pub struct Game {
    board: [[Player; 3]; 3],
    current_player: Player,
    winner: Player,
}

impl Game {
    pub fn board_is_full(&self) -> bool {
        self.board.iter()
            .flatten()
            .all(|&x| x != Player::None)
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: [[Player::None; 3]; 3],
            current_player: Player::X,
            winner: Player::None,
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.board.iter() {
            writeln!(f, "{} {} {}", row[0], row[1], row[2])?
        }

        if self.winner != Player::None {
            writeln!(f, "Winner: {}", self.winner)
        } else if self.board_is_full() {
            writeln!(f, "Draw")
        } else {
            writeln!(f, "Current player: {}", self.current_player)
        }
    }
}

impl TicTacToe for Game {
    type Player = Player;
    type Error = Error;

    fn new() -> Self {
        Self::default()
    }

    fn make_move(&mut self, row: u32, column: u32) -> Result<(), Self::Error> {
        fn switch_current_player(game: &mut Game) {
            game.current_player = match game.current_player {
                Player::X => Player::O,
                Player::O => Player::X,
                _ => unreachable!()
            }
        }

        fn check_current_player_has_won(game: &Game, row: usize, column: usize) -> bool {
            // horizontal
            game.board[row].iter().all(|&x| x == game.current_player) ||
                // vertical
                (0..=2).all(|x| game.board[x][column] == game.current_player) ||
                // main diagonal
                ((row == column) && (0..=2).all(|x| game.board[x][x] == game.current_player)) ||
                // antidiagonal
                ((row == 2 - column) && (0..=2).all(|x| game.board[x][2 - x] == game.current_player))
        }

        Guard::validate_board_position(row, column)?;
        if self.board_is_full() || self.winner != Player::None {
            return Err(Error::GameOver);
        }

        let row_usize = row as usize;
        let column_usize = column as usize;
        let cell = &mut self.board[row_usize][column_usize];
        if *cell != Player::None {
            Err(Error::CellIsNotEmpty)
        } else {
            *cell = self.current_player;
            if check_current_player_has_won(self, row_usize, column_usize) {
                self.winner = self.current_player;
            }

            switch_current_player(self);
            Ok(())
        }
    }

    fn current_player(&self) -> Self::Player {
        self.current_player
    }

    fn winner(&self) -> Self::Player {
        self.winner
    }

    fn get_board_position(&self, row: u32, column: u32) -> Result<Self::Player, Self::Error> {
        Guard::validate_board_position(row, column)?;

        Ok(self.board[row as usize][column as usize])
    }

    fn reset(&mut self) {
        *self = Self::default()
    }
}

// Write some unit tests for the TicTacToe game to exhibit basic game functionality.
#[cfg(test)]
mod tests {
    use crate::game::Game;
    use crate::tic_tac_toe::TicTacToe;
    use crate::player::Player;
    use crate::error::Error;

    macro_rules! make_moves {
        ($game:expr, $($move:expr),+) => {
            $(assert_eq!($game.make_move($move.0, $move.1), Ok(()));)+
        }
    }

    trait GameTestSuit {
        fn board_is_empty(&self) -> bool;
    }

    impl GameTestSuit for Game {
        fn board_is_empty(&self) -> bool {
            self.board.iter()
                .flat_map(|x| x)
                .all(|&x| x == Player::None)
        }
    }

    #[test]
    fn can_create_game() {
        let game = Game::new();

        assert_eq!(game.current_player(), Player::X);
        assert_eq!(game.winner(), Player::None);
        assert_eq!(game.board_is_empty(), true);
    }

    #[test]
    fn x_starts_first() {
        let game = Game::new();

        assert_eq!(game.current_player(), Player::X);
    }

    #[test]
    fn player_switches_after_move() {
        let mut game = Game::new();

        make_moves!(game, (1, 1));

        assert_eq!(game.current_player(), Player::O);
    }

    #[test]
    fn move_saves() {
        let mut game = Game::new();

        make_moves!(game, (1, 1));

        assert_eq!(game.get_board_position(1, 1), Ok(Player::X));
    }

    #[test]
    fn cannot_make_same_move_twice() {
        let mut game = Game::new();
        make_moves!(game, (1, 1));

        assert_eq!(game.make_move(1, 1), Err(Error::CellIsNotEmpty));
    }

    #[test]
    fn cannot_make_move_outside_board() {
        let mut game = Game::new();
        assert_eq!(game.make_move(3, 3), Err(Error::PositionIsOutsideBoard));
    }

    #[test]
    fn can_win_horizontal() {
        let mut game = Game::new();
        make_moves!(game, (0, 1), (1, 1), (0, 0), (1, 0), (0, 2));

        assert_eq!(game.winner(), Player::X);
    }

    #[test]
    fn can_win_vertical() {
        let mut game = Game::new();
        make_moves!(game, (0, 0), (0, 2), (0, 1), (1, 2), (1, 1), (2, 2));

        assert_eq!(game.winner(), Player::O);
    }

    #[test]
    fn can_win_main_diagonal() {
        let mut game = Game::new();
        make_moves!(game, (0, 0), (0, 2), (1, 1), (1, 2), (2, 2));

        assert_eq!(game.winner(), Player::X);
    }

    #[test]
    fn can_win_antidiagonal() {
        let mut game = Game::new();
        make_moves!(game, (0, 0), (0, 2), (0, 1), (2, 0), (1, 0), (1, 1));

        assert_eq!(game.winner(), Player::O);
    }

    #[test]
    fn can_draw() {
        let mut game = Game::new();
        make_moves!(game, (1, 1), (0, 0), (1, 2), (1, 0), (2, 0), (0, 2), (0, 1), (2, 1), (2, 2));

        assert_eq!(game.make_move(0, 1), Err(Error::GameOver));
        assert_eq!(game.winner(), Player::None);
    }

    #[test]
    fn cannot_make_move_when_game_is_over() {
        let mut game = Game::new();
        make_moves!(game, (0, 0), (0, 2), (1, 1), (1, 2), (2, 2));
        assert_eq!(game.winner(), Player::X);

        assert_eq!(game.make_move(0, 1), Err(Error::GameOver));
    }

    #[test]
    fn cannot_get_board_position_outside() {
        let game = Game::new();

        assert_eq!(game.get_board_position(3, 0), Err(Error::PositionIsOutsideBoard));
    }

    #[test]
    fn can_reset_game() {
        let mut game = Game::new();
        make_moves!(game, (0, 0), (0, 2), (1, 1), (1, 2), (2, 2));
        assert_eq!(game.winner(), Player::X);

        game.reset();

        assert_eq!(game.get_board_position(1, 1), Ok(Player::None));
        assert_eq!(game.current_player(), Player::X);
        assert_eq!(game.winner(), Player::None);
        assert_eq!(game.board_is_empty(), true);
    }

    #[test]
    fn ongoing_game_displays_board_and_current_player() {
        let mut game = Game::new();
        assert_eq!(game.make_move(1, 1), Ok(()));

        let actual = format!("{}", game);
        let expected = "* * *\n* X *\n* * *\nCurrent player: O\n";
        assert_eq!(actual, expected);
    }

    #[test]
    fn win_game_displays_board_and_winner() {
        let mut game = Game::new();
        make_moves!(game, (0, 0), (0, 2), (1, 1), (1, 2), (2, 2));

        let actual = format!("{}", game);
        let expected = "X * O\n* X O\n* * X\nWinner: X\n";
        assert_eq!(actual, expected);
    }

    #[test]
    fn draw_game_displays_board_and_draw() {
        let mut game = Game::new();
        make_moves!(game, (1, 1), (0, 0), (1, 2), (1, 0), (2, 0), (0, 2), (0, 1), (2, 1), (2, 2));

        let actual = format!("{}", game);
        let expected = "O X O\nO X X\nX O X\nDraw\n";
        assert_eq!(actual, expected);
    }
}
