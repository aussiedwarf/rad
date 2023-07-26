# RAD
Rust Advanced Design


## Requirements
For wasm, need to install emscripten.
`rustup target add wasm32-unknown-emscripten`

To compile wasm
`cargo build --target wasm32-unknown-emscripten`

To debug wasm.
`EMCC_CFLAGS="-g -gsource-map" cargo build --target wasm32-unknown-emscripten`

To compile generally
```
cargo build
cargo build --release
```
