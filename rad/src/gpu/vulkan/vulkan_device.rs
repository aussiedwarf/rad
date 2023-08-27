use crate::gpu::renderer_types::RendererError;
use crate::gpu::vulkan::vulkan_instance::VulkanInstance;

use std::ffi::{CStr};

pub struct VulkanLogicalDevice{
  device: ash::Device
}

impl VulkanLogicalDevice{
  pub fn new(a_instance: &VulkanInstance, a_physical_device: &VulkanPhysicalDevice) -> Result<VulkanLogicalDevice, RendererError> {
    let queue_priorities = [1.0];
    
    let queue_info = [ash::vk::DeviceQueueCreateInfo::builder()
      .queue_family_index(a_physical_device.queue_family as u32)
      .queue_priorities(&queue_priorities)
      .build()];
    
    let create_info = ash::vk::DeviceCreateInfo::builder()
      .queue_create_infos(&queue_info)
      .build();
    let device = unsafe { match a_instance.instance.create_device(a_physical_device.device, &create_info, None){
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    }};

    Ok(VulkanLogicalDevice{device: device})
  }
}

impl Drop for VulkanLogicalDevice{
  fn drop(&mut self){
    unsafe { self.device.destroy_device(None) };
  }
}

pub struct VulkanPhysicalDevice{
  device: ash::vk::PhysicalDevice,
  queue_family: usize
}

impl VulkanPhysicalDevice{
  pub fn new(a_instance: &VulkanInstance) -> Result<VulkanPhysicalDevice, RendererError> {
    let devices = unsafe { match a_instance.instance.enumerate_physical_devices() {
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    }};

    let extensions = std::vec!["VK_KHR_swapchain"];

    let device = match VulkanPhysicalDevice::select_device_type(a_instance, &devices, &extensions){
      Some(res) => res,
      None => return Err(RendererError::Error)
    };

    let queue_family = match VulkanPhysicalDevice::get_queue_family(a_instance, &device) {
      Some(res) => res,
      None => return Err(RendererError::Error)
    };

    return Ok(VulkanPhysicalDevice{device: device, queue_family: queue_family})
  }

  fn get_queue_family(a_instance: &VulkanInstance, a_device: &ash::vk::PhysicalDevice) -> Option<usize>{
    let queue_famalies = unsafe { a_instance.instance.get_physical_device_queue_family_properties(*a_device) };

    for (index, family) in queue_famalies.iter().enumerate(){
      if family.queue_count > 0 && family.queue_flags.contains(ash::vk::QueueFlags::GRAPHICS){
        return Some(index)
      }
    }

    None
  }

  fn select_device_type(a_instance: &VulkanInstance, a_devices: &Vec<ash::vk::PhysicalDevice>, a_extensions: &std::vec::Vec<&str>) -> Option<ash::vk::PhysicalDevice>{
    let device_types = [
      ash::vk::PhysicalDeviceType::DISCRETE_GPU, 
      ash::vk::PhysicalDeviceType::INTEGRATED_GPU,
      ash::vk::PhysicalDeviceType::VIRTUAL_GPU,
      ash::vk::PhysicalDeviceType::CPU];
    for device_type in device_types{
      let device_result = VulkanPhysicalDevice::select_device(a_instance, a_devices, a_extensions, device_type);

      if device_result.is_some(){
        return Some(device_result.unwrap())
      }
    };

    return None
  }

  fn select_device(a_instance: &VulkanInstance, a_devices: &Vec<ash::vk::PhysicalDevice>, a_extensions: &std::vec::Vec<&str>, a_device_type: ash::vk::PhysicalDeviceType) -> Option<ash::vk::PhysicalDevice> {
    for device in a_devices{
      let properties = unsafe { a_instance.instance.get_physical_device_properties(*device) };

      if properties.device_type == a_device_type{
        if VulkanPhysicalDevice::check_extension_support(a_instance, device, a_extensions){
          return Some(*device)
        }
      }
    }

    None
  }

  fn check_extension_support(a_instance: &VulkanInstance, a_device: &ash::vk::PhysicalDevice,  a_extensions: &std::vec::Vec<&str>) -> bool{
    let properties = unsafe { a_instance.instance.enumerate_device_extension_properties(*a_device).unwrap() };
    
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
