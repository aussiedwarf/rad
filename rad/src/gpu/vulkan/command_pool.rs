use super::device::LogicalDevice;
use crate::gpu::renderer_types::RendererError;

pub struct CommandPool{
  pub pool: ash::vk::CommandPool,
  pub logical_device: std::rc::Rc<LogicalDevice>
}

impl CommandPool{
  pub fn new(a_logical_device: std::rc::Rc<LogicalDevice>, a_queue_family_index: u32) -> Result<Self, RendererError>{
    let create_info = ash::vk::CommandPoolCreateInfo::builder()
      .flags(ash::vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
      .queue_family_index(a_queue_family_index)
      .build();
    let pool = unsafe {match a_logical_device.device.create_command_pool(&create_info, None){
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    }};
    Ok(Self{pool: pool, logical_device: a_logical_device})
  }

  // TODO WARNING if command pool is destroyed, the buffers are destroyed as well
  pub fn allocate_command_buffer(&self, a_command_buffer_count: u32) -> Result<std::vec::Vec::<ash::vk::CommandBuffer>, RendererError>{
    let allocate_info = ash::vk::CommandBufferAllocateInfo::builder()
      .command_pool(self.pool)
      .level(ash::vk::CommandBufferLevel::PRIMARY)
      .command_buffer_count(a_command_buffer_count)
      .build();
    let command_buffers = unsafe { match self.logical_device.device.allocate_command_buffers(&allocate_info) {
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    }};

    Ok(command_buffers)
  }
}

impl Drop for CommandPool{
  fn drop(&mut self){
    unsafe{self.logical_device.device.destroy_command_pool(self.pool, None)};
  }
}
