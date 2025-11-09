
use crate::gpu::command_list::*;
use crate::gpu::resource::*;

use super::device::*;
use super::fence::Fence;
use super::semaphore::Semaphore;

use std::rc::Rc;
use std::sync::Arc;

pub struct CommandListVulkan {
    pub resources: std::vec::Vec<Arc<dyn Resource>>,
    pub command_buffer: ash::vk::CommandBuffer,
    pub fence: Rc<Fence>,
    pub semaphore: Rc<Semaphore>,
    pub is_active: bool,
    pub is_submitted: bool,
}

impl CommandListVulkan {
    pub fn new(
        a_command_buffer: ash::vk::CommandBuffer,
        a_logical_device: Rc<LogicalDevice>,
    ) -> Box<CommandListVulkan> {
        let fence = Rc::new(match Fence::new(a_logical_device.clone()) {
            Ok(res) => res,
            Err(_res) => panic!("Unable to create vulkan fence"),
        });

        let semaphore = Rc::new(match Semaphore::new(a_logical_device.clone()){
            Ok(res) => res,
            Err(_res) => panic!("Unable to create vulkan semaphore"),
        });

        Box::new(Self {
            resources: std::vec::Vec::<Arc<dyn Resource>>::new(),
            command_buffer: a_command_buffer,
            fence: fence,
            semaphore: semaphore,
            is_active: false,
            is_submitted: false,
        })
    }
}

impl CommandList for CommandListVulkan {
    fn any(&self) -> &dyn std::any::Any {
        self
    }

    fn any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}
