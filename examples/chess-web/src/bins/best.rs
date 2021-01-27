use chess_web::{run, best_move, Board};

fn main() -> iced::Result {
    run(best_move, Board::default())
}
