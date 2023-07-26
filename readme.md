# RAD
Rust Advanced Design


## Requirements

- Rust
- C++ compiler
- Cmake
- Linux and Windows need the Vulkan SDK to be installed

### Wasm
For wasm, need to install emscripten and rust wasm32.
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
