
extern crate glam;

use glam::*;
use std::fmt;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum DeviceType {
  Default,
  HighPerformance,
  LowPower
}

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
  UnsupportedAPI,
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


/*
#[derive(Copy, Clone)]
pub enum RendererClearType{
  RendererClearColor = 0x1,
  RendererClearDepth = 0x2,
  RendererClearStencil = 0x4
}
*/

bitflags! {
  #[allow(dead_code)]
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
  pub struct RendererClearType: u32 {
    const NONE = 0b00000000;
    const COLOR = 0b00000001;
    const DEPTH = 0b00000010;
    const STENCIL = 0b00000100;
  }
}

impl std::error::Error for RendererError {}

impl fmt::Display for RendererError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      RendererError::Error => write!(f, "Error"),
      RendererError::InvalidCast => write!(f, "Error InvalidCast"),
      RendererError::ShaderCompile => write!(f, "Error ShaderCompile"),
      RendererError::UnsupportedAPI => write!(f, "Error UnsupportedAPI"),
      RendererError::Unimplemented => write!(f, "Error Unimplemented"),
    }
  }
}
