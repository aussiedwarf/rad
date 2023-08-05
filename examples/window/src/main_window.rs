extern crate sdl2;
//extern crate raw_window_handle;
extern crate libc;

use rad::gpu::camera::*;
use rad::gpu::renderer_types;
use rad::gpu::material;
use rad::gpu::renderer;
use rad::gui::window::*;

#[cfg(target_os = "emscripten")]
use rad::gui::emscripten::{emscripten};

use rad::gui::main_loop::*;

use rad::core::filesystem::{filesystem};

//use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::fs::File;
use std::io::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
//use std::rc::Rc;
use glam::*;

struct Renderer{
  renderer: Box<dyn renderer::Renderer>,
  window: Arc<Window>,
  camera: Camera,
  mesh: Box<renderer::Mesh>
}

impl Renderer {
  pub fn new(window: Arc<Window>) -> Result<Renderer, renderer_types::RendererError> {
    let mut renderer = match Window::init_renderer(
      window.renderer_type, 
      &(window.video_subsystem.lock().unwrap()).inner,
      &(window.window.lock().unwrap()).inner)
    {
      Ok(res) => res,
      Err(_res) => panic!("Error creating renderer")
    };

    let shader_path = match window.renderer_type {
      renderer_types::RendererType::OpenGL => "shaders/opengl/",
      renderer_types::RendererType::OpenGLES => "shaders/gles/",
      _ => "shaders/"
    };

    let source_vertex = match filesystem::read_text_file_immediate(&(shader_path.to_owned() + "basic.vert")){
      Ok(res) => res,
      Err(_res) => return Err(renderer_types::RendererError::Error)
    };

    println!("shader:\n{}", source_vertex);

    let shader_vertex = match renderer.load_shader(renderer_types::ShaderType::Vertex, source_vertex.as_ref()){
      Ok(res) => res,
      Err(_res) => return Err(renderer_types::RendererError::Error)
    };

    let mut source_frag = match filesystem::read_text_file_immediate(&(shader_path.to_owned() + "basic.frag")){
      Ok(res) => res,
      Err(_res) => return Err(renderer_types::RendererError::Error)
    };

    println!("shader:\n{}", source_frag);

    let shader_frag = match renderer.load_shader(renderer_types::ShaderType::Fragment, &mut source_frag){
      Ok(res) => res,
      Err(_res) => return Err(renderer_types::RendererError::Error)
    };

    let shader_program = match renderer.load_program_vert_frag(shader_vertex, shader_frag){
      Ok(res) => res,
      Err(_res) => return Err(renderer_types::RendererError::Error)
    };

    let image_data = match filesystem::read_file_immediate::<u8>("image.jpg"){
      Ok(res) => res,
      Err(_res) => return Err(renderer_types::RendererError::Error)
    };
    
    let img = match image::load_from_memory(image_data.as_slice()){
      Ok(res) => res,
      Err(_res) => return Err(renderer_types::RendererError::Error)
    };

    //only need to flip with opengl
    let img = img.flipv();
    let mut texture = renderer.gen_buffer_texture();

    // let img = image::DynamicImage::ImageRgba8(image::ImageBuffer::new(8, 8));
    renderer.load_texture(&img, &mut texture);

    //let uniform = self.renderer.get_uniform(&mut shader_program, "u_texture");

    let verts: std::vec::Vec<f32> = vec![
      -1.0, -1.0, 0.0, 0.0,
      1.0, -1.0, 1.0, 0.0,
      1.0, 1.0, 1.0, 1.0,
      1.0, 1.0, 1.0, 1.0,
      -1.0, 1.0, 0.0, 1.0,
      -1.0, -1.0, 0.0, 0.0];

    let vert_buffer = renderer.gen_buffer_vertex(&verts);

    let geometry = renderer.gen_geometry(&vert_buffer);

    let material = Box::new(material::MaterialBasic::new(shader_program, 
      renderer.gen_sampler(texture.into())));

    // let uniform_mvp = self.renderer.get_uniform(&mut shader_program, "u_mvp");

    //material.set_color_texture(&texture);

    let mesh = renderer.gen_mesh(geometry, material);
    
    renderer.set_clear_color(Vec4::new(0.1, 0.1, 0.0, 1.0));
    renderer.set_viewport(IVec2::new(0,0), IVec2::new(window.width as i32, window.height as i32));

    //self.renderer.use_program(&shader_program);
    //self.renderer.set_uniform(&uniform);
    //self.renderer.set_texture(&texture);

    let mut camera = Camera::new();
    camera.set_viewport(Vec2::new(window.width as f32, window.height as f32), 
      Vec2::ZERO, Vec2::new(window.width as f32, window.height as f32), Vec2::ZERO);

    return Ok(Renderer{
      renderer: renderer,
      window: window,
      camera: camera,
      mesh: mesh
    })
  }

