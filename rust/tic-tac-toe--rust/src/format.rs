use crate::Board;

pub const HINT: &str = "q | w | e
--|---|--
a | s | d
--|---|--
z | x | c";

pub fn format_board(board: &Board) -> String {
    let char_board = char_board(board);
    format!(
        "{} | {} | {}
--|---|--
{} | {} | {}
--|---|--
{} | {} | {}",
        char_board[0],
        char_board[1],
        char_board[2],
        char_board[3],
        char_board[4],
        char_board[5],
        char_board[6],
        char_board[7],
        char_board[8],
    )
}

fn char_board(board: &Board) -> Vec<char> {
    let flat_board: Vec<&i8> = board.board.iter().flatten().collect();

    flat_board
        .into_iter()
        .map(|a| match a {
            -1 => 'O',
            0 => ' ',
            1 => 'X',
            _ => unreachable!(),
        })
        .collect::<Vec<char>>()
}
