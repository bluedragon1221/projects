use console::Term;

mod check_win;
mod format;
use format::{format_board, HINT};

use tic_tac_toe::GameAction;

fn hint_to_coords(hint: char) -> Result<(usize, usize), GameAction> {
    match hint {
        'q' => Ok((0, 0)),
        'w' => Ok((0, 1)),
        'e' => Ok((0, 2)),
        'a' => Ok((1, 0)),
        's' => Ok((1, 1)),
        'd' => Ok((1, 2)),
        'z' => Ok((2, 0)),
        'x' => Ok((2, 1)),
        'c' => Ok((2, 2)),
        'h' => Err(GameAction::PrintHint),
        a => Err(GameAction::InvalidHint(a)),
    }
}

fn play_round(term: &Term, board: &mut Board) -> Result<(), GameAction> {
    let hint = term.read_char()?;
    let coords = hint_to_coords(hint)?;
    board.play(coords)?;

    println!("{}", format_board(&board));

    if let Some(a) = check_win::check_win(&board) {
        match a {
            check_win::WinState::XWins => println!("X Wins!"),
            check_win::WinState::OWins => println!("O Wins!"),
            check_win::WinState::Tie => println!("It's a tie!"),
        };
        std::process::exit(0)
    };

    board.cur_player *= -1;

    Ok(())
}

fn main() {
    let mut board = Board::default();
    let term = Term::stdout();

    println!("{}", HINT);
    println!("Use the hint to select where you want to play");
    loop {
        match play_round(&term, &mut board) {
            Err(GameAction::InvalidHint(a)) => println!("{}", GameAction::InvalidHint(a)),
            Err(GameAction::CellOccupied) => println!("{}", GameAction::CellOccupied),
            Err(GameAction::PrintHint) => println!("{}", HINT),
            Err(error) => panic!("An error occured: {}", error),
            Ok(()) => (),
        };
    }
}
