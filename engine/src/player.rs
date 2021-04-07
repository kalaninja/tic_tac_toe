use core::fmt;
use std::fmt::Write;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Player {
    /// empty/tie
    None,
    /// X player
    X,
    /// O player
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::None => f.write_char('*'),
            Player::X => f.write_char('X'),
            Player::O => f.write_char('O'),
        }
    }
}
