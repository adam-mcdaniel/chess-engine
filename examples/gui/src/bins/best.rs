use gui::{run, best_move, Board};

fn main() -> iced::Result {
    run(best_move, Board::default())
}
