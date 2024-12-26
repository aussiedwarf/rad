use crate::gpu::mesh::*;
use crate::gpu::uniforms::*;

use glam::*;
// use std::rc::Rc;
// use std::sync::Arc;

pub trait Material {
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
    fn any(&self) -> &dyn std::any::Any {
        self
    }

    fn num_uniforms(&self) -> usize {
        self.uniforms.len()
    }
    fn num_samplers(&self) -> usize {
        self.samplers.len()
    }

    fn get_uniform(&mut self, a_index: usize) -> &mut Box<dyn Uniform> {
        &mut self.uniforms[a_index]
    }
    //fn set_uniform(&self, a_index: usize, a_uniform: dyn Uniform){}
    //fn add_uniform(&self, a_uniform: dyn Uniform){}

    fn get_sampler(&mut self, a_index: usize) -> &mut Box<dyn Sampler> {
        &mut self.samplers[a_index]
    }
    //fn set_texture(&self, a_index: usize, a_uniform: dyn Sampler){}
    //fn add_texture(&self, a_uniform: dyn Sampler){}

    fn get_program(&self) -> &Box<dyn Program> {
        &self.program
    }
}

#[allow(dead_code)]
pub struct MaterialBasic {
    program: Box<dyn Program>,
    uniforms: std::vec::Vec<Box<dyn Uniform>>,
    samplers: std::vec::Vec<Box<dyn Sampler>>,

    mvp: Mat4,
}

#[allow(dead_code)]
impl MaterialBasic {
    pub fn new(a_program: Box<dyn Program>, a_sampler: Box<dyn Sampler>) -> Self {
        let mut samplers: Vec<Box<dyn Sampler>> = std::vec::Vec::new();
        let mut uniforms: Vec<Box<dyn Uniform>> = std::vec::Vec::new();

        samplers.push(a_sampler);
        samplers[0].set_name("u_color");

        let uniform_mvp = a_program.get_uniform("u_mvp", UniformData::new(Mat4::IDENTITY));

        uniforms.push(uniform_mvp);

        MaterialBasic {
            program: a_program,
            uniforms: uniforms,
            samplers: samplers,
            mvp: Mat4::IDENTITY,
        }
    }

    pub fn set_color_texture(&self, _texture: &Box<dyn Texture>) {}
}
