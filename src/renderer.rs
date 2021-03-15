

extern crate gl;
extern crate glam;

use std::fmt;
use std::ffi::{CString, CStr};
use glam::{Vec4, IVec2};

#[derive(Copy, Clone)]
pub enum RendererType {
  OpenGL,
  OpenGLES,
  DirectX,
  Vulkan,
  Metal
}

#[derive(Copy, Clone)]
pub enum ShaderType{
  Vertex,
  TesselationControl,
  TesselationEvaluation,
  Geometry,
  Fragment,
  Compute
}


#[derive(Debug, Clone)]
pub enum RendererError {
  Error,
  ShaderCompile,
  Unimplemented
}
/*
#[derive(Copy, Clone)]
pub enum RendererClearType{
  RendererClearColor = 0x1,
  RendererClearDepth = 0x2,
  RendererClearStencil = 0x4
}
*/
bitflags! {
  pub struct RendererClearType: u32 {
    const None = 0b00000000;
    const Color = 0b00000001;
    const Depth = 0b00000010;
    const Stencil = 0b00000100;
    //const ABC = Self::A.bits | Self::B.bits | Self::C.bits;
  }
}

impl std::error::Error for RendererError {}

impl fmt::Display for RendererError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      RendererError::Error => write!(f, "Error"),
      RendererError::Unimplemented => write!(f, "Error Unimplemented"),
      RendererError::ShaderCompile => write!(f, "Error ShaderCompile"),
    }
  }
}


pub trait Program{

}

pub struct ProgramOpenGL {

}

impl Program for ProgramOpenGL {

}

pub trait Shader{

}

pub struct ShaderOpenGL {
  id: gl::types::GLuint,
  source: String
}

impl Shader for ShaderOpenGL {

}


pub trait Texture{

}


pub trait Vertices{

}


pub struct Image{

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

  //color bgra
  fn set_clear_color(&mut self, a_color: Vec4);
  fn set_clear_depth(&mut self, a_depth: f32);
  fn set_clear_stencil(&mut self, a_stencil: i32);

  //color bgra
  fn get_clear_color(&self) -> Vec4;
  fn get_clear_depth(&self) -> f32;
  fn get_clear_stencil(&self) -> i32;

  fn set_viewport(&mut self, a_pos: IVec2, a_size: IVec2);
  fn get_viewport_pos(&self) -> IVec2;
  fn get_viewport_size(&self) -> IVec2;
  
  fn load_shader(&mut self, a_shader_type: ShaderType, a_source: &str) -> Result<Box<dyn Shader>, RendererError>;

  /*
  fn load_program_vert_frag(&mut self, a_shader_vert: Box<dyn Shader>, a_shader_frag: Box<dyn Shader>) -> Result<Box<dyn Program>, RendererError>;
  fn load_program_compute(&mut self, a_shader: Box<dyn Shader>) -> Result<Box<dyn Program>, RendererError>;

  fn load_texture(&mut self, a_image: Image) -> Box<dyn Texture>;

  fn load_vertex(&mut self, a_vertices: f32) -> Box<dyn Vertices>;
  */
}

pub struct RendererOpenGL {
  pub gl_context: sdl2::video::GLContext,
  pub version_major: i32,

  clear_color: Vec4,
  clear_depth: f32,
  clear_stencil: i32,

  viewport_pos: IVec2,
  viewport_size: IVec2,
}

pub struct RendererVulkan {
  pub version_major: i32,

  clear_color: Vec4,
  clear_depth: f32,
  clear_stencil: i32
}


impl Renderer for RendererOpenGL {
  fn name(&self) -> String{
    String::from("OpenGL")
  }

  fn get_type(&self) -> RendererType{
    RendererType::OpenGL
  }

  fn begin_frame(&mut self, a_clear: RendererClearType){}
  fn end_frame(&mut self){}

