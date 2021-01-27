# chess-web

This is an example implementation of my chess engine using a GUI in the web.

![Chess-Web](../../assets/web.png)

## Usage

Here's the shell commands for setting up and building my chess engine for the web.

```bash
# Add the WASM target
rustup target add wasm32-unknown-unknown
# Install wasm-bindgen for compiling a crate to WASM
cargo install wasm-bindgen-cli

git clone https://github.com/adam-mcdaniel/chess-engine
cd chess-engine/examples/chess-web

# Create a directory for the WASM outputs
mkdir wasm

# Build the WASM using the best move algorithm in `wasm/chess-best/`
cargo build --bin chess-best --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/chess-best.wasm --out-dir ./wasm/chess-best --web

# Build the WASM using the worst move algorithm in `wasm/chess-worst/`
cargo build --bin chess-worst --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/chess-worst.wasm --out-dir ./wasm/chess-worst --web

# Build the WASM using the random move algorithm in `wasm/chess-random/`
cargo build --bin chess-random --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/chess-random.wasm --out-dir ./wasm/chess-random --web

# Build the WASM using the horde variant in `wasm/chess-horde/`
cargo build --bin chess-horde --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/chess-horde.wasm --out-dir ./wasm/chess-horde --web
```

You can then run the engine in the browser with the following HTML code:
```html
<!DOCTYPE html>
<html>
  <head>
    <meta http-equiv="Content-type" content="text/html; charset=utf-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Chess</title>
  </head>
  <body>
    <script type="module">
      import init from "./wasm/chess-best/chess-best.js";

      init('./wasm/chess-best/chess-best_bg.wasm');
    </script>
  </body>
</html>
```

Or, if you want to embed my chess engine in your website, you can do so with the following code:

```html
<embed type="text/html" src="https://adam-mcdaniel.github.io/chess-engine/examples/chess-web/chess-best.html" width="420" height="420"/>
```