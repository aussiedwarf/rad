
use rad::gpu::renderer_types;
use rad::gui::window::Window;

use std::panic::{self, AssertUnwindSafe};

fn main() {
  let mut tests = Tests::new();

  tests.run("test_function_1", create_window);
  tests.run("test_function_2", init_opengl);

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

fn init_opengl() { 
  let minor_versions = [5, 1, 3, 6];

  for major_version in 1..=4{
    for minor_version in 0..=minor_versions[major_version-1]{
      let window_result = Window::new(renderer_types::RendererType::OpenGL, "Test", 240, 160);
        
      assert!(window_result.is_ok(), "window creation failed");

      let window = window_result.unwrap();

      let renderer_result = Window::init_renderer(window.renderer_type, 
        renderer_types::Version{major: renderer_types::VersionNum::Value(major_version as i32), minor: renderer_types::VersionNum::Value(minor_version), patch: renderer_types::VersionNum::Lowest},
        renderer_types::Version{major: renderer_types::VersionNum::Value(major_version as i32), minor: renderer_types::VersionNum::Value(minor_version), patch: renderer_types::VersionNum::Highest},
        &(window.video_subsystem.lock().unwrap()).inner,
        &(window.window.lock().unwrap()).inner);

      assert!(renderer_result.is_ok(), "renderer creation failed");
    }
  }
}

