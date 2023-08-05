
use rad::gpu;
use clap::Parser;

mod main_window;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[cfg(target_os = "emscripten")]
  #[arg(short, long, default_value_t = ("opengles".to_string()))]
  graphics_api: String,

  #[cfg(not(target_os = "emscripten"))]
  #[arg(short, long, default_value_t = ("opengl".to_string()))]
  graphics_api: String,
}

fn main() {
  let args = Args::parse();

  println!("Start");

  let graphics_api = match args.graphics_api.as_str() {
    "opengl" => gpu::renderer_types::RendererType::OpenGL,
    "opengles" => gpu::renderer_types::RendererType::OpenGLES,
    "directx" => gpu::renderer_types::RendererType::DirectX,
    "vulkan" => gpu::renderer_types::RendererType::Vulkan,
    "metal" => gpu::renderer_types::RendererType::Metal,
    _ => gpu::renderer_types::RendererType::OpenGL
  };

  let mut main_window = match main_window::MainWindow::new(graphics_api){
    Ok(res) => res,
    Err(_res) => {
      eprintln!("Error");
      std::process::exit(-1);
    }
  };

  main_window.init();

  rad::gui::main_loop::run_loop(main_window);

}
