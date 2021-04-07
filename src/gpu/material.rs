
use crate::gpu::renderer::*;

pub trait Material{
  fn any(&self) -> &dyn std::any::Any;

  fn num_uniforms(&self) -> usize;
  fn num_samplers(&self) -> usize;

  fn get_uniform(&mut self, a_index: usize) -> &mut Box<dyn Uniform>;
  //fn set_uniform(&self, a_index: usize, a_uniform: dyn Uniform);
  //fn add_uniform(&self, a_uniform: dyn Uniform);

  fn get_sampler(&mut self, a_index: usize) -> &mut Box<dyn Sampler>;
  //fn set_texture(&self, a_index: usize, a_uniform: dyn Sampler);
  //fn add_texture(&self, a_uniform: dyn Sampler);

  fn get_program(&self) -> &Box<dyn Program>;
}

impl Material for MaterialBasic {
  fn any(&self) -> &dyn std::any::Any{
    self
  }

  fn num_uniforms(&self) -> usize{
    self.uniforms.len()
  }
  fn num_samplers(&self) -> usize{
    self.samplers.len()
  }

  fn get_uniform(&mut self, a_index: usize) -> &mut Box<dyn Uniform>{
    &mut self.uniforms[a_index]
  }
  //fn set_uniform(&self, a_index: usize, a_uniform: dyn Uniform){}
  //fn add_uniform(&self, a_uniform: dyn Uniform){}

  fn get_sampler(&mut self, a_index: usize) -> &mut Box<dyn Sampler>{
    &mut self.samplers[a_index]
  }
  //fn set_texture(&self, a_index: usize, a_uniform: dyn Sampler){}
  //fn add_texture(&self, a_uniform: dyn Sampler){}

  fn get_program(&self) -> &Box<dyn Program>{
    &self.program
  }

}

pub struct MaterialBasic{
  program: Box<dyn Program>,
  uniforms: std::vec::Vec<Box<dyn Uniform>>,
  samplers: std::vec::Vec<Box<dyn Sampler>>,
}

impl MaterialBasic{
  pub fn new(a_program: Box<dyn Program>, a_sampler: Box<dyn Sampler>) -> Self{
    let mut samplers = std::vec::Vec::new();

    samplers.push(a_sampler);

    let mut material = MaterialBasic{program: a_program, uniforms: std::vec::Vec::new(), samplers: samplers};
    material.samplers[0].set_name("u_color");

    material
  }

  pub fn set_color_texture(&self, a_texture: &Box<dyn Texture>){

  }
}