  //clear immediatly
  fn clear(&mut self, a_clear: RendererClearType){
    let mut bits: gl::types::GLenum = 0;

    if (a_clear & RendererClearType::Color) == RendererClearType::Color {
      bits |= gl::COLOR_BUFFER_BIT;
    }

    if (a_clear & RendererClearType::Depth) == RendererClearType::Depth {
      bits |= gl::DEPTH_BUFFER_BIT;
    }

    if (a_clear & RendererClearType::Stencil) == RendererClearType::Stencil {
      bits |= gl::STENCIL_BUFFER_BIT;
    }

    unsafe {
      gl::Clear(bits);
    }
  }

  //Get and set clear values may be called before BeginFrame
  fn set_clear_color(&mut self, a_color: Vec4){
    self.clear_color = a_color;
    unsafe {
      gl::ClearColor(a_color.z, a_color.y, a_color.x, a_color.w);
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
    let id = match a_shader_type {
      Vertex => unsafe { gl::CreateShader(gl::VERTEX_SHADER) },
      TesselationControl => unsafe { gl::CreateShader(gl::TESS_CONTROL_SHADER) },
      TesselationEvaluation => unsafe { gl::CreateShader(gl::TESS_EVALUATION_SHADER) },
      Geometry => unsafe { gl::CreateShader(gl::GEOMETRY_SHADER) },
      Fragment => unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) },
      Compute => unsafe { gl::CreateShader(gl::COMPUTE_SHADER) }
    };

    let c_str = match CString::new(a_source){
      Ok(res) => res,
      Err(res) => return Err(RendererError::ShaderCompile)
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

      //return Err(error.to_string_lossy().into_owned());

      return Err(RendererError::Error)
    }

    Err(RendererError::Error)
  }

}

impl RendererOpenGL {
  pub fn new(a_video_subsystem: &sdl2::VideoSubsystem, a_window: &sdl2::video::Window) -> Result<Self, RendererError>{
    let gl_context = match init_gl_context(&a_video_subsystem, &a_window) {
      Ok(res) => res,
      Err(res) => return Err(RendererError::Error)
    };

    let gl = gl::load_with(|s| a_video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    Ok(Self {
      gl_context: gl_context,
      version_major: 0,
      clear_color: Vec4::new(0.0, 0.0, 0.0, 0.0),
      clear_depth: 1.0,
      clear_stencil: 0,
      viewport_pos: IVec2::new(0,0),
      viewport_size: IVec2::new(0,0),
    })
  }
}

impl Drop for ShaderOpenGL {
  fn drop(&mut self) {
      unsafe {
          gl::DeleteShader(self.id);
      }
  }
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


fn init_gl_context(a_video_subsystem: &sdl2::VideoSubsystem, a_window: &sdl2::video::Window) -> Result<sdl2::video::GLContext, RendererError> {
  //let mut attempt = true;
  let mut gl_version_major = 4;
  let mut gl_version_minor = 6;

  let gl_attr = a_video_subsystem.gl_attr();

  loop {
    if gl_version_major > 2 {
      gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    }
    
    gl_attr.set_context_version(gl_version_major, gl_version_minor);

    let gl_context_result = a_window.gl_create_context();

    match gl_context_result {
      Ok(res) => {
        return Ok(res);
      },
      Err(res) => {
        //try lower version of gl
        if gl_version_minor > 0 {
          gl_version_minor -= 1;
        }
        else if gl_version_major == 4 && gl_version_minor == 0 {
          gl_version_major = 3;
          gl_version_minor = 3;
        }
        else if gl_version_major == 3 && gl_version_minor == 0 {
          gl_version_major = 2;
          gl_version_minor = 1;
        }
        else if gl_version_major == 2 && gl_version_minor == 0 {
          gl_version_major = 1;
          gl_version_minor = 5;
        }
        else if gl_version_major == 1 && gl_version_minor == 0 {
          return Err(RendererError::Error)
        }
      }
    }
  }
}
