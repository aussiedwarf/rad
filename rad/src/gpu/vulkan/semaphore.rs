use super::device::LogicalDevice;
use crate::gpu::renderer_types::RendererError;
use crate::gpu::resource::*;
use std::rc::Rc;

pub struct Semaphore {
    pub semaphore: ash::vk::Semaphore,
    pub logical_device: std::rc::Rc<LogicalDevice>,
}

// TODO
// impl Resource for Semaphore {}
pub struct SemaphoreResource {
    pub semaphore: Rc<Semaphore>,
}

impl Resource for SemaphoreResource {}

impl Semaphore {
    pub fn new(a_logical_device: std::rc::Rc<LogicalDevice>) -> Result<Self, RendererError> {
        let semaphore_info = ash::vk::SemaphoreCreateInfo::default();

        let semaphore = match unsafe {
            a_logical_device
                .device
                .create_semaphore(&semaphore_info, None)
        } {
            Ok(res) => res,
            Err(_res) => return Err(RendererError::Error),
        };

        return Ok(Semaphore {
            semaphore: semaphore,
            logical_device: a_logical_device,
        });
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        unsafe {
            self.logical_device
                .device
                .destroy_semaphore(self.semaphore, None)
        };
    }
}
