#[derive(Eq, PartialEq, Debug, Error)]
pub enum Error {
    /// Specified position is outside of the board
    PositionIsOutsideBoard,
    /// The cell is not empty
    CellIsNotEmpty,
    /// The game is over
    GameOver,
}
