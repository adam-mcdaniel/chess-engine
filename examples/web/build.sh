#!/bin/bash
mkdir wasm

cargo build --bin chess-best --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/chess-best.wasm --out-dir ./wasm/chess-best --web

cargo build --bin chess-worst --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/chess-worst.wasm --out-dir ./wasm/chess-worst --web

cargo build --bin chess-random --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/chess-random.wasm --out-dir ./wasm/chess-random --web

cargo build --bin chess-horde --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/chess-horde.wasm --out-dir ./wasm/chess-horde --web