use super::device::LogicalDevice;
use super::image_view::ImageView;
use super::render_pass::RenderPass;
use crate::gpu::renderer_types::RendererError;

pub struct Framebuffer{
  pub framebuffer: ash::vk::Framebuffer,
  pub logical_device: std::rc::Rc<LogicalDevice>
}

impl Framebuffer{
  pub fn new(
    a_logical_device: std::rc::Rc<LogicalDevice>, 
    a_render_pass: &RenderPass,
    a_image_view: &ImageView,
    a_extent: ash::vk::Extent2D) -> Result<Self, RendererError> 
  {
    let attachments = [a_image_view.view];

    let info= ash::vk::FramebufferCreateInfo::builder()
      .render_pass(a_render_pass.render_pass)
      .attachments(&attachments)
      .width(a_extent.width)
      .height(a_extent.height)
      .layers(1)
      .build();

    let framebuffer = unsafe { match a_logical_device.device.create_framebuffer(&info, None) {
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    }};
  
    Ok(Framebuffer{framebuffer: framebuffer, logical_device: a_logical_device.clone()})
  }
}

impl Drop for Framebuffer{
  fn drop(&mut self){
    unsafe{self.logical_device.device.destroy_framebuffer(self.framebuffer, None)};
  }
}
