
extern crate gl;

use std::fmt;

#[derive(Copy, Clone)]
pub enum RendererType {
  OpenGL,
  OpenGLES,
  DirectX,
  Vulkan,
  Metal
}


#[derive(Debug, Clone)]
pub enum RendererError {
  Error
}

#[derive(Copy, Clone)]
pub enum RendererClearType{
  RendererClearColor = 0x1,
  RendererClearDepth = 0x2,
  RendererClearStencil = 0x4
}

impl std::error::Error for RendererError {}

impl fmt::Display for RendererError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      RendererError::Error => write!(f, "Error"),
    }
  }
}




pub trait Renderer {
  fn name(&self) -> String;

  //Frame to begin rendering. Render calls may now be made. Set weather to clear screen at render start
  //Reason on clearing here is vulkan rendering system has faster clear on start render
  //RendererClearColor | RendererClearDepth | RendererClearStencil
  fn begin_frame(&mut self, a_clear: RendererClearType);
  fn end_frame(&mut self);

  //clear immediatly
  //= RendererClearColor | RendererClearDepth | RendererClearStencil
  fn clear(&mut self, a_clear: RendererClearType);

  //Get and set clear values may be called before BeginFrame
  fn set_clear_color(&mut self, a_r: f32, a_g:  f32, a_b: f32, a_a: f32);
  fn set_clear_depth(&mut self, a_depth: f32);
  fn set_clear_stencil(&mut self, a_stencil: u8);
  fn get_clear_color(&self) -> (f32, f32, f32, f32);
  fn get_clear_depth(&self) -> f32;
  fn get_clear_stencil(&self) -> u8;
  
}

pub struct RendererOpenGL {
  pub gl_context: sdl2::video::GLContext,
  pub version_major: i32
}

pub struct RendererVulkan {
  pub version_major: i32
}


impl Renderer for RendererOpenGL {
  fn name(&self) -> String{
    String::from("OpenGL")
  }

  fn begin_frame(&mut self, a_clear: RendererClearType){}
  fn end_frame(&mut self){}

  //clear immediatly
  //= RendererClearColor | RendererClearDepth | RendererClearStencil
  fn clear(&mut self, a_clear: RendererClearType){}

  //Get and set clear values may be called before BeginFrame
  fn set_clear_color(&mut self, a_r: f32, a_g:  f32, a_b: f32, a_a: f32){}
  fn set_clear_depth(&mut self, a_depth: f32){}
  fn set_clear_stencil(&mut self, a_stencil: u8){}
  fn get_clear_color(&self) -> (f32, f32, f32, f32){
    (0.0,0.0,0.0,0.0)
  }
  fn get_clear_depth(&self) -> f32{
    0.0
  }

  fn get_clear_stencil(&self) -> u8{
    0
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
      version_major: 0
    })
  }
  
}

impl Renderer for RendererVulkan {
  fn name(&self) -> String{
    String::from("Vulkan")
  }

  fn begin_frame(&mut self, a_clear: RendererClearType){}
  fn end_frame(&mut self){}

  //clear immediatly
  //= RendererClearColor | RendererClearDepth | RendererClearStencil
  fn clear(&mut self, a_clear: RendererClearType){}

  //Get and set clear values may be called before BeginFrame
  fn set_clear_color(&mut self, a_r: f32, a_g:  f32, a_b: f32, a_a: f32){}
  fn set_clear_depth(&mut self, a_depth: f32){}
  fn set_clear_stencil(&mut self, a_stencil: u8){}
  fn get_clear_color(&self) -> (f32, f32, f32, f32){
    (0.0,0.0,0.0,0.0)
  }
  fn get_clear_depth(&self) -> f32{
    0.0
  }

  fn get_clear_stencil(&self) -> u8{
    0
  }
}

impl RendererVulkan{
  pub fn new() -> Result<Self, RendererError>{
    Ok(Self {
      version_major: 0
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
