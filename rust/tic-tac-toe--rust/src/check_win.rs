use crate::Board;

pub enum WinState {
    XWins,
    Tie,
    OWins,
}

pub fn check_win(board: &Board) -> Option<WinState> {
    let win_states = get_win_states(board);

    match win_states {
        a if a.contains(&3) => Some(WinState::XWins),
        a if a.contains(&-3) => Some(WinState::OWins),
        a if !a.contains(&0) => Some(WinState::Tie),
        _ => None,
    }
}

fn get_win_states(board: &Board) -> [i8; 8] {
    [
        // rows
        board.board[0][0] + board.board[0][1] + board.board[0][2],
        board.board[1][0] + board.board[1][1] + board.board[1][2],
        board.board[2][0] + board.board[2][1] + board.board[2][2],
        // columns
        board.board[0][0] + board.board[1][0] + board.board[2][0],
        board.board[0][1] + board.board[1][1] + board.board[2][1],
        board.board[0][2] + board.board[1][2] + board.board[2][2],
        // diagonals
        board.board[0][0] + board.board[1][1] + board.board[2][2],
        board.board[0][2] + board.board[1][1] + board.board[2][0],
    ]
}
