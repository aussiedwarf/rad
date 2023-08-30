use super::instance::Instance;
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
}

impl Drop for Surface{
  fn drop(&mut self){
    unsafe { self.surface.destroy_surface(self.surface_khr, None) } ;
  }
}

