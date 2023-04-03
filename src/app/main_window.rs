extern crate sdl2;
//extern crate raw_window_handle;
extern crate libc;

use crate::gpu::camera::*;
use crate::gpu::renderer_types;

use crate::gpu::material;

use crate::gui::window::*;

//use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::fs::File;
use std::io::prelude::*;
//use std::rc::Rc;
use glam::*;

pub struct MainWindow{
  window: Window,
}

impl MainWindow {
  pub fn new(a_renderer_type: renderer_types::RendererType) -> Result<MainWindow, WindowError> {
    match Window::new(a_renderer_type, "Title", 800, 600){
      Ok(res) => return Ok(MainWindow{window: res}),
      Err(res) => return Err(res)
    }
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

    let shader_vertex = match self.window.renderer.load_shader(renderer_types::ShaderType::Vertex, source_vertex.as_ref()){
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

    let shader_frag = match self.window.renderer.load_shader(renderer_types::ShaderType::Fragment, &mut source_frag){
      Ok(res) => res,
      Err(_res) => return
    };

    let shader_program = match self.window.renderer.load_program_vert_frag(shader_vertex, shader_frag){
      Ok(res) => res,
      Err(_res) => return
    };

    let img = match image::open("image.jpg"){
      Ok(res) => res,
      Err(_res) => return
    };

    //only need to flip with opengl
    let img = img.flipv();

    let mut texture = self.window.renderer.gen_buffer_texture();

    self.window.renderer.load_texture(&img, &mut texture);

    

    //let uniform = self.renderer.get_uniform(&mut shader_program, "u_texture");

    let verts: std::vec::Vec<f32> = vec![
      -1.0, -1.0, 0.0, 0.0,
      1.0, -1.0, 1.0, 0.0,
      1.0, 1.0, 1.0, 1.0,
      1.0, 1.0, 1.0, 1.0,
      -1.0, 1.0, 0.0, 1.0,
      -1.0, -1.0, 0.0, 0.0];

    let vert_buffer = self.window.renderer.gen_buffer_vertex(&verts);

    let geometry = self.window.renderer.gen_geometry(&vert_buffer);

    let material = Box::new(material::MaterialBasic::new(shader_program, 
      self.window.renderer.gen_sampler(texture.into())));

    //let uniform_mvp = self.renderer.get_uniform(&mut shader_program, "u_mvp");

    //material.set_color_texture(&texture);

    let mut mesh = self.window.renderer.gen_mesh(geometry, material);
    
    self.window.renderer.set_clear_color(Vec4::new(0.0, 0.0, 0.0, 1.0));
    self.window.renderer.set_viewport(IVec2::new(0,0), IVec2::new(self.window.width as i32, self.window.height as i32));

    //self.renderer.use_program(&shader_program);
    //self.renderer.set_uniform(&uniform);
    //self.renderer.set_texture(&texture);

    let mut camera = Camera::new();
    camera.set_viewport(Vec2::new(self.window.width as f32, self.window.height as f32), 
      Vec2::ZERO, Vec2::new(self.window.width as f32, self.window.height as f32), Vec2::ZERO);


    let mut event_pump = self.window.sdl_context.event_pump().unwrap();
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
      self.window.renderer.set_clear_color(Vec4::new(r, 0.0, 0.0, 1.0));

      // The rest of the game loop goes here...
      self.window.renderer.clear(renderer_types::RendererClearType::COLOR);

      self.window.renderer.draw_mesh(&camera, &mut mesh);
      
      self.window.window.gl_swap_window();
      //self.canvas.present();
      ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
  }
}



