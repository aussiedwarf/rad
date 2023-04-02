/*
Conditional compiliation
https://bitshifter.github.io/2020/05/07/conditional-compilation-in-rust/
*/

#[macro_use]
extern crate bitflags;

mod gpu;
mod app;

//use nfd2::Response;

fn main() {

  let mut foo = match app::main_window::MainWindow::init(gpu::renderer_types::RendererType::OpenGL){
    Ok(res) => res,
    Err(_res) => {
      eprintln!("Error");
      std::process::exit(-1);
    }
  };

  foo.run();
}
