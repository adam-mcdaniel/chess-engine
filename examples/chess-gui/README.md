# chess-gui

This is an example implementation of my chess engine using a desktop GUI.

![Chess-GUI](../../assets/gui.png)

## Usage

```bash
git clone https://github.com/adam-mcdaniel/chess-engine
cd chess-engine/examples/chess-gui

# Run the AI using the best move algorithm
cargo run --release --bin best

# Run the AI using the worst move algorithm
cargo run --release --bin worst

# Run the AI using the random move algorithm
cargo run --release --bin random

# Play the horde variant
cargo run --release --bin horde
```