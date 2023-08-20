
extern crate sdl2;
use crate::core::unsafe_send::UnsafeSend;
use crate::gpu::renderer;
use crate::gpu::renderer_types;
use crate::gpu::opengl::renderer_opengl;
use crate::gpu::vulkan::renderer_vulkan;
use crate::gpu::directx::renderer_directx12;
use std::fmt;
use std::sync::Mutex;
use std::sync::Arc;

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
  pub sdl_context: Arc<Mutex<UnsafeSend<sdl2::Sdl>>>,
  pub video_subsystem: Arc<Mutex<UnsafeSend<sdl2::VideoSubsystem>>>,
  pub window: Arc<Mutex<UnsafeSend<sdl2::video::Window>>>,
  //raw_window_handle: RawWindowHandle,
  //pub renderer: Box<dyn renderer::Renderer>,
  pub renderer_type: renderer_types::RendererType
  //canvas: sdl2::render::WindowCanvas
}

impl Window {
  pub fn new(
    a_renderer_type: renderer_types::RendererType, 
    a_name: &str, a_width: u32, a_height: u32, a_x: i32, a_y: i32, a_flags: u32) -> Result<Window, WindowError> 
  {
    let sdl_context = match sdl2::init(){
      Ok(res) => res,
      Err(_res) => return Err(WindowError::SdlInitError)
    };

    let video_subsystem = match sdl_context.video(){
      Ok(res) => res,
      Err(_res) => return Err(WindowError::SdlInitError)
    };

    let window = match a_renderer_type {
      renderer_types::RendererType::OpenGL | renderer_types::RendererType::OpenGLES => Window::init_window_opengl(&video_subsystem, a_name, a_width as u32, a_height as u32, a_x, a_y, a_flags),
      renderer_types::RendererType::DirectX => Window::init_window(&video_subsystem, a_name, a_width as u32, a_height as u32, a_x, a_y, a_flags),
      renderer_types::RendererType::Vulkan => Window::init_window_vulkan(&video_subsystem, a_name, a_width as u32, a_height as u32, a_x, a_y, a_flags),
      _ => Window::init_window(&video_subsystem, a_name, a_width as u32, a_height as u32, a_x, a_y, a_flags)
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
    
    // let renderer = match Window::init_renderer(a_renderer_type, &video_subsystem, &window){
    //   Ok(res) => res,
    //   Err(res) => return Err(res)
    // };

    let result = Window {
      active: false,
      width: a_width,
      height: a_height,
      sdl_context: Arc::new(Mutex::new(unsafe{UnsafeSend::new(sdl_context)})),
      video_subsystem: Arc::new(Mutex::new(unsafe{UnsafeSend::new(video_subsystem)})),
      window: Arc::new(Mutex::new(unsafe{UnsafeSend::new(window)})),
      //raw_window_handle: raw_window_handle,
      //renderer: renderer,
      //canvas: canvas
      renderer_type: a_renderer_type
    };

    Ok(result)
  } 

  pub fn init_renderer(
    a_renderer_type: renderer_types::RendererType, 
    a_min_version: renderer_types::Version, 
    a_max_version: renderer_types::Version, 
    a_video_subsystem: &sdl2::VideoSubsystem, 
    a_window: Arc<Window>) -> Result<Box<dyn renderer::Renderer>, WindowError > 
  {
    let is_gles = a_renderer_type == renderer_types::RendererType::OpenGLES;
    match a_renderer_type {
      renderer_types::RendererType::OpenGL | renderer_types::RendererType::OpenGLES => 
      {
        Ok(Box::new(match renderer_opengl::RendererOpenGL::new(a_video_subsystem, a_min_version, a_max_version, a_window, is_gles){
        Ok(res) => res,
        Err(_res) => return Err(WindowError::SdlRendererError)
      }
      ))
    },
      renderer_types::RendererType::DirectX => 
      {
      Ok(Box::new(match renderer_directx12::RendererDirectX12::new(a_video_subsystem, &a_window.window.lock().unwrap().inner){
        Ok(res) => res,
        Err(_res) => return Err(WindowError::SdlRendererError)
      }
      ))
    },
      renderer_types::RendererType::Vulkan => 
      {
      Ok(Box::new( match renderer_vulkan::RendererVulkan::new(&a_window.window.lock().unwrap().inner, true){
        Ok(res) => res,
        Err(_res) => return Err(WindowError::SdlRendererError)
      }))
    },
    _ => Err(WindowError::SdlRendererError)
  }
}

fn init_window(a_video_subsystem: &sdl2::VideoSubsystem, a_name: &str, a_width: u32, a_height: u32, a_x: i32, a_y: i32, a_flags: u32) -> Result<sdl2::video::Window, sdl2::video::WindowBuildError> {
  a_video_subsystem.window(a_name, a_width, a_height)
    .position(a_x, a_y)
    .set_window_flags(a_flags)
    .build()
}

fn init_window_opengl(a_video_subsystem: &sdl2::VideoSubsystem, a_name: &str, a_width: u32, a_height: u32, a_x: i32, a_y: i32, a_flags: u32) -> Result<sdl2::video::Window, sdl2::video::WindowBuildError> {
  a_video_subsystem.window(a_name, a_width, a_height)
    .position(a_x, a_y)
    .set_window_flags(a_flags)
    .opengl()
    .build()
}

fn init_window_vulkan(a_video_subsystem: &sdl2::VideoSubsystem, a_name: &str, a_width: u32, a_height: u32, a_x: i32, a_y: i32, a_flags: u32) -> Result<sdl2::video::Window, sdl2::video::WindowBuildError> {
  a_video_subsystem.window(a_name, a_width, a_height)
    .position(a_x, a_y)
    .set_window_flags(a_flags)
    .vulkan()
    .build()
}
}
