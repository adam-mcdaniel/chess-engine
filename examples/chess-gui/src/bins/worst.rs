use chess_gui::{run, worst_move, Board};

fn main() -> iced::Result {
    run(worst_move, Board::default())
}
