
extern crate gl;

use std::fmt;

#[derive(Copy, Clone)]
pub enum GpuType {
  OpenGL,
  OpenGLES,
  DirectX,
  Vulkan,
  Metal
}


#[derive(Debug, Clone)]
pub enum GpuError {
  Error
}

impl std::error::Error for GpuError {}

impl fmt::Display for GpuError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      GpuError::Error => write!(f, "Error"),
    }
  }
}


pub trait Gpu {
  fn name(&self) -> String;
  //fn new() -> Self;
}

pub struct GpuOpenGL {
  pub gl_context: sdl2::video::GLContext,
  pub versionMajor: i32
}

pub struct GpuVulkan {
  pub versionMajor: i32
}


impl Gpu for GpuOpenGL {
  fn name(&self) -> String{
    String::from("OpenGL")
  }

}

impl GpuOpenGL {
  pub fn new(a_video_subsystem: &sdl2::VideoSubsystem, a_window: &sdl2::video::Window) -> Result<Self, GpuError>{
    let gl_context = match init_gl_context(&a_video_subsystem, &a_window) {
      Ok(res) => res,
      Err(res) => return Err(GpuError::Error)
    };

    let gl = gl::load_with(|s| a_video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    Ok(Self {
      gl_context: gl_context,
      versionMajor: 0
    })
  }

  fn my_method(&self) -> i32{
    85
  }

  
}

impl Gpu for GpuVulkan {
  fn name(&self) -> String{
    String::from("Vulkan")
  }
}

impl GpuVulkan{
  pub fn new() -> Result<Self, GpuError>{
    Ok(Self {
      versionMajor: 0
    })
  }
}


fn init_gl_context(a_video_subsystem: &sdl2::VideoSubsystem, a_window: &sdl2::video::Window) -> Result<sdl2::video::GLContext, GpuError> {
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
          return Err(GpuError::Error)
        }
      }
    }
  }
}
