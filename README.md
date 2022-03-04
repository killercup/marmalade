# Jam Bevy Jam

This is a [Bevy](https://bevyengine.org/) game for the first [Bevy Jam](https://itch.io/jam/bevy-jam-1).

## Setup

- [Install Rust](https://rustup.rs/)
- Use VSCode with "rust analyzer" extension

## Start developing

1. Run `cargo start` in some terminal, e.g. that of VSCode.
   This will compile for a hot minute the first time you run it.
2. Edit code
3. See either beautiful game window or long error messages.
4. Go to 2

## Build for WASM

Tools: `cargo install wasm-bindgen`, `brew install binaryen`

Build:

```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --web --out-dir=. $CARGO_TARGET_DIR/wasm32-unknown-unknown/release/marmalade.wasm
wasm-opt -Oz -o marmalade_bg.wasm marmalade_bg.wasm
```

- Try `opt-level = "z"` or `s`
- Make sure to copy `assets/` next to `index.html`
