use super::device::LogicalDevice;
use crate::gpu::renderer_types::RendererError;

pub struct Fence{
  pub fence: ash::vk::Fence,
  pub logical_device: std::rc::Rc<LogicalDevice>, 
}

impl Fence{
  pub fn new(a_logical_device: std::rc::Rc<LogicalDevice>) -> Result<Self, RendererError>{
    let fence_info = ash::vk::FenceCreateInfo::builder()
      .build();

    let fence = match unsafe{a_logical_device.device.create_fence(&fence_info, None)}{
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    };

    return Ok(Fence{fence: fence, logical_device: a_logical_device})
  }
}

impl Drop for Fence{
  fn drop(&mut self){
    unsafe { self.logical_device.device.destroy_fence(self.fence, None) };
  }
}
