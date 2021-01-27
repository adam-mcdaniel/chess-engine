use chess_gui::{run, best_move, Board};

fn main() -> iced::Result {
    run(best_move, Board::horde())
}