  pub fn run(&mut self /* , i: &mut i32, r: &mut f32*/) {
    /*
    *r += 0.01;
    if *r > 1.0{
      *r = 0.0;
    }

    *i = (*i + 1) % 255;
    */
    let col = Vec4::new(0.5, 0.0, 0.0, 1.0);

    self.renderer.set_clear_color(col);

    // The rest of the game loop goes here...
    self.renderer.clear(renderer_types::RendererClearType::COLOR);

    self.renderer.draw_mesh(&self.camera, &mut self.mesh);

    self.renderer.end_frame();
    
    self.window.window.lock().unwrap().inner.gl_swap_window();
    //self.canvas.present();
    //std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }
}

pub struct MainWindow{
  window: Arc<Window>,
  renderer: Option<Renderer>,

  running_events: Arc<AtomicBool>,
  running_logic: Arc<AtomicBool>,
  running_render: Arc<AtomicBool>,

  thread_logic: Option<thread::JoinHandle<()>>,
  thread_render: Option<thread::JoinHandle<()>>,
}



impl MainWindow {
  pub fn new(a_renderer_type: renderer_types::RendererType) -> Result<MainWindow, WindowError> {

    match Window::new(a_renderer_type, "Title", 800, 600){
      Ok(res) => return Ok(MainWindow{
        window: Arc::new(res), 
        renderer: None,
        running_events: Arc::new(AtomicBool::new(true)),
        running_logic: Arc::new(AtomicBool::new(true)),
        running_render: Arc::new(AtomicBool::new(true)),
        thread_logic: None,
        thread_render: None
      }),
      Err(res) => return Err(res)
    }
  }

  #[cfg(not(target_os = "emscripten"))]
  pub fn init_threads(&mut self) {
    let running_logic = Arc::clone(&self.running_events);

    self.thread_logic = Some(thread::spawn(move|| {
      MainWindow::run_logic_loop(running_logic);
    }));

    let running_render = Arc::clone(&self.running_events);
    let window = Arc::clone(&self.window);

    self.thread_render = Some(thread::spawn(move|| {
      MainWindow::run_render_loop(running_render, window);
    }));
  }

  pub fn run_logic() {
  }

  #[cfg(not(target_os = "emscripten"))]
  pub fn run_logic_loop(running: Arc<AtomicBool>) {
    
    while running.load(Ordering::SeqCst){
      MainWindow::run_logic();
      std::thread::sleep(Duration::new(0, 1));
    }
    println!{"Thread Logic done"};
  }

  pub fn run_render_loop(running: Arc<AtomicBool>, window: Arc<Window>) {
    let mut renderer = match Renderer::new(window){
      Ok(res) => res,
      Err(_res) => return
    };

    // let mut i = 0;
    // let mut r: f32 = 0.0;

    while running.load(Ordering::SeqCst){
      renderer.run(/*&mut i, &mut r*/);
      std::thread::sleep(Duration::new(0, 1));
    }
    println!{"Thread Render done"};
  }
  
  #[cfg(not(target_os = "emscripten"))]
  pub fn init(&mut self) {
    self.init_threads();


    
  }

  #[cfg(target_os = "emscripten")]
  pub fn init(&mut self) {

    let mut renderer = match Renderer::new(Arc::clone(&self.window)){
      Ok(res) => res,
      Err(_res) => return
    };

    self.renderer = Some(renderer);
  }
  
  pub fn run_events(&mut self) -> bool {
    let mut event_pump = self.window.sdl_context.lock().unwrap().inner.event_pump().unwrap();

    for event in event_pump.poll_iter() {
      match event {
        Event::Quit {..} |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          return false
        },
        _ => {}
      }
    }

    return true
  }

  #[cfg(not(target_os = "emscripten"))]
  fn end(&mut self){
    self.running_events.store(false, Ordering::SeqCst);
    self.running_logic.store(false, Ordering::SeqCst);
    self.running_render.store(false, Ordering::SeqCst);

    self.thread_logic.take().unwrap().join().unwrap();
    self.thread_render.take().unwrap().join().unwrap();
  }

}

impl MainLoop for MainWindow{

  #[cfg(not(target_os = "emscripten"))]
  fn main_loop(&mut self) -> MainLoopEvent{
    if !self.run_events() {
      return MainLoopEvent::Terminate
    }
    std::thread::sleep(Duration::new(0, 1));
    return MainLoopEvent::Continue
  }
  
  #[cfg(target_os = "emscripten")]
  fn main_loop(&mut self) -> MainLoopEvent{
    if !self.run_events() {
      return MainLoopEvent::Terminate
    }
    else {
      MainWindow::run_logic();

      self.renderer.as_mut().unwrap().run();
    }

    return MainLoopEvent::Continue
  }
}

impl Drop for MainWindow {
  fn drop(&mut self) { 
    #[cfg(not(target_os = "emscripten"))]
    self.end();

    println!("MainWindow is being dropped.");
  }
}

impl Drop for Renderer {
  fn drop(&mut self) {
      println!("Renderer is being dropped.");
  }
}
