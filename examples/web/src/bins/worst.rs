use chess_web::{run, worst_move, Board};

fn main() -> iced::Result {
    run(worst_move, Board::default())
}
