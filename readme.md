[![Build](https://github.com/aussiedwarf/rad/actions/workflows/rust.yml/badge.svg?branch=dev)](https://github.com/aussiedwarf/rad/actions/workflows/rust.yml)

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

To compile wasm debug build with embedded files.
`EMCC_CFLAGS="-g4 -gsource-map" LDFLAGS="-s ALLOW_MEMORY_GROWTH=1 -s MAX_WEBGL_VERSION=2 -s ASSERTIONS=2 -s SAFE_HEAP=1 -s STACK_OVERFLOW_CHECK=1 -s TOTAL_MEMORY=512MB --preload-file shaders/gles/basic.vert --preload-file shaders/gles/basic.frag --preload-file image.jpg" cargo build --target wasm32-unknown-emscripten --verbose`

Release build
`LDFLAGS="-s ALLOW_MEMORY_GROWTH=1 -s MAX_WEBGL_VERSION=2 -s TOTAL_MEMORY=512MB --preload-file shaders/gles/basic.vert --preload-file shaders/gles/basic.frag --preload-file image.jpg" cargo build --target wasm32-unknown-emscripten --verbose --release`

Debug build
`EMCC_CFLAGS="-g4 -gsource-map" LDFLAGS="-g4 -gsource-map -s ALLOW_MEMORY_GROWTH=1 -s MAX_WEBGL_VERSION=2 -s ASSERTIONS=2 -s SAFE_HEAP=1 -s STACK_OVERFLOW_CHECK=1 -s TOTAL_MEMORY=512MB" cargo build --target wasm32-unknown-emscripten --verbose`

To compile generally
```
cargo build
cargo build --release
```
