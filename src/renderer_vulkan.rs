
use glam::{Vec4, IVec2};

use crate::renderer::*;

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

  fn gen_buffer_vertex(&mut self, a_verts: std::vec::Vec<f32>) -> Box<dyn Vertices>{
    Box::new(VerticesVulkan{id: 0})
  }

  fn gen_geometry(&mut self, a_buffer: Box<dyn Vertices>) -> Box<dyn Geometry>{
    Box::new(GeometryVulkan{id: 0})
  }

  fn use_program(&mut self, a_program: Box<dyn Program>){
  }

  fn draw_geometry(&mut self, a_geometry: &Box<dyn Geometry>){}
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
