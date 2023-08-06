use std::env;

fn main() {
  if env::var("TARGET").unwrap() == "wasm32-unknown-emscripten" {
    let linker_flags = "".to_string();
    let mut emcc_flags = "".to_string();

    if env::var("PROFILE").unwrap() == "debug" {
      emcc_flags = "-g3 -gsource-map ".to_string();
      println!("cargo:rustc-env=EMCC_CFLAGS={}", emcc_flags);
    }

    println!("cargo:rustc-env=LDFLAGS={}", emcc_flags + &linker_flags);
  }
}
