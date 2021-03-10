extern crate sdl2;
//extern crate raw_window_handle;
extern crate libc;

//mod gpu;

use crate::gpu;
use crate::gpu::Gpu;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::fmt;

#[derive(Debug, Clone)]
pub enum WindowError {
  SdlInitError,
  SdlWindowError,
  SdlGlError,
  Error
}

impl std::error::Error for WindowError {}

impl fmt::Display for WindowError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      WindowError::Error => write!(f, "Error"),
      WindowError::SdlInitError => write!(f, "Sdl Init Error"),
      WindowError::SdlGlError => write!(f, "Gl Init Error"),
      WindowError::SdlWindowError => write!(f, "Window Init Error"),
    }
  }
}

pub struct MainWindow{
  active: bool,
  width: i32,
  height: i32,
  sdl_context: sdl2::Sdl,
  video_subsystem: sdl2::VideoSubsystem,
  window: sdl2::video::Window,
  gl_context: sdl2::video::GLContext
  //canvas: sdl2::render::WindowCanvas
}



impl MainWindow {
  pub fn init(a_gpu_type: gpu::GpuType) -> Result<MainWindow, WindowError>  {
    let width: i32 = 800;
    let height: i32 = 600;
    let sdl_context = match sdl2::init(){
      Ok(res) => res,
      Err(res) => return Err(WindowError::SdlInitError)
    };

    let video_subsystem = match sdl_context.video(){
      Ok(res) => res,
      Err(res) => return Err(WindowError::SdlInitError)
    };

    /*
    let window: sdl2::video::Window = match video_subsystem.window("rust-sdl2 demo", width as u32, height as u32)
      .position_centered()
      .allow_highdpi()
      .opengl()
      .resizable()
      .build() {
        Ok(res) => res,
        Err(res) => return Err(WindowError::SdlWindowError)
      };
      */
    let window_name = "rust-sdl2 demo";

    let window = match a_gpu_type {
      OpenGL => init_window_opengl(&video_subsystem, window_name, width as u32, height as u32),
      Vulkan => init_window_vulkan(&video_subsystem, window_name, width as u32, height as u32),
      _ => init_window(&video_subsystem, window_name, width as u32, height as u32)
    };
      
    let window = match window {
      Ok(res) => res,
      Err(res) => return Err(WindowError::SdlWindowError)
    };

    let raw_windows_handle = window.raw_window_handle();

    
    #[cfg(target_os = "windows")]
    println!("Windows");
    #[cfg(any(
      target_os = "linux",
      target_os = "dragonfly",
      target_os = "freebsd",
      target_os = "netbsd",
      target_os = "openbsd",
    ))]
    println!("Linux");
    
    //window.

    //Box<dyn gpu::Gpu>
    let gp = init_gpu(a_gpu_type);

    let gl_context = match init_gl_context(&video_subsystem, &window) {
      Ok(res) => res,
      Err(res) => return Err(WindowError::SdlGlError)
    };

    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);


    let result = MainWindow {
      active: false,
      width: width,
      height: height,
      sdl_context: sdl_context,
      video_subsystem: video_subsystem,
      window: window,
      gl_context: gl_context
      //canvas: canvas
    };

    Ok(result)
  } 

  pub fn run(&mut self) {
    
    //self.canvas.set_draw_color(Color::RGB(0, 255, 255));
    //self.canvas.clear();
    //self.canvas.present();
    unsafe {
      gl::ClearColor(0.3, 0.3, 0.5, 1.0);
      gl::Viewport(0, 0, self.width, self.height);
    }

    let mut event_pump = self.sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
      i = (i + 1) % 255;
      //self.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
      //self.canvas.clear();
      for event in event_pump.poll_iter() {
          match event {
              Event::Quit {..} |
              Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                  break 'running
              },
              _ => {}
          }
      }
      // The rest of the game loop goes here...
      unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
      }
      
      self.window.gl_swap_window();
      //self.canvas.present();
      ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
  }
}

fn init_gpu(a_gpu_type: gpu::GpuType) -> Box<dyn gpu::Gpu> {
  match a_gpu_type {
    gpu::GpuType::OpenGL => Box::new(gpu::GpuOpenGL::new()),
    gpu::GpuType::Vulkan => Box::new(gpu::GpuVulkan::new()),
    _ => panic!("Unknown gpu type")
  }
}

fn init_window(a_video_subsystem: &sdl2::VideoSubsystem, a_name: &str, a_width: u32, a_height: u32) -> Result<sdl2::video::Window, sdl2::video::WindowBuildError> {
  a_video_subsystem.window(a_name, a_width, a_height)
      .position_centered()
      .allow_highdpi()
      .resizable()
      .build()
}

fn init_window_opengl(a_video_subsystem: &sdl2::VideoSubsystem, a_name: &str, a_width: u32, a_height: u32) -> Result<sdl2::video::Window, sdl2::video::WindowBuildError> {
  a_video_subsystem.window(a_name, a_width, a_height)
      .position_centered()
      .allow_highdpi()
      .resizable()
      .opengl()
      .build()
}

fn init_window_vulkan(a_video_subsystem: &sdl2::VideoSubsystem, a_name: &str, a_width: u32, a_height: u32) -> Result<sdl2::video::Window, sdl2::video::WindowBuildError> {
  a_video_subsystem.window(a_name, a_width, a_height)
      .position_centered()
      .allow_highdpi()
      .resizable()
      .vulkan()
      .build()
}

fn init_gl_context(a_video_subsystem: &sdl2::VideoSubsystem, a_window: &sdl2::video::Window) -> Result<sdl2::video::GLContext, WindowError> {
  //let mut attempt = true;
  let mut gl_version_major = 4;
  let mut gl_version_minor = 7;

  let gl_attr = a_video_subsystem.gl_attr();

  loop {
    if gl_version_major > 2 {
      gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    }
    
    gl_attr.set_context_version(gl_version_major, gl_version_minor);

    let gl_context_result = a_window.gl_create_context();

    match gl_context_result {
      Ok(res) => {
        return Ok(res);
      },
      Err(res) => {
        //try lower version of gl
        if gl_version_minor > 0 {
          gl_version_minor -= 1;
        }
        else if gl_version_major == 4 && gl_version_minor == 0 {
          gl_version_major = 3;
          gl_version_minor = 3;
        }
        else if gl_version_major == 3 && gl_version_minor == 0 {
          gl_version_major = 2;
          gl_version_minor = 1;
        }
        else if gl_version_major == 2 && gl_version_minor == 0 {
          gl_version_major = 1;
          gl_version_minor = 5;
        }
        else if gl_version_major == 1 && gl_version_minor == 0 {
          return Err(WindowError::SdlGlError)
        }
      }
    }
  }
}
