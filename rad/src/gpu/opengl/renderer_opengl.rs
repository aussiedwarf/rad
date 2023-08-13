

extern crate gl;

use std::ffi::{CString};
use std::rc::Rc;
use std::sync::Arc;
use glam::*;

use crate::gpu::renderer::*;
use crate::gpu::renderer_types::*;
use crate::gpu::material::*;
use crate::gpu::camera::*;
use crate::gpu::uniforms::*;
use crate::gui::window::Window;
use crate::gpu::image::*;

pub struct SamplerOpenGL{
  name: String,
  texture: Rc<dyn Texture>,
  uniform: gl::types::GLint,
}

impl Sampler for SamplerOpenGL {
  fn any(&self) -> &dyn std::any::Any{
    self
  }

  fn set_name(&mut self, a_name: &str){
    self.name = String::from(a_name);
  }
}

pub struct ProgramOpenGL {
  id: gl::types::GLuint
}

impl Program for ProgramOpenGL {
  fn any(&self) -> &dyn std::any::Any{
    self
  }

  fn get_uniform(&self, a_name: &str, a_data: UniformData) -> Box<dyn Uniform>{
    let c_str = match CString::new(a_name){
      Ok(res) => res,
      Err(_res) => panic!("Invalid text cast")
    };

    let location = unsafe{gl::GetUniformLocation(self.id, c_str.as_ptr())};

    Box::new(UniformOpenGL{
      name: UniformName::new(a_name),
      data: a_data,
      id: location,
      modified: true
    })
  }
}

pub struct ShaderOpenGL {
  id: gl::types::GLuint,
  //source: String
}

impl Shader for ShaderOpenGL {
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

pub struct VerticesOpenGL {
  id: gl::types::GLuint,
  num: gl::types::GLsizei
}

impl Vertices for VerticesOpenGL {
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

pub struct GeometryOpenGL {
  vao: gl::types::GLuint,
  num: gl::types::GLsizei
}

impl Geometry for GeometryOpenGL {
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

#[allow(dead_code)]
pub struct TextureOpenGL {
  id: gl::types::GLuint,
  width: u32,
  height: u32
}

impl Texture for TextureOpenGL {
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

#[allow(dead_code)]
pub struct UniformOpenGL {
  name: UniformName,
  data: UniformData,
  id: gl::types::GLint,
  modified: bool,
}

#[allow(dead_code)]
impl UniformOpenGL {
  pub fn new<T: 'static + GetType>(a_name: &str, a_data: T, a_id: gl::types::GLint) -> UniformOpenGL{
    UniformOpenGL{
      name: UniformName::new(a_name), 
      data: UniformData::new::<T>(a_data),
      id: a_id,
      modified: true
    }
  }
}

#[allow(dead_code)]
impl Uniform for UniformOpenGL {
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

#[allow(dead_code)]
pub struct UniformShaderOpenGL {
  name: UniformName,
  id: gl::types::GLint
}

impl UniformShader for UniformShaderOpenGL {
  fn any(&mut self) -> &mut dyn std::any::Any{
    self
  }
}

pub struct RendererOpenGL {
  pub gl_context: sdl2::video::GLContext,
  pub version_major: i32,
  pub version_minor: i32,

  window: Arc<Window>,

  clear_color: Vec4,
  clear_depth: f32,
  clear_stencil: i32,

  viewport_pos: IVec2,
  viewport_size: IVec2,

  vao: gl::types::GLint,
  program_id: gl::types::GLint,
}

#[allow(dead_code)]
impl Renderer for RendererOpenGL {
  fn name(&self) -> String{
    String::from("OpenGL")
  }

  fn get_type(&self) -> RendererType{
    RendererType::OpenGL
  }

  fn begin_frame(&mut self, a_clear: RendererClearType){
    self.clear(a_clear);
  }

  fn end_frame(&mut self){
    self.window.window.lock().unwrap().inner.gl_swap_window();
  }

