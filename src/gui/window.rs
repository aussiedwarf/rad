
extern crate sdl2;
use crate::gpu::renderer;
use crate::gpu::renderer_types;
use crate::gpu::renderer_opengl;
use crate::gpu::renderer_vulkan;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum WindowError {
  SdlInitError,
  SdlWindowError,
  SdlRendererError,
  Error
}

impl std::error::Error for WindowError {}

impl fmt::Display for WindowError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      WindowError::Error => write!(f, "Error"),
      WindowError::SdlInitError => write!(f, "Sdl Init Error"),
      WindowError::SdlRendererError => write!(f, "Renderer Init Error"),
      WindowError::SdlWindowError => write!(f, "Window Init Error"),
    }
  }
}

#[allow(dead_code)]
pub struct Window{
  active: bool,
  pub width: u32,
  pub height: u32,
  pub sdl_context: sdl2::Sdl,
  pub video_subsystem: sdl2::VideoSubsystem,
  pub window: sdl2::video::Window,
  //raw_window_handle: RawWindowHandle,
  pub renderer: Box<dyn renderer::Renderer>
  //canvas: sdl2::render::WindowCanvas
}

impl Window {
  pub fn new(a_renderer_type: renderer_types::RendererType, a_name: &str, a_width: u32, a_height: u32) -> Result<Window, WindowError> {
    let sdl_context = match sdl2::init(){
      Ok(res) => res,
      Err(_res) => return Err(WindowError::SdlInitError)
    };

    let video_subsystem = match sdl_context.video(){
      Ok(res) => res,
      Err(_res) => return Err(WindowError::SdlInitError)
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

    let window = match a_renderer_type {
      renderer_types::RendererType::OpenGL => Window::init_window_opengl(&video_subsystem, a_name, a_width as u32, a_height as u32),
      renderer_types::RendererType::Vulkan => Window::init_window_vulkan(&video_subsystem, a_name, a_width as u32, a_height as u32),
      _ => Window::init_window(&video_subsystem, a_name, a_width as u32, a_height as u32)
    };
      
    let window = match window {
      Ok(res) => res,
      Err(_res) => return Err(WindowError::SdlWindowError)
    };

    //let raw_window_handle = window.raw_window_handle();

    
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

    //Box<dyn renderer::Renderer>
    let renderer = match Window::init_renderer(a_renderer_type, &video_subsystem, &window){
      Ok(res) => res,
      Err(res) => return Err(res)
    };

    let result = Window {
      active: false,
      width: a_width,
      height: a_height,
      sdl_context: sdl_context,
      video_subsystem: video_subsystem,
      window: window,
      //raw_window_handle: raw_window_handle,
      renderer: renderer,
      //canvas: canvas
    };

    Ok(result)
  } 

  fn init_renderer(a_renderer_type: renderer_types::RendererType, a_video_subsystem: &sdl2::VideoSubsystem, a_window: &sdl2::video::Window) -> 
  Result<Box<dyn renderer::Renderer>, WindowError > {
  match a_renderer_type {
    renderer_types::RendererType::OpenGL => {
      Ok(Box::new(match renderer_opengl::RendererOpenGL::new(a_video_subsystem, a_window){
        Ok(res) => res,
        Err(_res) => return Err(WindowError::SdlRendererError)
      }
      ))
    },
    renderer_types::RendererType::Vulkan => {
      Ok(Box::new( match renderer_vulkan::RendererVulkan::new(){
        Ok(res) => res,
        Err(_res) => return Err(WindowError::SdlRendererError)
      }))
    },
    _ => Err(WindowError::SdlRendererError)
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
}
