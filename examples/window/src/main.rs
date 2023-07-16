/*
Conditional compiliation
https://bitshifter.github.io/2020/05/07/conditional-compilation-in-rust/
*/

use rad::gpu;
// mod app;
// mod gpu;
// mod gui;

mod main_window;


//use nfd2::Response;

fn main() {

  let mut main_window = match main_window::MainWindow::new(gpu::renderer_types::RendererType::DirectX){
    Ok(res) => res,
    Err(_res) => {
      eprintln!("Error");
      std::process::exit(-1);
    }
  };

  main_window.run();
}
