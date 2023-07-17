
use rad::gpu;
use clap::Parser;

mod main_window;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long, default_value_t = ("opengl".to_string()))]
  graphics_api: String,
}

fn main() {
  let args = Args::parse();

  let graphics_api = match args.graphics_api.as_str() {
    "opengl" => gpu::renderer_types::RendererType::OpenGL,
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

  main_window.run();
}
