
mod main_window;

fn main() {
  let mut foo = match main_window::MainWindow::init(){
    Ok(res) => res,
    Err(res) => {
      eprintln!("Error");
      std::process::exit(-1);
    }
  };

  foo.run();
}
