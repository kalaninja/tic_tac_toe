use crate::error::Error;

pub(crate) struct Guard;

impl Guard {
    pub(crate) fn validate_board_position(row: u32, column: u32) -> Result<(), Error> {
        if row > 2 || column > 2 {
            Err(Error::PositionIsOutsideBoard)
        } else {
            Ok(())
        }
    }
}