  //clear immediatly
  fn clear(&mut self, a_clear: RendererClearType){
    let mut bits: gl::types::GLenum = 0;

    if (a_clear & RendererClearType::COLOR) == RendererClearType::COLOR {
      bits |= gl::COLOR_BUFFER_BIT;
    }

    if (a_clear & RendererClearType::DEPTH) == RendererClearType::DEPTH {
      bits |= gl::DEPTH_BUFFER_BIT;
    }

    if (a_clear & RendererClearType::STENCIL) == RendererClearType::STENCIL {
      bits |= gl::STENCIL_BUFFER_BIT;
    }

    unsafe {
      gl::Clear(bits);
    }
  }

  // Get and set clear values may be called before BeginFrame
  fn set_clear_color(&mut self, a_color: Vec4){
    self.clear_color = a_color;
    unsafe {
      gl::ClearColor(a_color.x, a_color.y, a_color.z, a_color.w);
    }
  }

  fn set_clear_depth(&mut self, a_depth: f32){
    self.clear_depth = a_depth;
    unsafe {
      gl::ClearDepthf(a_depth);
    }
  }

  fn set_clear_stencil(&mut self, a_stencil: i32){
    self.clear_stencil = a_stencil;
    unsafe {
      gl::ClearStencil(a_stencil);
    }
  }

  fn get_clear_color(&self) -> Vec4{
    self.clear_color
  }

  fn get_clear_depth(&self) -> f32{
    self.clear_depth
  }

  fn get_clear_stencil(&self) -> i32{
    self.clear_stencil
  }

  fn set_viewport(&mut self, a_pos: IVec2, a_size: IVec2){
    self.viewport_pos = a_pos;
    self.viewport_size = a_size;
    unsafe {
      gl::Viewport(a_pos.x, a_pos.y, a_size.x, a_size.y);
    }
  }

  fn get_viewport_pos(&self) -> IVec2{
    self.viewport_pos
  }
  fn get_viewport_size(&self) -> IVec2{
    self.viewport_size
  }

