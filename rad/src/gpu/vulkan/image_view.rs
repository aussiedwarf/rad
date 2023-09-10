use crate::gpu::renderer_types::RendererError;
use crate::gpu::vulkan::device::LogicalDevice;

pub struct ImageView{
  pub view: ash::vk::ImageView,
  pub logical_device: std::rc::Rc<LogicalDevice>
}

impl ImageView{
  pub fn new(a_logical_device: std::rc::Rc<LogicalDevice>, a_image: &ash::vk::Image, a_format: ash::vk::Format) -> Result<Self, RendererError>{

    let create_info = ash::vk::ImageViewCreateInfo::builder()
      .image(*a_image)
      .view_type(ash::vk::ImageViewType::TYPE_2D)
      .format(a_format)
      .subresource_range(ash::vk::ImageSubresourceRange{
        aspect_mask: ash::vk::ImageAspectFlags::COLOR,
        base_mip_level: 0,
        level_count: 1,
        base_array_layer: 0,
        layer_count: 1
      })
      .build();

    let view = unsafe {match a_logical_device.device.create_image_view(&create_info, None){
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    }};

    Ok(ImageView{view: view, logical_device: a_logical_device})
  }
}

impl Drop for ImageView{
  fn drop(&mut self){
    unsafe {
      if self.view != ash::vk::ImageView::null(){
        self.logical_device.device.destroy_image_view(self.view, None);
        self.view = ash::vk::ImageView::null();
      }
    }
  }
}
