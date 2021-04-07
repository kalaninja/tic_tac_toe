#[macro_use]
extern crate derive_error;

pub use game::Game;
pub use player::Player;
pub use tic_tac_toe::TicTacToe;

mod player;
mod tic_tac_toe;
mod error;
mod game;
mod guard;