  fn load_shader(&mut self, a_shader_type: ShaderType, a_source: &str) -> Result<Box<dyn Shader>, RendererError>{
    /*
    let id = match a_shader_type {
      Vertex => unsafe { gl::CreateShader(gl::VERTEX_SHADER) },
      TesselationControl => unsafe { gl::CreateShader(gl::TESS_CONTROL_SHADER) },
      TesselationEvaluation => unsafe { gl::CreateShader(gl::TESS_EVALUATION_SHADER) },
      Geometry => unsafe { gl::CreateShader(gl::GEOMETRY_SHADER) },
      Fragment => unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) },
      Compute => unsafe { gl::CreateShader(gl::COMPUTE_SHADER) }
    };
    */
    let shader_type = match a_shader_type {
      ShaderType::Vertex => gl::VERTEX_SHADER,
      ShaderType::TesselationControl => gl::TESS_CONTROL_SHADER,
      ShaderType::TesselationEvaluation => gl::TESS_EVALUATION_SHADER,
      ShaderType::Geometry => gl::GEOMETRY_SHADER,
      ShaderType::Fragment => gl::FRAGMENT_SHADER,
      ShaderType::Compute => gl::COMPUTE_SHADER
    };

    let id = unsafe {gl::CreateShader(shader_type)};

    let c_str = match CString::new(a_source){
      Ok(res) => res,
      Err(_res) => return Err(RendererError::ShaderCompile)
    };
    //let c_world: *const c_char = c_str.as_ptr() as *const c_char;

    unsafe {
      gl::ShaderSource(id, 1, &c_str.as_ptr(), std::ptr::null());
      gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
      gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {

      let mut len: gl::types::GLint = 0;
      unsafe {
          gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
      }

      // allocate buffer of correct size
      let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
      // fill it with len spaces
      buffer.extend([b' '].iter().cycle().take(len as usize));
      // convert buffer to CString
      let error: CString = unsafe { CString::from_vec_unchecked(buffer) };

      unsafe {
        gl::GetShaderInfoLog(
            id,
            len,
            std::ptr::null_mut(),
            error.as_ptr() as *mut gl::types::GLchar
        );
      }

      eprintln!("Error {}", error.to_string_lossy().into_owned());
      //return Err(error.to_string_lossy().into_owned());

      return Err(RendererError::Error)
    }

    Ok(Box::new(ShaderOpenGL{id:id}))
  }

  fn load_program_vert_frag(&mut self, a_shader_vert: Box<dyn Shader>, a_shader_frag: Box<dyn Shader>) -> Result<Box<dyn Program>, RendererError>{
    let program_id = unsafe { gl::CreateProgram() };

    let shader_vert = match a_shader_vert.any().downcast_ref::<ShaderOpenGL>() {
      Some(res) => res,
      None => return Err(RendererError::InvalidCast)
    };

    let shader_frag = match a_shader_frag.any().downcast_ref::<ShaderOpenGL>() {
      Some(res) => res,
      None => return Err(RendererError::InvalidCast)
    };

    unsafe {
      gl::AttachShader(program_id, shader_vert.id);
      gl::AttachShader(program_id, shader_frag.id);
      gl::LinkProgram(program_id);
      gl::DetachShader(program_id, shader_vert.id);
      gl::DetachShader(program_id, shader_frag.id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
    }

    if success == 0 {
      let mut len: gl::types::GLint = 0;
      unsafe {
        gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
      }

      // allocate buffer of correct size
      let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
      // fill it with len spaces
      buffer.extend([b' '].iter().cycle().take(len as usize));
      // convert buffer to CString
      let error: CString = unsafe { CString::from_vec_unchecked(buffer) };

      unsafe {
        gl::GetProgramInfoLog(
          program_id,
          len,
          std::ptr::null_mut(),
          error.as_ptr() as *mut gl::types::GLchar
        );
      }

      eprintln!("Error {}", error.to_string_lossy().into_owned());

      //return Err(error.to_string_lossy().into_owned());
      return Err(RendererError::Error)
    }

    Ok(Box::new(ProgramOpenGL{id: program_id}))
  }

  fn get_uniform(&mut self, a_shader: &mut Box<dyn Program>, a_name: &str) -> Box<dyn UniformShader>{
    let shader = match a_shader.any().downcast_ref::<ProgramOpenGL>() {
      Some(res) => res,
      None => panic!("Invalid shader cast")
    };

    let c_str = match CString::new(a_name){
      Ok(res) => res,
      Err(_res) => panic!("Invalid text cast")
    };

    let location = unsafe{gl::GetUniformLocation(shader.id, c_str.as_ptr())};

    Box::new(UniformShaderOpenGL{
      name: UniformName::new(a_name),
      id: location
    })
  }

  /*
  fn set_uniform(&mut self, a_uniform: &Box<dyn Uniform>){
    let uniform = match a_uniform.any().downcast_ref::<UniformOpenGL>() {
      Some(res) => res,
      None => panic!("Invalid uniform cast")
    };

    unsafe{
      gl::Uniform1i(uniform.id, 0);
    }
  }

  fn set_texture(&mut self, a_texture: &Box<dyn Texture>){
    let texture = match a_texture.any().downcast_ref::<TextureOpenGL>() {
      Some(res) => res,
      None => panic!("Invalid uniform cast")
    };

    unsafe{
      gl::ActiveTexture(gl::TEXTURE0 + 0);
      gl::BindTexture(gl::TEXTURE_2D,  texture.id);
    }
  }
  */

  fn gen_buffer_vertex(&mut self, a_verts: &std::vec::Vec<f32>) -> Box<dyn Vertices>{
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
      gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
      gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
      gl::BufferData(
          gl::ARRAY_BUFFER, // target
          (a_verts.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
          a_verts.as_ptr() as *const gl::types::GLvoid, // pointer to data
          gl::STATIC_DRAW, // usage
      );
      gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
    }

    Box::new(VerticesOpenGL{id: vbo, num: (a_verts.len()/4) as gl::types::GLsizei})
  }

  fn gen_geometry(&mut self, a_buffer: &Box<dyn Vertices>) -> Box<dyn Geometry>{
    let buffer = match a_buffer.any().downcast_ref::<VerticesOpenGL>() {
      Some(res) => res,
      None => panic!("Invalid vertex")
    };

    let mut vao: gl::types::GLuint = 0;
    unsafe {
      gl::GenVertexArrays(1, &mut vao);
      gl::BindVertexArray(vao);
      gl::BindBuffer(gl::ARRAY_BUFFER, buffer.id);

      gl::EnableVertexAttribArray(0); // todo this is "layout (location = 0)" in vertex shader
      gl::VertexAttribPointer(
        0, // index of the generic vertex attribute ("layout (location = 0)")
        2, // the number of components per generic vertex attribute
        gl::FLOAT, // data type
        gl::FALSE, // normalized (int-to-float conversion)
        (4 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
        std::ptr::null() // offset of the first component in bytes
      );
      
      gl::EnableVertexAttribArray(1); // todo this is "layout (location = 0)" in vertex shader
      gl::VertexAttribPointer(
        1, // index of the generic vertex attribute ("layout (location = 0)")
        2, // the number of components per generic vertex attribute
        gl::FLOAT, // data type
        gl::FALSE, // normalized (int-to-float conversion)
        (4 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
        (2 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component in bytes
      );
      

      gl::BindBuffer(gl::ARRAY_BUFFER, 0);
      gl::BindVertexArray(0);
    }
    Box::new(GeometryOpenGL{vao:vao, num: buffer.num})
  }

  fn gen_mesh(&mut self, a_geometry: Box<dyn Geometry>, a_material: Box<dyn Material>) -> Box<Mesh>{
    Box::new(Mesh{
      geometry: a_geometry,
      material: a_material
      })
  }


  fn gen_buffer_texture(&mut self) -> Box<dyn Texture>{
    let mut id: gl::types::GLuint = 0;
    unsafe {
      gl::GenTextures(1, &mut id);
    }

    Box::new(TextureOpenGL{
      id: id,
      width: 0,
      height: 0})
  }

  fn gen_sampler(&mut self, a_texture: Rc<dyn Texture>) -> Box<dyn Sampler>{
    let sampler = SamplerOpenGL{name: String::from(""), texture: a_texture, uniform: -1};

    Box::new(sampler)
  }

  fn load_texture(&mut self, a_image: &image::DynamicImage, a_texture: &mut Box<dyn Texture>){
    let texture = match a_texture.any().downcast_ref::<TextureOpenGL>() {
      Some(res) => res,
      None => panic!("Invalid texture")
    };

    // let mut bgra =  image::DynamicImage::ImageRgba8(a_image.to_rgba8());
    // self.to_bgra8(&mut bgra);

    let rgba =  image::DynamicImage::ImageRgba8(a_image.to_rgba8());
    // self.to_rgba8(&mut rgba);

    unsafe{
      gl::BindTexture(gl::TEXTURE_2D, texture.id);
      // gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, bgra.width() as i32, bgra.height() as i32, 0, 
      //   gl::BGRA, gl::UNSIGNED_BYTE, bgra.as_bytes().as_ptr() as *const std::os::raw::c_void);

      gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, rgba.width() as i32, rgba.height() as i32, 0, 
        gl::RGBA, gl::UNSIGNED_BYTE, rgba.as_bytes().as_ptr() as *const std::os::raw::c_void);

      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

      gl::GenerateMipmap(gl::TEXTURE_2D);
    }
  }

  fn use_program(&mut self, a_program: &Box<dyn Program>){
    let program = match a_program.any().downcast_ref::<ProgramOpenGL>() {
      Some(res) => res,
      None => return
    };

    if self.program_id != program.id as gl::types::GLint{
      self.program_id = program.id as gl::types::GLint;

      unsafe {
        gl::UseProgram(program.id);
      }
    }
  }

  fn draw_geometry(&mut self, a_geometry: &Box<dyn Geometry>){
    let geometry = match a_geometry.any().downcast_ref::<GeometryOpenGL>() {
      Some(res) => res,
      None => panic!("Invalid vertex")
    };

    if self.vao != geometry.vao as gl::types::GLint{
      self.vao = geometry.vao as gl::types::GLint;

      unsafe {
        gl::BindVertexArray(geometry.vao);
      }
    }

    unsafe {
      gl::DrawArrays(
        gl::TRIANGLES, // mode
        0, // starting index in the enabled arrays
        geometry.num // number of indices to be rendered
      );
    }
  }

  fn draw_mesh(&mut self, _camera: &Camera, a_mesh: &mut Box<Mesh>){
    let geometry = match a_mesh.geometry.any().downcast_ref::<GeometryOpenGL>() {
      Some(res) => res,
      None => panic!("Invalid vertex")
    };

    self.use_program(a_mesh.material.get_program());

    if self.vao != geometry.vao as gl::types::GLint{
      self.vao = geometry.vao as gl::types::GLint;

      unsafe {
        gl::BindVertexArray(geometry.vao);
      }
    }

    let num_uniforms = a_mesh.material.num_uniforms();
    for i in 0..num_uniforms {
      self.update_uniform(a_mesh.material.get_uniform(i));
    }

    let num_samplers = a_mesh.material.num_samplers();
    for i in 0..num_samplers {
      self.update_sampler(a_mesh.material.get_sampler(i));
    }

    unsafe {
      gl::DrawArrays(
        gl::TRIANGLES, // mode
        0, // starting index in the enabled arrays
        geometry.num // number of indices to be rendered
      );
    }
  }

  fn read_render_buffer(&mut self) -> Image {
    let mut image = Image{
      width: self.window.width, 
      height: self.window.height, 
      pitch: self.window.height * 4, 
      pixels: vec![0u8; (self.window.width * self.window.height  * 4) as usize]};

    unsafe { gl::ReadPixels( 
      0, 0, 
      self.window.width as i32, self.window.height as i32, 
      gl::RGBA,
      gl::UNSIGNED_BYTE,
      image.pixels.as_mut_ptr() as *mut _) };
    
    return image
  }

}

#[allow(dead_code)]
impl RendererOpenGL {
  const GL_MAX_VERSION_MINOR: [i32; 5] = [0, 5, 1, 3, 6];
  const GLES_MAX_VERSION_MINOR: [i32; 4] = [0, 1, 0, 2];

  pub fn new(
    a_video_subsystem: &sdl2::VideoSubsystem, 
    a_min_version: Version, 
    a_max_version: Version, 
    a_window: Arc<Window>, 
    a_is_gles: bool) -> Result<Self, RendererError>
  {
    let gl_context = match a_is_gles {
      true => match init_gles_context(&a_video_subsystem, a_min_version, a_max_version, &a_window.window.lock().unwrap().inner) {
        Ok(res) => res,
        Err(_res) => return Err(RendererError::Error)
      },
      false => match init_gl_context(&a_video_subsystem, a_min_version, a_max_version, &a_window.window.lock().unwrap().inner) {
        Ok(res) => res,
        Err(_res) => return Err(RendererError::Error)
      }
    };
    
    gl::load_with(|s| a_video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // // swap interval requires emscripten main loop to be set first
    // #[cfg(not(target_os = "emscripten"))]
    // match a_video_subsystem.gl_set_swap_interval(sdl2::video::SwapInterval::Immediate){
    //   Ok(_res) => _res,
    //   Err(_res) => print!("Unable to set vsync\n")
    // };

    Ok(Self {
      gl_context: gl_context,
      window: a_window,
      version_major: 0,
      version_minor: 0,
      clear_color: Vec4::new(0.0, 0.0, 0.0, 0.0),
      clear_depth: 1.0,
      clear_stencil: 0,
      viewport_pos: IVec2::new(0,0),
      viewport_size: IVec2::new(0,0),
      vao: -1,
      program_id: -1
    })
  }

  pub fn update_uniform(&self, a_uniform: &mut Box<dyn Uniform>){
    let mut uniform = match a_uniform.any().downcast_mut::<UniformOpenGL>() {
      Some(res) => res,
      None => panic!("Invalid uniform cast")
    };

    if uniform.modified {
      match uniform.data.info.element_type {
        ElementType::Float32 => {
          match uniform.data.info.container_type{
            ContainerType::Single => {
              let data = uniform.data.get::<f32>();
              unsafe{
                gl::Uniform1fv(uniform.id, 1, &data as *const f32);
              }
            },
            ContainerType::Vec2 => {
              let data = uniform.data.get::<Vec2>();
              unsafe{
                gl::Uniform2fv(uniform.id, 1, &data[0] as *const f32);
              }
            },
            ContainerType::Vec3 => {
              let data = uniform.data.get::<Vec3>();
              unsafe{
                gl::Uniform3fv(uniform.id, 1, &data[0] as *const f32);
              }
            },
            ContainerType::Vec4 => {
              let data = uniform.data.get::<Vec4>();
              unsafe{
                gl::Uniform4fv(uniform.id, 1, &data[0] as *const f32);
              }
            },
            
            ContainerType::Mat2x2 => {
              let data = uniform.data.get::<Mat2>();
              unsafe{
                gl::UniformMatrix2fv(uniform.id, 1, 0, &data.to_cols_array()[0] as *const f32);
              }
            },
            
            ContainerType::Mat3x3 => {
              let data = uniform.data.get::<Mat3>();
              unsafe{
                gl::UniformMatrix3fv(uniform.id, 1, 0, &data.to_cols_array()[0] as *const f32);
              }
            },
            ContainerType::Mat4x4 => {
              let data = uniform.data.get::<Mat4>();
              unsafe{
                gl::UniformMatrix4fv(uniform.id, 1, 0, &data.to_cols_array()[0] as *const f32);
              }
            }
            //_ => panic!("Invalid number of components")
          };
        },
        _ => panic!("Unimplemented type")
        
      };

      uniform.modified = false;
    }
  }

  pub fn update_sampler(&self, a_sampler: &Box<dyn Sampler>){
    let sampler = match a_sampler.any().downcast_ref::<SamplerOpenGL>() {
      Some(res) => res,
      None => panic!("Invalid sampler cast")
    };

    unsafe{
      gl::Uniform1i(sampler.uniform, 0);
    }
  
    let texture = match sampler.texture.any().downcast_ref::<TextureOpenGL>() {
      Some(res) => res,
      None => panic!("Invalid uniform cast")
    };

    unsafe{
      gl::ActiveTexture(gl::TEXTURE0 + 0);
      gl::BindTexture(gl::TEXTURE_2D,  texture.id);
    }
  }

  fn to_bgra8(&self, a_image: &mut image::DynamicImage){
    let mut pixels_it = match a_image.as_mut_rgba8(){
      Some(res) => res.pixels_mut(),
      None => panic!("invalid image")
    };

    for pix in &mut pixels_it{
      let val = pix.0[2];
      pix.0[2] = pix.0[0];
      pix.0[0] = val;
    }


  }
}

impl Drop for ShaderOpenGL {
  fn drop(&mut self) {
      unsafe {
          gl::DeleteShader(self.id);
      }
  }
}


impl Drop for ProgramOpenGL {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteProgram(self.id);
    }
  }
}

impl Drop for VerticesOpenGL {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteBuffers(1, &mut self.id);
    }
  }
}

impl Drop for TextureOpenGL {
  fn drop(&mut self) {
    unsafe {
      gl::DeleteTextures(1, &mut self.id);
    }
  }
}

fn get_gl_version_major(a_version: VersionNum) -> i32 {
  return match a_version {
    VersionNum::Highest => 4,
    VersionNum::Lowest => 1,
    VersionNum::Value(res) => res
  }
}

fn get_gl_version_minor( a_version_major: i32, a_version_minor: VersionNum) -> Result<i32, RendererError> {
  return match a_version_minor{
    VersionNum::Highest => {
      if a_version_major >= 1 && a_version_major < RendererOpenGL::GL_MAX_VERSION_MINOR.len() as i32{
        Ok(RendererOpenGL::GL_MAX_VERSION_MINOR[a_version_major as usize])
      }
      else {
        Err(RendererError::InvalidVersion)
      }
    },
    VersionNum::Lowest => Ok(0),
    VersionNum::Value(res) => Ok(res)
  };
}

fn get_gles_version_major(a_version: VersionNum) -> i32 {
  return match a_version {
    VersionNum::Highest => 3,
    VersionNum::Lowest => 1,
    VersionNum::Value(res) => res
  }
}

fn get_gles_version_minor( a_version_major: i32, a_version_minor: VersionNum) -> Result<i32, RendererError> {
  return match a_version_minor{
    VersionNum::Highest => {
      if a_version_major >= 1 && a_version_major < RendererOpenGL::GLES_MAX_VERSION_MINOR.len() as i32{
        Ok(RendererOpenGL::GLES_MAX_VERSION_MINOR[a_version_major as usize])
      }
      else {
        Err(RendererError::InvalidVersion)
      }
    },
    VersionNum::Lowest => Ok(0),
    VersionNum::Value(res) => Ok(res)
  };
}

fn init_gl_context(
  a_video_subsystem: &sdl2::VideoSubsystem, 
  a_min_version: Version, 
  a_max_version: Version,
  a_window: &sdl2::video::Window) -> Result<sdl2::video::GLContext, RendererError> 
{
  let mut version_major = get_gl_version_major(a_max_version.major);

  let mut version_minor = match get_gl_version_minor(version_major, a_max_version.minor){
    Ok(res) => res,
    Err(res) => return Err(res)
  };

  let min_version_major = get_gl_version_major(a_min_version.major);
  let min_version_minor = match get_gl_version_minor(min_version_major, a_min_version.minor){
    Ok(res) => res,
    Err(res) => return Err(res)
  };

  let gl_attr = a_video_subsystem.gl_attr();

  loop {
    if version_major > 2 {
      gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    }
    
    gl_attr.set_context_version(version_major as u8, version_minor as u8);

    let gl_context_result = a_window.gl_create_context();

    match gl_context_result {
      Ok(res) => {
        return Ok(res);
      },
      Err(_res) => {
        //try lower version of gl
        if version_minor > 0 {
          version_minor -= 1;
        }
        else if version_minor == 0 && version_major > 0{
          version_major -= 1;
          version_minor = RendererOpenGL::GL_MAX_VERSION_MINOR[version_major as usize];
        }

        //check if we go below min version
        if (version_major < min_version_major) || (version_major == min_version_major && version_minor < min_version_minor){
          return Err(RendererError::Error)
        }
      }
    }
  }
}

fn init_gles_context(
  a_video_subsystem: &sdl2::VideoSubsystem, 
  a_min_version: Version, 
  a_max_version: Version, 
  a_window: &sdl2::video::Window) -> Result<sdl2::video::GLContext, RendererError> 
{
  let mut version_major = get_gles_version_major(a_max_version.major);

  let mut version_minor = match get_gles_version_minor(version_major, a_max_version.minor){
    Ok(res) => res,
    Err(res) => return Err(res)
  };

  let min_version_major = get_gles_version_major(a_min_version.major);
  let min_version_minor = match get_gles_version_minor(min_version_major, a_min_version.minor){
    Ok(res) => res,
    Err(res) => return Err(res)
  };

  let gl_attr = a_video_subsystem.gl_attr();

  loop {
    if version_major > 2 {
      gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
    }
    
    gl_attr.set_context_version(version_major as u8, version_minor as u8);

    let gl_context_result = a_window.gl_create_context();

    match gl_context_result {
      Ok(res) => {
        return Ok(res);
      },
      Err(_res) => {
        //try lower version of gl
        if version_minor > 0 {
          version_minor -= 1;
        }
        else if version_minor == 0 && version_major > 0{
          version_major -= 1;
          version_minor = RendererOpenGL::GL_MAX_VERSION_MINOR[version_major as usize];
        }

        //check if we go below min version
        if (version_major < min_version_major) || (version_major == min_version_major && version_minor < min_version_minor){
          return Err(RendererError::Error)
        }
      }
    }
  }
}
