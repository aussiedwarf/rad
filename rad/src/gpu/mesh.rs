use crate::gpu::material::*;
use crate::gpu::resource::*;
use crate::gpu::uniforms::*;

use std::sync::Arc;

pub trait Program {
    fn any(&self) -> &dyn std::any::Any;

    fn get_uniform(&self, a_name: &str, a_data: UniformData) -> Box<dyn Uniform>;
}

pub trait Shader {
    fn any(&self) -> &dyn std::any::Any;
}

pub trait Texture {
    fn any(&self) -> &dyn std::any::Any;
}

pub trait Vertices {
    fn any(&self) -> &dyn std::any::Any;
}

pub trait Geometry {
    fn any(&self) -> &dyn std::any::Any;
}

pub struct Mesh {
    pub geometry: Arc<dyn Geometry>,
    pub material: Arc<dyn Material>,
}

impl Resource for Mesh {}
