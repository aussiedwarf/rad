/*
Conditional compiliation
https://bitshifter.github.io/2020/05/07/conditional-compilation-in-rust/
*/

extern crate nfd2;

mod gpu;
mod main_window;

use nfd2::Response;

fn main() {

  let mut foo = match main_window::MainWindow::init(gpu::GpuType::OpenGL){
    Ok(res) => res,
    Err(res) => {
      eprintln!("Error");
      std::process::exit(-1);
    }
  };

  /*
  //https://github.com/EmbarkStudios/nfd2
  //https://github.com/native-toolkit/nfd
  match nfd2::open_file_dialog(Some("jpg,jpeg;png;bmp"), None).expect("oh no") {
    Response::Okay(file_path) => println!("File path = {:?}", file_path),
    Response::OkayMultiple(files) => println!("Files {:?}", files),
    Response::Cancel => println!("User canceled"),
  }
  */

  foo.run();
}
