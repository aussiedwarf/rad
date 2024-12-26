use crate::gpu::mesh::*;
use crate::gpu::uniforms::*;
use super::device::LogicalDevice;

pub struct SamplerVulkan {
    pub name: String,
}

impl Sampler for SamplerVulkan {
    fn any(&self) -> &dyn std::any::Any {
        self
    }

    fn set_name(&mut self, a_name: &str) {
        self.name = String::from(a_name);
    }
}

#[allow(dead_code)]
pub struct VerticesVulkan {
    pub id: i32,
}

impl Vertices for VerticesVulkan {
    fn any(&self) -> &dyn std::any::Any {
        self
    }
}

#[allow(dead_code)]
pub struct GeometryVulkan {
    pub id: i32,
}

impl Geometry for GeometryVulkan {
    fn any(&self) -> &dyn std::any::Any {
        self
    }
}

#[allow(dead_code)]
pub struct TextureVulkan {
    pub id: i32,
}

impl Texture for TextureVulkan {
    fn any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct ShaderVulkan {
    pub module: ash::vk::ShaderModule,
    pub logical_device: std::rc::Rc<LogicalDevice>,
}

impl Shader for ShaderVulkan {
    fn any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Drop for ShaderVulkan {
    fn drop(&mut self) {
        unsafe {
            self.logical_device
                .device
                .destroy_shader_module(self.module, None)
        };
    }
}

pub struct ProgramVulkan {
    pub pipeline: ash::vk::Pipeline,
    pub pipeline_layout: ash::vk::PipelineLayout,
    pub logical_device: std::rc::Rc<LogicalDevice>,
}

impl Program for ProgramVulkan {
    fn any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_uniform(&self, a_name: &str, a_data: UniformData) -> Box<dyn Uniform> {
        Box::new(UniformVulkan {
            name: UniformName::new(a_name),
            data: a_data,
        })
    }
}

impl Drop for ProgramVulkan {
    fn drop(&mut self) {
        unsafe {
            self.logical_device
                .device
                .destroy_pipeline(self.pipeline, None);
            self.logical_device
                .device
                .destroy_pipeline_layout(self.pipeline_layout, None);
        }
    }
}

#[allow(dead_code)]
pub struct UniformVulkan {
    pub name: UniformName,
    pub data: UniformData,
}

impl Uniform for UniformVulkan {
    fn any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn set_f32(&mut self, _a: f32) {}

    fn get_f32(&self) -> f32 {
        0.0
    }

    fn get_name(&self) -> &str {
        &self.name.get_name()
    }

    fn set_name(&mut self, a_name: &str) {
        self.name.set_name(a_name);
    }
}

#[allow(dead_code)]
pub struct UniformShaderVulkan {
    pub name: UniformName,
    pub id: i32, //todo check type
}

impl UniformShader for UniformShaderVulkan {
    fn any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
