
use std::ffi::{CString, CStr};
use libc::{c_char};

use crate::gpu::renderer_types::*;

use ash::{vk, Entry};

pub struct VulkanInstance{
  pub instance: ash::Instance
}

impl VulkanInstance{
  pub fn new(a_entry: &Entry, a_enable_validation_layers: bool) -> Result<Self, RendererError>{
    //let validation_layer_names: Vec<[i8, vk::constants::MAX_EXTENSION_NAME_SIZE]>;

    //const REQUIRED_LAYERS: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];
    let mut layers = std::vec::Vec::<&str>::new();

    if a_enable_validation_layers {
      layers.push("VK_LAYER_KHRONOS_validation");
    }

    match VulkanInstance::check_instance_layer_support(a_entry, &layers){
      false => return Err(RendererError::Error),
      true => {}
    };

    let mut extensions = std::vec::Vec::<&str>::new();
    
    extensions.push("VK_KHR_surface");
    #[cfg(windows)]
    extensions.push("VK_KHR_win32_surface");

    match VulkanInstance::check_instance_extension_support(a_entry, &extensions){
      false => return Err(RendererError::Error),
      true => {}
    }

    let (_layer_names, layer_names_ptrs) = VulkanInstance::get_names_and_pointers(&layers);
    let (_extension_names, extension_names_ptrs) = VulkanInstance::get_names_and_pointers(&extensions);

    let app_info = vk::ApplicationInfo::builder()
      .api_version(vk::make_api_version(0, 1, 0, 0))
      .build();

    let create_info = vk::InstanceCreateInfo::builder()
      .application_info(&app_info)
      .enabled_extension_names(&extension_names_ptrs)
      .enabled_layer_names(&layer_names_ptrs);

    println!("start Instance");
    let instance = unsafe {match a_entry.create_instance(&create_info, None){
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    }};
    println!("Instance created");

    Ok(VulkanInstance{instance: instance})
  }

  // Get the pointers to the validation layers names.
  // Also return the corresponding `CString` to avoid dangling pointers.
  fn get_names_and_pointers(a_layers: &std::vec::Vec<&str>) -> (Vec<CString>, Vec<*const c_char>) {
    let layer_names = a_layers
      .iter()
      .map(|name| CString::new(*name).unwrap())
      .collect::<Vec<_>>();
    let layer_names_ptrs = layer_names
      .iter()
      .map(|name| name.as_ptr())
      .collect::<Vec<_>>();
    (layer_names, layer_names_ptrs)
  }

  fn check_instance_layer_support(a_entry: &Entry, a_layers: &std::vec::Vec<&str>) -> bool{
    let properties = a_entry.enumerate_instance_layer_properties().unwrap();
    
    for required in a_layers.iter() {
      let found = properties
        .iter()
        .any(|layer| {
          let name = unsafe { CStr::from_ptr(layer.layer_name.as_ptr()) };
          let name = name.to_str().expect("Failed to get layer name pointer");
          required == &name
        });

      if !found {
        return false
      }
    }

    return true
  }

  fn check_instance_extension_support(a_entry: &Entry, a_extensions: &std::vec::Vec<&str>) -> bool{
    let properties: Vec<vk::ExtensionProperties> = a_entry.enumerate_instance_extension_properties(None).unwrap();
    
    for required in a_extensions.iter() {
      let found = properties
        .iter()
        .any(|ext| {
          let name = unsafe { CStr::from_ptr(ext.extension_name.as_ptr()) };
          let name = name.to_str().expect("Failed to get extension name pointer");
          required == &name
        });

      if !found {
        return false
      }
    }

    return true
  }
}

impl Drop for VulkanInstance{
  fn drop(&mut self){
    unsafe { self.instance.destroy_instance(None) };
  }
}
