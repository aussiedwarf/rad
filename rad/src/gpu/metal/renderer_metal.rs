

use std::ffi::{CString};
use std::rc::Rc;
use std::sync::Arc;
use glam::*;

use crate::gpu::renderer::*;
use crate::gpu::renderer_types::*;
use crate::gpu::material::*;
use crate::gpu::camera::*;
use crate::gpu::uniforms::*;
use crate::gui::window::Window;
use crate::gpu::image::*;

pub struct SamplerMetal{
  name: String,
  texture: Rc<dyn Texture>,
}

impl Sampler for SamplerMetal {
  fn any(&self) -> &dyn std::any::Any{
    self
  }

  fn set_name(&mut self, a_name: &str){
    self.name = String::from(a_name);
  }
}

pub struct ProgramMetal {
}

impl Program for ProgramMetal {
  fn any(&self) -> &dyn std::any::Any{
    self
  }

  fn get_uniform(&self, a_name: &str, a_data: UniformData) -> Box<dyn Uniform>{
    let c_str = match CString::new(a_name){
      Ok(res) => res,
      Err(_res) => panic!("Invalid text cast")
    };

    Box::new(UniformMetal{
      name: UniformName::new(a_name),
      data: a_data,
      modified: true
    })
  }
}

pub struct ShaderMetal {
}

impl Shader for ShaderMetal {
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

pub struct VerticesMetal {

}

impl Vertices for VerticesMetal {
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

pub struct GeometryMetal {

}

impl Geometry for GeometryMetal {
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

#[allow(dead_code)]
pub struct TextureMetal {
  width: u32,
  height: u32
}

impl Texture for TextureMetal {
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

#[allow(dead_code)]
pub struct UniformMetal {
  name: UniformName,
  data: UniformData,
  modified: bool,
}

#[allow(dead_code)]
impl UniformMetal {
  pub fn new<T: 'static + GetType>(a_name: &str, a_data: T) -> UniformMetal{
    UniformMetal{
      name: UniformName::new(a_name), 
      data: UniformData::new::<T>(a_data),
      modified: true
    }
  }
}

#[allow(dead_code)]
impl Uniform for UniformMetal {
  fn any(&mut self) -> &mut dyn std::any::Any{
    self
  }

  fn set_f32(&mut self, a: f32){
    self.data.set::<f32>(a);
  }

  fn get_f32(&self) -> f32{
    self.data.get::<f32>()
  }
  
  fn get_name(&self) -> &str{
    &self.name.get_name()
  }
  
  fn set_name(&mut self, a_name: &str){
    self.name.set_name(a_name);
  }
}

#[allow(dead_code)]
pub struct UniformShaderMetal {
  name: UniformName,
}

impl UniformShader for UniformShaderMetal {
  fn any(&mut self) -> &mut dyn std::any::Any{
    self
  }
}

pub struct RendererMetal {

  window: Arc<Window>,

  clear_color: Vec4,
  clear_depth: f32,
  clear_stencil: i32,

  viewport_pos: IVec2,
  viewport_size: IVec2,

}

#[allow(dead_code)]
impl Renderer for RendererMetal {
  fn name(&self) -> String{
    String::from("Metal")
  }

  fn get_type(&self) -> RendererType{
    RendererType::Metal
  }

  fn begin_frame(&mut self, a_clear: RendererClearType){
    self.clear(a_clear);
  }

  fn end_frame(&mut self){

  }

  //clear immediatly
  fn clear(&mut self, a_clear: RendererClearType){

  }

  // Get and set clear values may be called before BeginFrame
  fn set_clear_color(&mut self, a_color: Vec4){
    self.clear_color = a_color;
  }

  fn set_clear_depth(&mut self, a_depth: f32){
    self.clear_depth = a_depth;
  }

  fn set_clear_stencil(&mut self, a_stencil: i32){
    self.clear_stencil = a_stencil;
  }

  fn get_clear_color(&self) -> Vec4{
    self.clear_color
  }

  fn get_clear_depth(&self) -> f32{
    self.clear_depth
  }

