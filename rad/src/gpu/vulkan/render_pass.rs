
use super::device::LogicalDevice;
use crate::gpu::renderer_types::RendererError;

pub struct RenderPass{
  pub render_pass: ash::vk::RenderPass,
  pub logical_device: std::rc::Rc<LogicalDevice>
}

impl RenderPass{
  pub fn new(
    a_logical_device: std::rc::Rc<LogicalDevice>, a_format: ash::vk::Format) -> Result<Self, RendererError>
  {
    let attachment_desc_color = ash::vk::AttachmentDescription::builder()
      .format(a_format)
      .samples(ash::vk::SampleCountFlags::TYPE_1)
      .load_op(ash::vk::AttachmentLoadOp::CLEAR)  //also create renderpass that does not clear, or create/fetch system to generate render pass as needed
      .store_op(ash::vk::AttachmentStoreOp::STORE)
      .stencil_load_op(ash::vk::AttachmentLoadOp::DONT_CARE)
      .stencil_store_op(ash::vk::AttachmentStoreOp::DONT_CARE)
      .initial_layout(ash::vk::ImageLayout::UNDEFINED)
      .final_layout(ash::vk::ImageLayout::PRESENT_SRC_KHR)
      .build();

    let attachment_descs = [attachment_desc_color];

    let attachment_ref_color = ash::vk::AttachmentReference::builder()
      .attachment(0)
      .layout(ash::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
      .build();

    let attachment_refs = [attachment_ref_color];
  
    let subpass_descr = ash::vk::SubpassDescription::builder()
      .pipeline_bind_point(ash::vk::PipelineBindPoint::GRAPHICS)
      .color_attachments(&attachment_refs)
      .build();

    let subpasses = [subpass_descr];

    let subpass_dep = ash::vk::SubpassDependency::builder()
      .src_subpass(ash::vk::SUBPASS_EXTERNAL)
      .dst_subpass(0)
      .src_stage_mask(ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
      .src_access_mask(ash::vk::AccessFlags::NONE)
      .dst_stage_mask(ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
      .dst_access_mask(ash::vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
      .dependency_flags(ash::vk::DependencyFlags::BY_REGION)
      .build();

    let dependencies = [subpass_dep];
    
    let info = ash::vk::RenderPassCreateInfo::builder()
      .attachments(&attachment_descs)
      .subpasses(&subpasses)
      .dependencies(&dependencies)
      .build();
    let render_pass = unsafe { match a_logical_device.device.create_render_pass(&info, None){
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    }};

    Ok(RenderPass{render_pass: render_pass, logical_device: a_logical_device.clone()})
  }
}

impl Drop for RenderPass
{
  fn drop(&mut self){
    unsafe{ self.logical_device.device.destroy_render_pass(self.render_pass, None)};
  }
}
