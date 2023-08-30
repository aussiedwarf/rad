use crate::gpu::renderer_types::RendererError;

use super::vulkan_surface::VulkanSurface;
use super::vulkan_instance::VulkanInstance;
use super::vulkan_device::{VulkanPhysicalDevice, VulkanLogicalDevice};
use super::image_view::ImageView;

pub struct Swapchain{
  pub image_views: Vec<ImageView>,
  pub swapchain: ash::vk::SwapchainKHR,
  pub swapchain_loader: ash::extensions::khr::Swapchain,
  pub swapchain_images: Vec<ash::vk::Image>,
  pub logical_device: std::rc::Rc<VulkanLogicalDevice>,
  pub extent: ash::vk::Extent2D
}

impl Swapchain{
  pub fn new(
    a_logical_device: std::rc::Rc<VulkanLogicalDevice>,
    a_instance: &VulkanInstance, 
    a_surface: &VulkanSurface, 
    a_physical_device: &VulkanPhysicalDevice, 
    a_extent: ash::vk::Extent2D) -> Result<Self, RendererError>{
    
    let surface_formats = unsafe { match a_surface.surface.get_physical_device_surface_formats(
      a_physical_device.device, a_surface.surface_khr){
        Ok(res) => res,
        Err(_res) => return Err(RendererError::Error)
      } };

    let format = match Swapchain::pick_format(&surface_formats){
      Some(res) => res,
      None => return Err(RendererError::Error)
    };

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

    let create_info = ash::vk::SwapchainCreateInfoKHR::builder()
      .surface(a_surface.surface_khr)
      .min_image_count(min_image_count)
      .image_format(format.format)
      .image_color_space(format.color_space)
      .image_extent(image_extent)
      .image_array_layers(1)
      .image_usage(ash::vk::ImageUsageFlags::COLOR_ATTACHMENT)
      .image_sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
      .pre_transform(surface_capabilities.current_transform)
      .composite_alpha(ash::vk::CompositeAlphaFlagsKHR::OPAQUE)
      .present_mode(present_mode)
      .clipped(true)
      .build();

    let swapchain_loader = ash::extensions::khr::Swapchain::new(&a_instance.instance, &a_logical_device.device);

    let swapchain = unsafe { match swapchain_loader.create_swapchain(&create_info, None) {
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    }};

    let swapchain_images = unsafe { match swapchain_loader.get_swapchain_images(swapchain) {
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    }};

    let mut image_views = Vec::<ImageView>::new();
    for &image in swapchain_images.iter(){
      image_views.push(match ImageView::new(a_logical_device.clone(), &image, format.format){
        Ok(res) => res,
        Err(_res) => return Err(RendererError::Error)
      });
    }

    Ok(Swapchain{
      image_views: image_views,
      swapchain: swapchain, 
      swapchain_loader: swapchain_loader, 
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

  fn pick_format(a_formats: &Vec<ash::vk::SurfaceFormatKHR>) -> Option<&ash::vk::SurfaceFormatKHR>{
    for format in a_formats{
      if format.format == ash::vk::Format::B8G8R8A8_SRGB && format.color_space == ash::vk::ColorSpaceKHR::SRGB_NONLINEAR{
        return Some(format)
      }
    }
    None
  }

  fn pick_present_mode(a_modes: &Vec<ash::vk::PresentModeKHR>) -> ash::vk::PresentModeKHR{
    for &mode in a_modes.iter(){
      if mode == ash::vk::PresentModeKHR::MAILBOX{
        return mode
      }
    }
    ash::vk::PresentModeKHR::FIFO
  }
}

impl Drop for Swapchain{
  fn drop(&mut self){
    self.swapchain_images.clear();

    unsafe { self.swapchain_loader.destroy_swapchain(self.swapchain, None) };
  }
}
