extern crate sdl2;
//extern crate raw_window_handle;
extern crate libc;

use rad::gpu::camera::*;
use rad::gpu::renderer_types;
use rad::gpu::material;
use rad::gui::window::*;

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

pub struct MainWindow{
  window: Arc<Window>,

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
        running_events: Arc::new(AtomicBool::new(true)),
        running_logic: Arc::new(AtomicBool::new(true)),
        running_render: Arc::new(AtomicBool::new(true)),
        thread_logic: None,
        thread_render: None
      }),
      Err(res) => return Err(res)
    }
  }

  pub fn init_threads(&mut self) {
    let running_logic = Arc::clone(&self.running_events);

    self.thread_logic = Some(thread::spawn(move|| {
      MainWindow::run_logic(running_logic);
    }));

    let running_render = Arc::clone(&self.running_events);
    let window = Arc::clone(&self.window);

    self.thread_render = Some(thread::spawn(move|| {
      MainWindow::run_render(running_render, window);
    }));
  }

  pub fn run_logic(running: Arc<AtomicBool>) {
    
    while running.load(Ordering::SeqCst){
      
      std::thread::sleep(Duration::new(0, 1));
    }
    print!{"Thread Logic done"};
  }

  pub fn run_render(running: Arc<AtomicBool>, window: Arc<Window>) {
    let mut renderer = match Window::init_renderer(
      window.renderer_type, 
      &(window.video_subsystem.lock().unwrap()).inner,
      &(window.window.lock().unwrap()).inner)
    {
      Ok(res) => res,
      Err(_res) => panic!("Error creating renderer")
    };

    
    let mut file_vertex = match File::open("basic.vert"){
      Ok(res) => res,
      Err(_res) => return
    };
    let mut source_vertex = String::new();
    match file_vertex.read_to_string(&mut source_vertex){
      Ok(res) => res,
      Err(_res) => return
    };

    let shader_vertex = match renderer.load_shader(renderer_types::ShaderType::Vertex, source_vertex.as_ref()){
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

    let shader_frag = match renderer.load_shader(renderer_types::ShaderType::Fragment, &mut source_frag){
      Ok(res) => res,
      Err(_res) => return
    };

    let shader_program = match renderer.load_program_vert_frag(shader_vertex, shader_frag){
      Ok(res) => res,
      Err(_res) => return
    };

    let img = match image::open("image.jpg"){
      Ok(res) => res,
      Err(_res) => return
    };

    //only need to flip with opengl
    let img = img.flipv();

    let mut texture = renderer.gen_buffer_texture();

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

    //let uniform_mvp = self.renderer.get_uniform(&mut shader_program, "u_mvp");

    //material.set_color_texture(&texture);

    let mut mesh = renderer.gen_mesh(geometry, material);
    
    renderer.set_clear_color(Vec4::new(0.0, 0.0, 0.0, 1.0));
    renderer.set_viewport(IVec2::new(0,0), IVec2::new(window.width as i32, window.height as i32));

    //self.renderer.use_program(&shader_program);
    //self.renderer.set_uniform(&uniform);
    //self.renderer.set_texture(&texture);

    let mut camera = Camera::new();
    camera.set_viewport(Vec2::new(window.width as f32, window.height as f32), 
      Vec2::ZERO, Vec2::new(window.width as f32, window.height as f32), Vec2::ZERO);


    let mut i = 0;
    let mut r: f32 = 0.0;

    while running.load(Ordering::SeqCst){
      r += 0.01;
      if r > 1.0{
        r = 0.0;
      }

      i = (i + 1) % 255;

      renderer.set_clear_color(Vec4::new(r, 0.0, 0.0, 1.0));

      // The rest of the game loop goes here...
      renderer.clear(renderer_types::RendererClearType::COLOR);

      renderer.draw_mesh(&camera, &mut mesh);
      
      window.window.lock().unwrap().inner.gl_swap_window();
      //self.canvas.present();
      //std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

      std::thread::sleep(Duration::new(0, 1));
    }
    print!{"Thread Render done"};

    
  }
  
  pub fn run(&mut self) {
    self.init_threads();

    let mut event_pump = self.window.sdl_context.lock().unwrap().inner.event_pump().unwrap();

    'running: loop {
      
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
      
    }

    self.running_events.store(false, Ordering::SeqCst);
    self.running_logic.store(false, Ordering::SeqCst);
    self.running_render.store(false, Ordering::SeqCst);

    self.thread_logic.take().unwrap().join().unwrap();
    self.thread_render.take().unwrap().join().unwrap();

    // match self.thread_events{
    //   Some(thread) => {
    //     thread.join();
    //   },
    //   None => {},
    // }
  }
}



