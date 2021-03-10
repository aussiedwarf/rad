
extern crate gl;

#[derive(Copy, Clone)]
pub enum GpuType {
  OpenGL,
  OpenGLES,
  DirectX,
  Vulkan,
  Metal
}



pub trait Gpu {
  fn name(&self) -> String;
  //fn new() -> Self;
}

pub struct GpuOpenGL {
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
  pub fn new() -> Self{
    Self {
      versionMajor: 0
    }
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
  pub fn new() -> Self{
    Self {
      versionMajor: 0
    }
  }
}
