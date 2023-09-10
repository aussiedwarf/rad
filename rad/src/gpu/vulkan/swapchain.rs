use crate::gpu::renderer_types::RendererError;

use super::device::{PhysicalDevice, LogicalDevice};
use super::framebuffer::Framebuffer;
use super::instance::Instance;
use super::image_view::ImageView;
use super::render_pass::RenderPass;
use super::surface::Surface;

pub struct SwapchainBase{
  pub swapchain: ash::vk::SwapchainKHR,
  pub swapchain_loader: ash::extensions::khr::Swapchain,
}

pub struct Swapchain{
  pub logical_device: std::rc::Rc<LogicalDevice>,
  pub extent: ash::vk::Extent2D,

  // order matters
  pub framebuffers: Vec<Framebuffer>,
  pub image_views: Vec<ImageView>,
  pub swapchain_images: Vec<ash::vk::Image>,
  pub swapchain: SwapchainBase,
}

impl Swapchain{
  pub fn new(
    a_logical_device: std::rc::Rc<LogicalDevice>,
    a_instance: &Instance, 
    a_surface: &Surface, 
    a_physical_device: &PhysicalDevice, 
    a_render_pass: &RenderPass,
    a_format: &ash::vk::SurfaceFormatKHR,
    a_extent: ash::vk::Extent2D) -> Result<Self, RendererError>{

    let present_modes: Vec<ash::vk::PresentModeKHR> = unsafe { match a_surface.surface.get_physical_device_surface_present_modes(
      a_physical_device.device, a_surface.surface_khr){
        Ok(res) => res,
        Err(_res) => return Err(RendererError::Error)
      }};

    let present_mode = Swapchain::pick_present_mode(&present_modes);

    let surface_capabilities = unsafe { match a_surface.surface.get_physical_device_surface_capabilities(
      a_physical_device.device, a_surface.surface_khr){
        Ok(res) => res,
        Err(_res) => return Err(RendererError::Error)
      } };

    let mut min_image_count = surface_capabilities.min_image_count + 1;
    if (surface_capabilities.max_image_count > 0 && min_image_count > surface_capabilities.max_image_count)
		{
			min_image_count = surface_capabilities.max_image_count;
		}
    let image_extent = Swapchain::get_swap_extent(&surface_capabilities, a_extent);

    let swapchain_loader = ash::extensions::khr::Swapchain::new(&a_instance.instance, &a_logical_device.device);

    if image_extent.width == 0 || image_extent.height == 0{
      return Ok(Swapchain{
        framebuffers: std::vec::Vec::<Framebuffer>::new(),
        image_views: std::vec::Vec::<ImageView>::new(),
        swapchain: SwapchainBase{swapchain: ash::vk::SwapchainKHR::null(), swapchain_loader: swapchain_loader}, 
        swapchain_images: std::vec::Vec::<ash::vk::Image>::new(), 
        logical_device: a_logical_device,
        extent: image_extent})
    }

    let create_info = ash::vk::SwapchainCreateInfoKHR::builder()
      .surface(a_surface.surface_khr)
      .min_image_count(min_image_count)
      .image_format(a_format.format)
      .image_color_space(a_format.color_space)
      .image_extent(image_extent)
      .image_array_layers(1)
      .image_usage(ash::vk::ImageUsageFlags::COLOR_ATTACHMENT)
      .image_sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
      .pre_transform(surface_capabilities.current_transform)
      .composite_alpha(ash::vk::CompositeAlphaFlagsKHR::OPAQUE)
      .present_mode(present_mode)
      .clipped(true)
      .build();

    let swapchain = unsafe { match swapchain_loader.create_swapchain(&create_info, None) {
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    }};

    let swapchain_base = SwapchainBase{swapchain: swapchain, swapchain_loader: swapchain_loader};

    let swapchain_images = unsafe { match swapchain_base.swapchain_loader.get_swapchain_images(swapchain) {
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    }};

    let mut image_views = Vec::<ImageView>::new();
    for &image in swapchain_images.iter(){
      image_views.push(match ImageView::new(a_logical_device.clone(), &image, a_format.format){
        Ok(res) => res,
        Err(_res) => return Err(RendererError::Error)
      });
    }

    let mut framebuffers = std::vec::Vec::<Framebuffer>::new();

    for image_view in &image_views{
      let framebuffer = match Framebuffer::new(a_logical_device.clone(), &a_render_pass, &image_view, image_extent){
        Ok(res) => res,
        Err(_res) => return Err(RendererError::Error)
      };
      framebuffers.push(framebuffer);
    }

    Ok(Swapchain{
      framebuffers: framebuffers,
      image_views: image_views,
      swapchain: swapchain_base, 
      swapchain_images: swapchain_images, 
      logical_device: a_logical_device,
      extent: image_extent})
  }

  fn get_swap_extent(a_capabilities: &ash::vk::SurfaceCapabilitiesKHR, a_extents: ash::vk::Extent2D) -> ash::vk::Extent2D {
    if a_capabilities.current_extent.width != u32::max_value() {
      a_capabilities.current_extent
    } 
    else {
      ash::vk::Extent2D {
        width: Ord::clamp(
          a_extents.width,
          a_capabilities.min_image_extent.width,
          a_capabilities.max_image_extent.width,
        ),
        height: Ord::clamp(
          a_extents.height,
          a_capabilities.min_image_extent.height,
          a_capabilities.max_image_extent.height,
        ),
      }
    }
  }

  fn pick_present_mode(a_modes: &Vec<ash::vk::PresentModeKHR>) -> ash::vk::PresentModeKHR{
    for &mode in a_modes.iter(){
      if mode == ash::vk::PresentModeKHR::MAILBOX{
        return mode
      }
    }
    ash::vk::PresentModeKHR::FIFO
  }

  pub fn clear(&mut self){
    self.swapchain_images.clear();

    for framebuffer in self.framebuffers.iter_mut(){
      unsafe{self.logical_device.device.destroy_framebuffer(framebuffer.framebuffer, None)};
      framebuffer.framebuffer = ash::vk::Framebuffer::null();
    }

    for image_view in self.image_views.iter_mut(){
      unsafe { self.logical_device.device.destroy_image_view(image_view.view, None) };
      image_view.view = ash::vk::ImageView::null();
    }

    unsafe { self.swapchain.swapchain_loader.destroy_swapchain(self.swapchain.swapchain, None) };
    self.swapchain.swapchain = ash::vk::SwapchainKHR::null();

    self.extent.width = 0;
    self.extent.height = 0;
  }
}

impl Drop for Swapchain{
  fn drop(&mut self){
    self.swapchain_images.clear();
  }
}

impl Drop for SwapchainBase{
  fn drop(&mut self){
    unsafe { 
      if self.swapchain != ash::vk::SwapchainKHR::null(){
        self.swapchain_loader.destroy_swapchain(self.swapchain, None);
        self.swapchain = ash::vk::SwapchainKHR::null();
      }
    };
  }
}