  fn get_clear_stencil(&self) -> i32{
    self.clear_stencil
  }

  fn set_viewport(&mut self, a_pos: IVec2, a_size: IVec2){
    self.viewport_pos = a_pos;
    self.viewport_size = a_size;
  }

  fn get_viewport_pos(&self) -> IVec2{
    self.viewport_pos
  }
  fn get_viewport_size(&self) -> IVec2{
    self.viewport_size
  }

  fn load_shader(&mut self, a_shader_type: ShaderType, a_source: &str) -> Result<Box<dyn Shader>, RendererError>{
    

    Ok(Box::new(ShaderMetal{}))
  }

  fn load_shader_intermediate(&mut self, a_shader_type: ShaderType, a_source: &std::vec::Vec::<u8>) -> Result<Box<dyn Shader>, RendererError>{
    return Err(RendererError::Unimplemented)
  }

  fn load_program_vert_frag(&mut self, a_shader_vert: Box<dyn Shader>, a_shader_frag: Box<dyn Shader>) -> Result<Box<dyn Program>, RendererError>{

    Ok(Box::new(ProgramMetal{}))
  }

  fn get_uniform(&mut self, a_shader: &mut Box<dyn Program>, a_name: &str) -> Box<dyn UniformShader>{
    Box::new(UniformShaderMetal{
      name: UniformName::new(a_name)
    })
  }


  fn gen_buffer_vertex(&mut self, a_verts: &std::vec::Vec<f32>) -> Box<dyn Vertices>{

    Box::new(VerticesMetal{})
  }

  fn gen_geometry(&mut self, a_buffer: &Box<dyn Vertices>) -> Box<dyn Geometry>{
    
    Box::new(GeometryMetal{})
  }

  fn gen_mesh(&mut self, a_geometry: Box<dyn Geometry>, a_material: Box<dyn Material>) -> Box<Mesh>{
    Box::new(Mesh{
      geometry: a_geometry,
      material: a_material
      })
  }

  fn gen_buffer_texture(&mut self) -> Box<dyn Texture>{
    Box::new(TextureMetal{
      width: 0,
      height: 0})
  }

  fn gen_sampler(&mut self, a_texture: Rc<dyn Texture>) -> Box<dyn Sampler>{
    let sampler = SamplerMetal{name: String::from(""), texture: a_texture};

    Box::new(sampler)
  }

  fn load_texture(&mut self, a_image: &image::DynamicImage, a_texture: &mut Box<dyn Texture>){
  }

  fn use_program(&mut self, a_program: &Box<dyn Program>){
  }

  fn draw_geometry(&mut self, a_geometry: &Box<dyn Geometry>){
    
  }

  fn draw_mesh(&mut self, _camera: &Camera, a_mesh: &mut Box<Mesh>){
    
  }

  fn read_render_buffer(&mut self) -> Image {
    let mut image = Image{
      width: self.window.width, 
      height: self.window.height, 
      pitch: self.window.height * 4, 
      pixels: vec![0u8; (self.window.width * self.window.height  * 4) as usize]};
    
    return image
  }

}

#[allow(dead_code)]
impl RendererMetal {
  pub fn new(
    a_video_subsystem: &sdl2::VideoSubsystem, 
    a_window: Arc<Window>) -> Result<Self, RendererError>
  {
    

    Ok(Self {
      window: a_window,
      clear_color: Vec4::new(0.0, 0.0, 0.0, 0.0),
      clear_depth: 1.0,
      clear_stencil: 0,
      viewport_pos: IVec2::new(0,0),
      viewport_size: IVec2::new(0,0),
    })
  }

  pub fn update_uniform(&self, a_uniform: &mut Box<dyn Uniform>){
  }

  pub fn update_sampler(&self, a_sampler: &Box<dyn Sampler>){
  }
}

impl Drop for ShaderMetal {
  fn drop(&mut self) {
  }
}


impl Drop for ProgramMetal {
  fn drop(&mut self) {
  }
}

impl Drop for VerticesMetal {
  fn drop(&mut self) {
  }
}

impl Drop for TextureMetal {
  fn drop(&mut self) {
  }
}
