use std::sync::Arc;
use rad::gpu::renderer_types;
use rad::gpu::renderer::*;
use rad::gui::window::Window;
use std::env;
use std::panic::{self, AssertUnwindSafe};

use glam::*;

fn main() {
  let mut tests = Tests::new();

  tests.run("create_window", create_window);
  tests.run("init_opengl", init_opengl);
  tests.run("init_opengles", init_opengles);
  tests.run("clear_screen", clear_screen);

  println!("\nTest Results:");
  println!("Total: {}", tests.passed + tests.failed);
  println!("Passed: {}", tests.passed);
  println!("Failed: {}", tests.failed);

  if tests.failed > 0 {
    std::process::exit(1);
  }
}

struct Tests{
  pub passed: usize,
  pub failed: usize,
}

impl Tests{
  pub fn new() -> Tests{
    return Tests{
      passed: 0,
      failed: 0
    }
  }

  pub fn run<F: FnOnce() + panic::UnwindSafe>(&mut self, name: &str, test: F) {
    println!("Running test: {}", name);
    let result = panic::catch_unwind(AssertUnwindSafe(test));
    match result {
        Ok(_) => {
          println!("Test passed: {}", name);
          self.passed += 1;
        },
        Err(_) => {
          println!("Test FAILED: {}", name);
          self.failed += 1;
        },
    }
  }
}

fn create_window() { 
  let renderer_types = [
    renderer_types::RendererType::OpenGL,
    renderer_types::RendererType::OpenGLES
    ];

  for renderer_type in renderer_types
  {
    let window_result = Window::new(
      renderer_type, "Test", 240, 160,
      sdl2::sys::SDL_WINDOWPOS_CENTERED_MASK as i32, sdl2::sys::SDL_WINDOWPOS_CENTERED_MASK as i32, 
      sdl2::sys::SDL_WindowFlags::SDL_WINDOW_BORDERLESS as u32 | sdl2::sys::SDL_WindowFlags::SDL_WINDOW_ALLOW_HIGHDPI as u32);
      
    assert!(window_result.is_ok(), "window creation failed");
  }
}

fn get_env_version(key: &str) -> Option<i32> {
  match env::var(key) {
    Ok(res) => match res.parse::<i32>() {
      Ok(res) => return Some(res),
      Err(_res) => return None
    },
    Err(_res) => return None
  };
}

fn get_api_supported(key: &str) -> Option<(i32, i32)> {
  let major = match get_env_version((key.to_owned() + "_MAJOR").as_str()) {
    Some(res) => res,
    None => return None
  };

  let minor = match get_env_version((key.to_owned() + "_MINOR").as_str()) {
    Some(res) => res,
    None => 0
  };

  return Some((major, minor))
}

fn init_renderer(
    a_renderer_type: renderer_types::RendererType, 
    a_minor_versions: &Vec<i32>, a_max_version_major: i32, a_max_version_minor: i32, 
    function: fn(Arc<Window>, &mut Box<dyn Renderer>)) 
  {  
  let mut minor_versions = a_minor_versions.clone();

  minor_versions[(a_max_version_major-1) as usize] = a_max_version_minor;
  
  for major_version in 1..=a_max_version_major{
    for minor_version in 0..=minor_versions[(major_version-1) as usize]{
      
      let window_result = Window::new(
        a_renderer_type, "Test", 240, 160, 
        sdl2::sys::SDL_WINDOWPOS_CENTERED_MASK as i32, sdl2::sys::SDL_WINDOWPOS_CENTERED_MASK as i32,
        sdl2::sys::SDL_WindowFlags::SDL_WINDOW_BORDERLESS as u32 | sdl2::sys::SDL_WindowFlags::SDL_WINDOW_ALLOW_HIGHDPI as u32);
        
      assert!(window_result.is_ok(), "window creation failed");

      let window = Arc::new(window_result.unwrap());

      let renderer_result = Window::init_renderer(window.renderer_type, 
        renderer_types::Version{major: renderer_types::VersionNum::Value(major_version as i32), minor: renderer_types::VersionNum::Value(minor_version), patch: renderer_types::VersionNum::Lowest},
        renderer_types::Version{major: renderer_types::VersionNum::Value(major_version as i32), minor: renderer_types::VersionNum::Value(minor_version), patch: renderer_types::VersionNum::Highest},
        &(window.video_subsystem.lock().unwrap()).inner,
        window.clone());

      assert!(renderer_result.is_ok(), "Renderer creation failed with {} version {}.{}", a_renderer_type, major_version, minor_version);

      function(window.clone(), &mut renderer_result.unwrap());
    }
  }
}

fn do_nothing(_window: Arc<Window>, _renderer: &mut Box<dyn Renderer>){}

fn test_opengl(function: fn(Arc<Window>, &mut Box<dyn Renderer>)) {
  let renderer = renderer_types::RendererType::OpenGL;

  let version = match get_api_supported(renderer.to_string().to_uppercase().as_str()){
    Some(res) => res,
    None => return
  };

  let minor_versions = vec![5, 1, 3, 6];

  init_renderer(renderer, &minor_versions, version.0, version.1, function);
}

fn test_opengles(function: fn(Arc<Window>, &mut Box<dyn Renderer>)) {
  let renderer = renderer_types::RendererType::OpenGLES;

  let version = match get_api_supported(renderer.to_string().to_uppercase().as_str()){
    Some(res) => res,
    None => return
  };

  let minor_versions = vec![1, 0, 2];

  init_renderer(renderer, &minor_versions, version.0, version.1, function);
}

fn init_opengl() { 
  test_opengl(do_nothing);
}

fn init_opengles() { 
  test_opengles(do_nothing);
}

fn squared(num: i32) -> i64{
  num as i64 * num as i64
}

fn mean_square_error(buffer: &Vec<u8>, color: [u8; 4]) -> f64{
  let mut diff: i64 = 0;
  for i in (0..buffer.len()).step_by(4)
  {
    diff += squared(color[0] as i32 - buffer[i + 0] as i32);
    diff += squared(color[1] as i32 - buffer[i + 1] as i32);
    diff += squared(color[2] as i32 - buffer[i + 2] as i32);
    diff += squared(color[3] as i32 - buffer[i + 3] as i32);
  }
  return diff as f64 / buffer.len() as f64;
}

fn test_clear_screen(_window:Arc<Window>, renderer: &mut Box<dyn Renderer>){
  let color = [191, 127, 63, 255 ];
  
  renderer.set_clear_color(Vec4::new(color[0] as f32 / 255.0, color[1] as f32 / 255.0, color[2] as f32 / 255.0, color[3] as f32 / 255.0));

  renderer.begin_frame(renderer_types::RendererClearType::COLOR);

  let image = renderer.read_render_buffer();

  renderer.end_frame();

  let mse = mean_square_error(image.pixels.as_ref(), color);

  assert!(mse <= 1.0, "Clear color is not within tolerance. MSE: {}", mse);
}

fn clear_screen() {
  test_opengl(test_clear_screen);
  test_opengles(test_clear_screen);
}
