/*
Conditional compiliation
https://bitshifter.github.io/2020/05/07/conditional-compilation-in-rust/
*/

#[macro_use]
extern crate bitflags;

mod app;
mod gpu;
mod gui;


//use nfd2::Response;

fn main() {

  let mut main_window = match app::main_window::MainWindow::new(gpu::renderer_types::RendererType::DirectX){
    Ok(res) => res,
    Err(_res) => {
      eprintln!("Error");
      std::process::exit(-1);
    }
  };

  main_window.run();
}
