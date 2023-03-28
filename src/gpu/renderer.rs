
extern crate glam;
extern crate static_assertions as sa;

use crate::gpu::material::*;
use crate::gpu::camera::*;


use std::fmt;
use std::rc::Rc;
use glam::*;
use std::io::Cursor;
use murmur3::murmur3_32;


#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum RendererType {
  OpenGL,
  OpenGLES,
  DirectX,
  Vulkan,
  Metal
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum ShaderType{
  Vertex,
  TesselationControl,
  TesselationEvaluation,
  Geometry,
  Fragment,
  Compute
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum RendererError {
  Error,
  ShaderCompile,
  InvalidCast,
  Unimplemented
}


#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum ElementType {
  Float16,
  Float32,
  Float64,
  Int8,
  Int16,
  Int32,
  Int64,
  Uint8,
  Uint16,
  Uint32,
  Uint64
}


#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum ContainerType {
  Single,
  Vec2,
  Vec3,
  Vec4,
  Mat2x2,
  Mat3x3,
  Mat4x4
}

pub trait GetType{
  fn get_element_type(&self) -> ElementType;
  fn get_container_type(&self) -> ContainerType;
}

impl GetType for f32{
  fn get_element_type(&self) -> ElementType{
    ElementType::Float32
  }
  fn get_container_type(&self) -> ContainerType{
    ContainerType::Single
  }
}

impl GetType for Vec2{
  fn get_element_type(&self) -> ElementType{
    ElementType::Float32
  }
  fn get_container_type(&self) -> ContainerType{
    ContainerType::Vec2
  }
}

impl GetType for Vec3{
  fn get_element_type(&self) -> ElementType{
    ElementType::Float32
  }
  fn get_container_type(&self) -> ContainerType{
    ContainerType::Vec3
  }
}

impl GetType for Vec4{
  fn get_element_type(&self) -> ElementType{
    ElementType::Float32
  }
  fn get_container_type(&self) -> ContainerType{
    ContainerType::Vec4
  }
}
/*
impl GetType for Mat2{
  fn get_element_type(&self) -> ElementType{
    ElementType::Float32
  }
  fn get_container_type(&self) -> ContainerType{
    ContainerType::Mat2x2
  }
}

impl GetType for Mat3{
  fn get_element_type(&self) -> ElementType{
    ElementType::Float32
  }
  fn get_container_type(&self) -> ContainerType{
    ContainerType::Mat3x3
  }
}

impl GetType for Mat4{
  fn get_element_type(&self) -> ElementType{
    ElementType::Float32
  }
  fn get_container_type(&self) -> ContainerType{
    ContainerType::Mat4x4
  }
}
*/

/*
#[derive(Copy, Clone)]
pub enum RendererClearType{
  RendererClearColor = 0x1,
  RendererClearDepth = 0x2,
  RendererClearStencil = 0x4
}
*/

bitflags! {
  //#[allow(non_upper_case_globals)]
  #[allow(dead_code, non_upper_case_globals)]
  pub struct RendererClearType: u32 {
    const None = 0b00000000;
    const Color = 0b00000001;
    const Depth = 0b00000010;
    const Stencil = 0b00000100;
    //const ABC = Self::A.bits | Self::B.bits | Self::C.bits;
  }
}

impl std::error::Error for RendererError {}

impl fmt::Display for RendererError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      RendererError::Error => write!(f, "Error"),
      RendererError::Unimplemented => write!(f, "Error Unimplemented"),
      RendererError::InvalidCast => write!(f, "Error InvalidCast"),
      RendererError::ShaderCompile => write!(f, "Error ShaderCompile"),
    }
  }
}


pub trait Program{
  fn any(&self) -> &dyn std::any::Any;
}


pub trait Shader{
  fn any(&self) -> &dyn std::any::Any;
}


pub trait Texture{
  fn any(&self) -> &dyn std::any::Any;
}


pub trait Vertices{
  fn any(&self) -> &dyn std::any::Any;
}


