use std::env;

fn main() {
  if env::var("TARGET").unwrap() == "wasm32-unknown-emscripten" {
    let mut linker_flags = "-s ALLOW_MEMORY_GROWTH=1 -s MAX_WEBGL_VERSION=2 -s TOTAL_MEMORY=512MB --preload-file shaders/gles/basic.vert --preload-file shaders/gles/basic.frag --preload-file image.jpg ".to_string();
    let mut emcc_flags = "".to_string();

    if env::var("PROFILE").unwrap() == "debug" {
      emcc_flags = "-g3 -gsource-map ".to_string();
      linker_flags += "-s ASSERTIONS=2 -s SAFE_HEAP=1 -s STACK_OVERFLOW_CHECK=1 ";

      println!("cargo:rustc-env=EMCC_CFLAGS={}", emcc_flags);
    }

    println!("cargo:rustc-env=LDFLAGS={}", emcc_flags + &linker_flags);
  }
}
