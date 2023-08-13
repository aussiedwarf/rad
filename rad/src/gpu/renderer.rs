
extern crate glam;

use crate::gpu::renderer_types::*;
use crate::gpu::uniforms::*;
use crate::gpu::material::*;
use crate::gpu::camera::*;

use glam::*;
use std::rc::Rc;

pub trait Program{
  fn any(&self) -> &dyn std::any::Any;

  fn get_uniform(&self, a_name: &str, a_data: UniformData) -> Box<dyn Uniform>;
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

  // set clear color rgba
  fn set_clear_color(&mut self, a_color: Vec4);
  fn set_clear_depth(&mut self, a_depth: f32);
  fn set_clear_stencil(&mut self, a_stencil: i32);

  //color rgba
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

  //fn gen_instances(&mut self, Box<Mesh>, u32 a_num_instances) -> Box<Instances>;  //should return instances object, or vector of instances?

  fn gen_buffer_texture(&mut self) -> Box<dyn Texture>;

  fn gen_sampler(&mut self, a_texture: Rc<dyn Texture>) -> Box<dyn Sampler>;

  fn load_texture(&mut self, a_image: &image::DynamicImage, a_texture: &mut Box<dyn Texture>);

  fn use_program(&mut self, a_program: &Box<dyn Program>);

  fn draw_geometry(&mut self, a_geometry: &Box<dyn Geometry>);
  fn draw_mesh(&mut self, a_camera: &Camera, a_mesh: &mut Box<Mesh>);

  //fn gen_render_target
  //fn set_render_target
  //fn clear_render_target
}
