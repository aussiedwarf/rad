use ash::{vk, Entry};
use glam::*;

use crate::gpu::vulkan::vulkan_instance::*;
use crate::gpu::renderer::*;
use crate::gpu::renderer_types::*;
use crate::gpu::material::*;
use crate::gpu::camera::*;
use crate::gpu::uniforms::*;
use crate::gpu::image::*;
use std::ffi::{CString, CStr};
use std::rc::Rc;
use libc::{c_char};

pub struct SamplerVulkan{
  name: String,
}

impl Sampler for SamplerVulkan {
  fn any(&self) -> &dyn std::any::Any{
    self
  }

  fn set_name(&mut self, a_name: &str){
    self.name = String::from(a_name);
  }
}

#[allow(dead_code)]
pub struct VerticesVulkan {
  id: i32
}

impl Vertices for VerticesVulkan {
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

#[allow(dead_code)]
pub struct GeometryVulkan {
  id: i32
}

impl Geometry for GeometryVulkan {
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

#[allow(dead_code)]
pub struct TextureVulkan {
  id: i32
}

impl Texture for TextureVulkan {
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

#[allow(dead_code)]
pub struct UniformVulkan {
  name: UniformName,
  data: UniformData,
}

impl Uniform for UniformVulkan {
  fn any(&mut self) -> &mut dyn std::any::Any{
    self
  }

  fn set_f32(&mut self, _a: f32){
  }

  fn get_f32(&self) -> f32{
    0.0
  }
 
  fn get_name(&self) -> &str{
    &self.name.get_name()
  }
  
  fn set_name(&mut self, a_name: &str){
    self.name.set_name(a_name);
  }
}

#[allow(dead_code)]
pub struct UniformShaderVulkan {
  name: UniformName,
  id: i32 //todo check type
}

impl UniformShader for UniformShaderVulkan {
  fn any(&mut self) -> &mut dyn std::any::Any{
    self
  }
}

pub struct RendererVulkan {
  pub version_major: i32,
  pub version_minor: i32,
  pub version_patch: i32,

  clear_color: Vec4,
  clear_depth: f32,
  clear_stencil: i32,

  instance: VulkanInstance
}

#[allow(dead_code)]
impl Renderer for RendererVulkan {
  fn name(&self) -> String{
    String::from("Vulkan")
  }

  fn get_type(&self) -> RendererType{
    RendererType::Vulkan
  }

  fn begin_frame(&mut self, _clear: RendererClearType){}
  fn end_frame(&mut self){}

  //clear immediatly
  //= RendererClearColor | RendererClearDepth | RendererClearStencil
  fn clear(&mut self, _clear: RendererClearType){}

  //Get and set clear values may be called before BeginFrame
  fn set_clear_color(&mut self, _color: Vec4){}
  fn set_clear_depth(&mut self, _depth: f32){}
  fn set_clear_stencil(&mut self, _stencil: i32){}
  fn get_clear_color(&self) -> Vec4{
    self.clear_color
  }
  fn get_clear_depth(&self) -> f32{
    self.clear_depth
  }

  fn get_clear_stencil(&self) -> i32{
    self.clear_stencil
  }

  fn set_viewport(&mut self, _pos: IVec2, _size: IVec2){}

  fn get_viewport_pos(&self) -> IVec2{
    IVec2::new(0,0)
  }
  fn get_viewport_size(&self) -> IVec2{
    IVec2::new(0,0)
  }

  fn load_shader(&mut self, _shader_type: ShaderType, _source: &str) -> Result<Box<dyn Shader>, RendererError>{
    Err(RendererError::Unimplemented)
  }

  fn load_program_vert_frag(&mut self, _shader_vert: Box<dyn Shader>, _shader_frag: Box<dyn Shader>) -> Result<Box<dyn Program>, RendererError>{
    Err(RendererError::Unimplemented)
  }

  fn get_uniform(&mut self, _shader: &mut Box<dyn Program>, a_name: &str) -> Box<dyn UniformShader>{
    Box::new(UniformShaderVulkan{name: UniformName::new(a_name), id: 0})
  }

  //fn set_uniform(&mut self, a_uniform: &Box<dyn Uniform>){}

  //fn set_texture(&mut self, a_texture: &Box<dyn Texture>){}

  fn gen_buffer_vertex(&mut self, _verts: &std::vec::Vec<f32>) -> Box<dyn Vertices>{
    Box::new(VerticesVulkan{id: 0})
  }

  fn gen_geometry(&mut self, _buffer: &Box<dyn Vertices>) -> Box<dyn Geometry>{
    Box::new(GeometryVulkan{id: 0})
  }

  fn gen_mesh(&mut self, a_geometry: Box<dyn Geometry>, a_material: Box<dyn Material>) -> Box<Mesh>{
    Box::new(Mesh{
      geometry: a_geometry,
      material: a_material
      })
  }


  fn gen_buffer_texture(&mut self) -> Box<dyn Texture>{
    Box::new(TextureVulkan{id: 0})
  }

  fn gen_sampler(&mut self, _texture: Rc<dyn Texture>) -> Box<dyn Sampler>{
    Box::new(SamplerVulkan{name: String::from("")})
  }

  fn load_texture(&mut self, _image: &image::DynamicImage, _texture: &mut Box<dyn Texture>){
  }

  fn use_program(&mut self, _program: &Box<dyn Program>){}

  fn draw_geometry(&mut self, _geometry: &Box<dyn Geometry>){}
  fn draw_mesh(&mut self, _camera: &Camera, _geometry: &mut Box<Mesh>){}

  fn read_render_buffer(&mut self) -> Image{
    return Image{width: 0, height: 0, pitch: 0, pixels: std::vec::Vec::<u8>::new()}
  }
}

impl RendererVulkan{
  pub fn new(a_window: &sdl2::video::Window, a_enable_validation_layers: bool) -> Result<Self, RendererError>{
    /*
    let extensions = match a_window.vulkan_instance_extensions(){
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    };
    */

    let entry = unsafe { match Entry::load()  {
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    }};

    let instance = match VulkanInstance::new(&entry, a_enable_validation_layers){
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    };
    
    let mut major = 1;
    let mut minor = 0;
    let mut patch = 0;
    
    match entry.try_enumerate_instance_version().unwrap() {
      Some(version) => {
        major = vk::api_version_major(version) as i32;
        minor = vk::api_version_minor(version) as i32;
        patch = vk::api_version_patch(version) as i32;
      },
      None => {},
    };

    Ok(Self {
      version_major: major,
      version_minor: minor,
      version_patch: patch,
      clear_color: Vec4::new(0.0, 0.0, 0.0, 0.0),
      clear_depth: 1.0,
      clear_stencil: 0,
      instance: instance
    })
  }

}

impl Drop for RendererVulkan{
  fn drop(&mut self){
    
  }
}
