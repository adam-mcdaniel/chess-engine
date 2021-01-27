use chess_gui::{run, random_move, Board};

fn main() -> iced::Result {
    run(random_move, Board::default())
}