pub trait Geometry{
  fn any(&self) -> &dyn std::any::Any;
}

pub trait UniformBase{
  fn any(&self) -> &dyn std::any::Any;
}

#[allow(dead_code)]
pub struct UniformType{
  pub element_type: ElementType,
  pub container_type: ContainerType,
  pub num_components: u16
}

sa::const_assert!(std::mem::size_of::<UniformType>() == 4);

type UniformHash = u32;

pub struct UniformName{
  name: String,
  hash: UniformHash,
}

impl UniformName{
  pub fn new(a_name: &str) -> UniformName{
    UniformName{name: String::from(a_name), hash: UniformName::calculate_hash(a_name)}
  }
  
  pub fn get_name(&self) -> &str{
    &self.name
  }

  pub fn get_hash(&self) -> UniformHash{
    self.hash
  }

  pub fn set_name(&mut self, a_name: &str){
    self.name = String::from(a_name);
    self.hash = UniformName::calculate_hash(a_name);
  }

  fn calculate_hash(a_name: &str) -> UniformHash{
    match murmur3_32(&mut Cursor::new(a_name), 0) {
      Ok(res) => res,
      Err(_res) => 0
    }
  }
}

impl PartialEq for UniformName {
  fn eq(&self, other: &Self) -> bool {
    assert!(self.name == other.name);
    return self.hash == other.hash;
  }
}


pub trait UniformDataDyn{
  fn any_mut(&mut self) -> &mut dyn std::any::Any;
  fn any(&self) -> &dyn std::any::Any;
}

pub struct UniformDataGen<T>{
  pub data: T,
}

impl<T: 'static> UniformDataDyn for UniformDataGen<T>{
  fn any_mut(&mut self) -> &mut dyn std::any::Any{
    self
  }
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

pub struct UniformData{
  pub info: UniformType,
  pub data: Box<dyn UniformDataDyn>,
}

impl UniformData{
  pub fn new<T: 'static + GetType>(a: T) -> UniformData{
    UniformData{
      info: UniformType{element_type: a.get_element_type(), container_type: a.get_container_type(), num_components: 1},
      data: Box::new(UniformDataGen::<T>{data: a})
    }
  }

  pub fn set<T: 'static>(&mut self, a: T){
    let mut uniform = match self.data.any_mut().downcast_mut::<UniformDataGen<T>>() {
      Some(res) => res,
      None => panic!("Invalid cast")
    };
    uniform.data = a;
  }

  pub fn get<T: 'static>(&self) -> T
    where T: Copy,
  {
    let uniform = match self.data.any().downcast_ref::<UniformDataGen<T>>() {
      Some(res) => res,
      None => panic!("Invalid cast")
    };
    let data = uniform.data;
    data
  }
/*
  fn set_f32(&self, a: f32){
    assert!(self.info.component_type == ComponentType::Single);
    assert!(self.info.variable_type == VariableType::Float32);
    assert!(self.info.num_components == 1);

    let mut data = match self.data.any().downcast_ref::<f32>() {
      Some(res) => res,
      None => panic!("Invalid cast")
    };
    *data = a;
  }
  fn set_vec2f32(&self, a: Vec2){
    assert!(self.info.component_type == ComponentType::Vec2);
    assert!(self.info.variable_type == VariableType::Float32);
    assert!(self.info.num_components == 1);
    
    let mut data = match self.data.any().downcast_ref::<Vec2>() {
      Some(res) => res,
      None => panic!("Invalid cast")
    };
    *data = a;
  }
  fn set_vec3f32(&self, a: Vec3){
    assert!(self.info.component_type == ComponentType::Vec3);
    assert!(self.info.variable_type == VariableType::Float32);
    assert!(self.info.num_components == 1);

    self.data = a.to_ne_bytes().to_vec();
  }
  fn set_vec4f32(&self, a: Vec4){
    assert!(self.info.component_type == ComponentType::Vec4);
    assert!(self.info.variable_type == VariableType::Float32);
    assert!(self.info.num_components == 1);

    self.data = a.to_ne_bytes().to_vec();
  }
  fn set_mat4x4f32(&self, a: Mat4){
    assert!(self.info.component_type == ComponentType::Mat4x4);
    assert!(self.info.variable_type == VariableType::Float32);
    assert!(self.info.num_components == 1);

    self.data = a.to_ne_bytes().to_vec();
  }
  */
}

