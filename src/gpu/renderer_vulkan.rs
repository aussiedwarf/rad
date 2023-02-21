
use glam::*;

use crate::gpu::renderer::*;
use crate::gpu::material::*;
use crate::gpu::camera::*;
use std::rc::Rc;

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

pub struct VerticesVulkan {
  id: i32
}

impl Vertices for VerticesVulkan {
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

pub struct GeometryVulkan {
  id: i32
}

impl Geometry for GeometryVulkan {
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

pub struct TextureVulkan {
  id: i32
}

impl Texture for TextureVulkan {
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

pub struct UniformVulkan {
  id: i32,
  name: String,
}

impl Uniform for UniformVulkan {
  
  fn any(&mut self) -> &mut dyn std::any::Any{
    self
  }

  fn set_f32(&self, a: f32){}
  fn set_vec2f32(&self, a: Vec2){}
  fn set_vec3f32(&self, a: Vec3){}
  fn set_vec4f32(&self, a: Vec4){}
  fn set_mat4x4f32(&self, a: Mat4){}
  
  fn get_name(&self) -> &str{&self.name}
}

pub struct RendererVulkan {
  pub version_major: i32,

  clear_color: Vec4,
  clear_depth: f32,
  clear_stencil: i32
}


impl Renderer for RendererVulkan {
  fn name(&self) -> String{
    String::from("Vulkan")
  }

  fn get_type(&self) -> RendererType{
    RendererType::Vulkan
  }

  fn begin_frame(&mut self, a_clear: RendererClearType){}
  fn end_frame(&mut self){}

  //clear immediatly
  //= RendererClearColor | RendererClearDepth | RendererClearStencil
  fn clear(&mut self, a_clear: RendererClearType){}

  //Get and set clear values may be called before BeginFrame
  fn set_clear_color(&mut self, a_color: Vec4){}
  fn set_clear_depth(&mut self, a_depth: f32){}
  fn set_clear_stencil(&mut self, a_stencil: i32){}
  fn get_clear_color(&self) -> Vec4{
    self.clear_color
  }
  fn get_clear_depth(&self) -> f32{
    self.clear_depth
  }

  fn get_clear_stencil(&self) -> i32{
    self.clear_stencil
  }

  fn set_viewport(&mut self, a_pos: IVec2, a_size: IVec2){}

  fn get_viewport_pos(&self) -> IVec2{
    IVec2::new(0,0)
  }
  fn get_viewport_size(&self) -> IVec2{
    IVec2::new(0,0)
  }

  fn load_shader(&mut self, a_shader_type: ShaderType, a_source: &str) -> Result<Box<dyn Shader>, RendererError>{
    Err(RendererError::Unimplemented)
  }

  fn load_program_vert_frag(&mut self, a_shader_vert: Box<dyn Shader>, a_shader_frag: Box<dyn Shader>) -> Result<Box<dyn Program>, RendererError>{
    Err(RendererError::Unimplemented)
  }

  fn get_uniform(&mut self, a_shader: &mut Box<dyn Program>, a_name: &str) -> Box<dyn Uniform>{
    Box::new(UniformVulkan{id: 0, name: a_name.to_string()})
  }

  //fn set_uniform(&mut self, a_uniform: &Box<dyn Uniform>){}

  //fn set_texture(&mut self, a_texture: &Box<dyn Texture>){}

  fn gen_buffer_vertex(&mut self, a_verts: &std::vec::Vec<f32>) -> Box<dyn Vertices>{
    Box::new(VerticesVulkan{id: 0})
  }

  fn gen_geometry(&mut self, a_buffer: &Box<dyn Vertices>) -> Box<dyn Geometry>{
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

  fn gen_sampler(&mut self, a_texture: Rc<dyn Texture>) -> Box<dyn Sampler>{
    Box::new(SamplerVulkan{name: String::from("")})
  }

  fn load_texture(&mut self, a_image: &image::DynamicImage, a_texture: &mut Box<dyn Texture>){
  }

  fn use_program(&mut self, a_program: &Box<dyn Program>){}

  fn draw_geometry(&mut self, a_geometry: &Box<dyn Geometry>){}
  fn draw_mesh(&mut self, a_camera: &Camera, a_geometry: &mut Box<Mesh>){}
}

impl RendererVulkan{
  pub fn new() -> Result<Self, RendererError>{
    Ok(Self {
      version_major: 0,
      clear_color: Vec4::new(0.0, 0.0, 0.0, 0.0),
      clear_depth: 1.0,
      clear_stencil: 0
    })
  }
}
