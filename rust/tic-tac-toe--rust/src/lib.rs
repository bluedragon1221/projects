use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameAction {
    #[error("A number larger than the size of the board was used to index it")]
    IndexOutOfBounds,
    #[error("'{0}' is not a valid hint. Use a provided hint (or 'h' for help).")]
    InvalidHint(char),
    #[error("It looks like that square is already taken! Try another one.")]
    CellOccupied,
    #[error("Something went wrong when parsing the user input")]
    UserInputError(#[from] std::io::Error),
    #[error("Printing the hint")]
    PrintHint,
    // #[error("Error when formatting the board into a String")]
    // FormattingError,
}