pub trait UniformShader{
  fn any(&mut self) -> &mut dyn std::any::Any;
}

pub trait Uniform{
  fn any(&mut self) -> &mut dyn std::any::Any;

  fn set_f32(&mut self, a: f32);

  fn get_f32(&self) -> f32;

  fn get_name(&self) -> &str;
  fn set_name(&mut self, a_name: &str);

}

pub trait Sampler{
  fn any(&self) -> &dyn std::any::Any;

  fn set_name(&mut self, a_name: &str);
}

pub struct Mesh{
  pub geometry: Box<dyn Geometry>,
  pub material: Box<dyn Material>
}

pub trait Renderer {
  fn name(&self) -> String;
  fn get_type(&self) -> RendererType;

  //Frame to begin rendering. Render calls may now be made. Set whether to clear screen at render start
  //Reason on clearing here is vulkan rendering system has faster clear on start render
  //RendererClearColor | RendererClearDepth | RendererClearStencil
  fn begin_frame(&mut self, a_clear: RendererClearType);
  fn end_frame(&mut self);

  //clear immediatly
  //= RendererClearColor | RendererClearDepth | RendererClearStencil
  fn clear(&mut self, a_clear: RendererClearType);

  //Get and set clear values may be called before BeginFrame

  //color bgra
  fn set_clear_color(&mut self, a_color: Vec4);
  fn set_clear_depth(&mut self, a_depth: f32);
  fn set_clear_stencil(&mut self, a_stencil: i32);

  //color bgra
  fn get_clear_color(&self) -> Vec4;
  fn get_clear_depth(&self) -> f32;
  fn get_clear_stencil(&self) -> i32;

  fn set_viewport(&mut self, a_pos: IVec2, a_size: IVec2);
  fn get_viewport_pos(&self) -> IVec2;
  fn get_viewport_size(&self) -> IVec2;
  
  fn load_shader(&mut self, a_shader_type: ShaderType, a_source: &str) -> Result<Box<dyn Shader>, RendererError>;
  fn load_program_vert_frag(&mut self, a_shader_vert: Box<dyn Shader>, a_shader_frag: Box<dyn Shader>) -> Result<Box<dyn Program>, RendererError>;

  fn get_uniform(&mut self, a_shader: &mut Box<dyn Program>, a_name: &str) -> Box<dyn UniformShader>;
  //fn set_uniform(&mut self, a_uniform: &Box<dyn Uniform>);
  //fn set_texture(&mut self, a_texture: &Box<dyn Texture>);
  /*
  fn load_program_compute(&mut self, a_shader: Box<dyn Shader>) -> Result<Box<dyn Program>, RendererError>;
  */

  fn gen_buffer_vertex(&mut self, a_verts: &std::vec::Vec<f32>) -> Box<dyn Vertices>;

  fn gen_geometry(&mut self, a_buffer: &Box<dyn Vertices>) -> Box<dyn Geometry>;

  fn gen_mesh(&mut self, a_geometry: Box<dyn Geometry>, a_material: Box<dyn Material>) -> Box<Mesh>;

  fn gen_buffer_texture(&mut self) -> Box<dyn Texture>;

  fn gen_sampler(&mut self, a_texture: Rc<dyn Texture>) -> Box<dyn Sampler>;

  fn load_texture(&mut self, a_image: &image::DynamicImage, a_texture: &mut Box<dyn Texture>);

  fn use_program(&mut self, a_program: &Box<dyn Program>);

  fn draw_geometry(&mut self, a_geometry: &Box<dyn Geometry>);
  fn draw_mesh(&mut self, a_camera: &Camera, a_mesh: &mut Box<Mesh>);
}
