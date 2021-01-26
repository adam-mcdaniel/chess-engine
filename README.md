# chess-engine

A pure Rust, dependency-free chess engine built to run anywhere.

## Embedded in the Web

Because it has zero dependencies, it's extremely simple to embed in the web browser using wasm. [Try playing it yourself!](https://adam-mcdaniel.github.io/chess-engine/docs/book/index.html)

## Example

```rust
fn main() {
    println!("Waiting for CPU to move...");
    // Generate a move with 4 moves of lookahead
    let cpu_move = board.get_best_next_move(4);

    // alternatively, choose the worst move with 4 moves of lookahead
    let cpu_move = board.get_worst_next_move(4);


    print!("CPU chose to ");
    match cpu_move {
        Move::Piece(from, to) => {
            println!("move {} to {}", from, to)
        }
        Move::KingSideCastle => {
            println!("castle kingside")
        }
        Move::QueenSideCastle => {
            println!("castle queenside")
        }
        Move::Resign => println!("resign")
    }

    match board.play_move(cpu_move) {
        GameResult::Continuing(next_board) => {
            board = next_board;
            println!("{}", board);
        }

        GameResult::Victory(winner) => {
            println!("{} loses. {} is victorious.", !winner, winner);
        }

        GameResult::IllegalMove(x) => {
            eprintln!("{} is an illegal move.", x);
        }

        GameResult::Stalemate => {
            println!("Drawn game.");
        }
    }
}

```