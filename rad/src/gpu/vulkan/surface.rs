use super::instance::Instance;
use super::device::{PhysicalDevice};
use crate::gpu::renderer_types::*;
use ash::{Entry};
use ash::vk::Handle;

pub struct Surface{
  pub surface: ash::extensions::khr::Surface,
  pub surface_khr: ash::vk::SurfaceKHR
}

impl Surface{
  pub fn new(a_window: &sdl2::video::Window, a_entry: &Entry, a_instance: &Instance) -> Result<Self, RendererError>{
    let surface_raw = match a_window.vulkan_create_surface(a_instance.instance.handle().as_raw() as usize){
      Ok(res) => res,
      Err(res) => {
        println!("{}", res);
        return Err(RendererError::Error)
      }
    };

    let surface_khr = ash::vk::SurfaceKHR::from_raw(surface_raw);

    let surface = ash::extensions::khr::Surface::new(a_entry, &a_instance.instance);

    Ok(Surface{surface: surface, surface_khr: surface_khr})
  }

  pub fn pick_format(
    &self, 
    a_physical_device: &PhysicalDevice, 
  ) -> Result<ash::vk::SurfaceFormatKHR, RendererError> {
    let surface_formats = unsafe { match self.surface.get_physical_device_surface_formats(
    a_physical_device.device, self.surface_khr){
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    }};

    let format = match Self::pick(&surface_formats){
      Some(res) => res,
      None => return Err(RendererError::Error)
    };

    Ok(*format)
  }

  fn pick(a_formats: &Vec<ash::vk::SurfaceFormatKHR>) -> Option<&ash::vk::SurfaceFormatKHR>{
    for format in a_formats{
      if format.format == ash::vk::Format::B8G8R8A8_SRGB && format.color_space == ash::vk::ColorSpaceKHR::SRGB_NONLINEAR{
        return Some(format)
      }
    }
    None
  }
}

impl Drop for Surface{
  fn drop(&mut self){
    unsafe { self.surface.destroy_surface(self.surface_khr, None) } ;
  }
}

