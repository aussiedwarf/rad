extern crate sdl2;
//extern crate raw_window_handle;
extern crate libc;

//mod renderer;

use crate::gpu::camera::*;
use crate::gpu::renderer;
use crate::gpu::renderer_types;
use crate::gpu::renderer_opengl;
use crate::gpu::renderer_vulkan;
use crate::gpu::material;

//use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
//use std::rc::Rc;
use glam::*;


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
pub struct MainWindow{
  active: bool,
  width: i32,
  height: i32,
  sdl_context: sdl2::Sdl,
  video_subsystem: sdl2::VideoSubsystem,
  window: sdl2::video::Window,
  //raw_window_handle: RawWindowHandle,
  renderer: Box<dyn renderer::Renderer>
  //canvas: sdl2::render::WindowCanvas
}



impl MainWindow {
  pub fn init(a_renderer_type: renderer_types::RendererType) -> Result<MainWindow, WindowError>  {
    let width: i32 = 800;
    let height: i32 = 600;
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
    let window_name = "rust-sdl2 demo";

    let window = match a_renderer_type {
      renderer_types::RendererType::OpenGL => init_window_opengl(&video_subsystem, window_name, width as u32, height as u32),
      renderer_types::RendererType::Vulkan => init_window_vulkan(&video_subsystem, window_name, width as u32, height as u32),
      _ => init_window(&video_subsystem, window_name, width as u32, height as u32)
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
    let renderer = match init_renderer(a_renderer_type, &video_subsystem, &window){
      Ok(res) => res,
      Err(res) => return Err(res)
    };

    let result = MainWindow {
      active: false,
      width: width,
      height: height,
      sdl_context: sdl_context,
      video_subsystem: video_subsystem,
      window: window,
      //raw_window_handle: raw_window_handle,
      renderer: renderer,
      //canvas: canvas
    };

    Ok(result)
  } 


  pub fn run(&mut self) {
    let mut file_vertex = match File::open("basic.vert"){
      Ok(res) => res,
      Err(_res) => return
    };
    let mut source_vertex = String::new();
    match file_vertex.read_to_string(&mut source_vertex){
      Ok(res) => res,
      Err(_res) => return
    };

    let shader_vertex = match self.renderer.load_shader(renderer_types::ShaderType::Vertex, source_vertex.as_ref()){
      Ok(res) => res,
      Err(_res) => return
    };

    let mut file_frag = match File::open("basic.frag"){
      Ok(res) => res,
      Err(_res) => return
    };
    let mut source_frag = String::new();
    match file_frag.read_to_string(&mut source_frag){
      Ok(res) => res,
      Err(_res) => return
    };

    let shader_frag = match self.renderer.load_shader(renderer_types::ShaderType::Fragment, &mut source_frag){
      Ok(res) => res,
      Err(_res) => return
    };

    let shader_program = match self.renderer.load_program_vert_frag(shader_vertex, shader_frag){
      Ok(res) => res,
      Err(_res) => return
    };

    let img = match image::open("image.jpg"){
      Ok(res) => res,
      Err(_res) => return
    };

    //only need to flip with opengl
    let img = img.flipv();

    let mut texture = self.renderer.gen_buffer_texture();

    self.renderer.load_texture(&img, &mut texture);

    

    //let uniform = self.renderer.get_uniform(&mut shader_program, "u_texture");

    let verts: std::vec::Vec<f32> = vec![
      -1.0, -1.0, 0.0, 0.0,
      1.0, -1.0, 1.0, 0.0,
      1.0, 1.0, 1.0, 1.0,
      1.0, 1.0, 1.0, 1.0,
      -1.0, 1.0, 0.0, 1.0,
      -1.0, -1.0, 0.0, 0.0];

    let vert_buffer = self.renderer.gen_buffer_vertex(&verts);

    let geometry = self.renderer.gen_geometry(&vert_buffer);

    let material = Box::new(material::MaterialBasic::new(shader_program, 
      self.renderer.gen_sampler(texture.into())));

    //let uniform_mvp = self.renderer.get_uniform(&mut shader_program, "u_mvp");

    //material.set_color_texture(&texture);

    let mut mesh = self.renderer.gen_mesh(geometry, material);
    
    self.renderer.set_clear_color(Vec4::new(0.0, 0.0, 0.0, 1.0));
    self.renderer.set_viewport(IVec2::new(0,0), IVec2::new(self.width, self.height));

    //self.renderer.use_program(&shader_program);
    //self.renderer.set_uniform(&uniform);
    //self.renderer.set_texture(&texture);

    let mut camera = Camera::new();
    camera.set_viewport(Vec2::new(self.width as f32, self.height as f32), Vec2::ZERO, Vec2::new(self.width as f32, self.height as f32), Vec2::ZERO);


    let mut event_pump = self.sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut r: f32 = 0.0;
    'running: loop {
      r += 0.01;
      if r > 1.0{
        r = 0.0;
      }

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
      self.renderer.set_clear_color(Vec4::new(r, 0.0, 0.0, 1.0));

      // The rest of the game loop goes here...
      self.renderer.clear(renderer_types::RendererClearType::COLOR);

      self.renderer.draw_mesh(&camera, &mut mesh);
      
      self.window.gl_swap_window();
      //self.canvas.present();
      ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
  }
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

