extern crate static_assertions as sa;

use crate::gpu::renderer_types::*;
use murmur3::murmur3_32;
use std::io::Cursor;

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
}

pub trait UniformShader{
  fn any(&mut self) -> &mut dyn std::any::Any;
}

pub struct UniformMaterial{
  name: UniformName,
  data: UniformData,
}

#[allow(dead_code)]
impl UniformMaterial {
  pub fn new<T: 'static + GetType>(a_name: &str, a_data: T) -> UniformMaterial{
    UniformMaterial{
      name: UniformName::new(a_name), 
      data: UniformData::new::<T>(a_data)
    }
  }
}

impl Uniform for UniformMaterial {
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
