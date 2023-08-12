
use rad::gpu::renderer_types;
use rad::gui::window::Window;
use std::env;

use std::panic::{self, AssertUnwindSafe};

fn main() {
  let mut tests = Tests::new();

  tests.run("create_window", create_window);
  tests.run("init_opengl", init_opengl);
  tests.run("init_opengles", init_opengles);

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
    let window_result = Window::new(renderer_type, "Test", 240, 160);
      
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

fn init_renderer(a_renderer_type: renderer_types::RendererType, a_minor_versions: &Vec<i32>, a_max_version_major: i32, a_max_version_minor: i32) {  
  let mut minor_versions = a_minor_versions.clone();

  minor_versions[(a_max_version_major-1) as usize] = a_max_version_minor;
  
  for major_version in 1..=a_max_version_major{
    for minor_version in 0..=minor_versions[(major_version-1) as usize]{
      let window_result = Window::new(a_renderer_type, "Test", 240, 160);
        
      assert!(window_result.is_ok(), "window creation failed");

      let window = window_result.unwrap();

      let renderer_result = Window::init_renderer(window.renderer_type, 
        renderer_types::Version{major: renderer_types::VersionNum::Value(major_version as i32), minor: renderer_types::VersionNum::Value(minor_version), patch: renderer_types::VersionNum::Lowest},
        renderer_types::Version{major: renderer_types::VersionNum::Value(major_version as i32), minor: renderer_types::VersionNum::Value(minor_version), patch: renderer_types::VersionNum::Highest},
        &(window.video_subsystem.lock().unwrap()).inner,
        &(window.window.lock().unwrap()).inner);

      assert!(renderer_result.is_ok(), "Renderer creation failed version {}.{}", major_version, minor_version);
    }
  }
}

fn init_opengl() { 
  let renderer = renderer_types::RendererType::OpenGL;

  let version = match get_api_supported(renderer.to_string().to_uppercase().as_str()){
    Some(res) => res,
    None => return
  };

  let minor_versions = vec![5, 1, 3, 6];

  init_renderer(renderer, &minor_versions, version.0, version.1);
}

fn init_opengles() { 
  let renderer = renderer_types::RendererType::OpenGLES;

  let version = match get_api_supported(renderer.to_string().to_uppercase().as_str()){
    Some(res) => res,
    None => return
  };

  let minor_versions = vec![1, 0, 2];

  init_renderer(renderer, &minor_versions, version.0, version.1);
}
