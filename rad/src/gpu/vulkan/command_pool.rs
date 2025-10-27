use super::command_list_vulkan::CommandListVulkan;
use super::device::LogicalDevice;
use crate::gpu::renderer_types::RendererError;

pub struct CommandPool {
    // TODO: have a vector of command pools to allow for any number of command buffers
    pub pool: ash::vk::CommandPool,
    pub logical_device: std::rc::Rc<LogicalDevice>,
    pub command_buffers: std::vec::Vec::<Option<Box<CommandListVulkan>>>,
}

impl CommandPool {
    pub fn new(
        a_logical_device: std::rc::Rc<LogicalDevice>,
        a_queue_family_index: u32,
        a_command_buffer_count: u32,
    ) -> Result<Self, RendererError> {
        let create_info = ash::vk::CommandPoolCreateInfo::builder()
            .flags(ash::vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(a_queue_family_index)
            .build();
        let pool = unsafe {
            match a_logical_device
                .device
                .create_command_pool(&create_info, None)
            {
                Ok(res) => res,
                Err(_res) => return Err(RendererError::Error),
            }
        };

        //let command_buffers = std::vec::Vec::<Option<Box<CommandListVulkan>>>::new();

        let command_buffers =  match CommandPool::allocate_command_buffer(pool, a_logical_device.clone(), a_command_buffer_count)
        {
            Ok(res) => res,
            Err(err) => return Err(err)
        };

        let command_lists: Vec<Option<Box<CommandListVulkan>>> = command_buffers
            .into_iter()
            .map(|buffer| Some(CommandListVulkan::new(buffer, a_logical_device.clone())))
            .collect();

        Ok(Self {
            pool: pool,
            logical_device: a_logical_device,
            command_buffers: command_lists,
        })
    }

    // TODO WARNING if command pool is destroyed, the buffers are destroyed as well
    fn allocate_command_buffer(
        a_command_pool: ash::vk::CommandPool,
        a_logical_device: std::rc::Rc<LogicalDevice>,
        a_command_buffer_count: u32,
    ) -> Result<std::vec::Vec<ash::vk::CommandBuffer>, RendererError> {
        let allocate_info = ash::vk::CommandBufferAllocateInfo::builder()
            .command_pool(a_command_pool)
            .level(ash::vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(a_command_buffer_count)
            .build();
        let command_buffers = unsafe {
            match a_logical_device
                .device
                .allocate_command_buffers(&allocate_info)
            {
                Ok(res) => res,
                Err(_res) => return Err(RendererError::Error),
            }
        };

        Ok(command_buffers)
    }

    pub fn get_command_list(&mut self) -> Option<Box<CommandListVulkan>> {
        for command_buffer in &mut self.command_buffers.iter_mut() {
            match command_buffer {
                Some(cmd_buf) => {
                    if cmd_buf.is_submitted {
                        let fences = [cmd_buf.fence.fence];
                        unsafe {
                            let status = self.logical_device
                                .device
                                .get_fence_status(cmd_buf.fence.fence)
                                .expect("could not get fence status");
                                // .wait_for_fences(&fences, true, u64::MAX)
                                // .expect("Failed to wait for fences");

                            if(status)
                            {
                                self.logical_device
                                    .device
                                    .reset_fences(&fences)
                                    .expect("Failed to reset fences");

                                cmd_buf.is_submitted = false;
                            }
                        }
                    }

                    if !cmd_buf.is_submitted {
                        return command_buffer.take();
                    }
                }
                None => continue,
            }
        }
        None
    }

    pub fn release_command_list(&mut self, a_command_list: Box<CommandListVulkan>) {
        for command_buffer in &mut self.command_buffers.iter_mut() {
            match command_buffer {
                Some(_) => continue,
                None => {
                    *command_buffer = Some(a_command_list);
                    return
                }
            }
        }
    }
}

impl Drop for CommandPool {
    fn drop(&mut self) {
        unsafe {
            self.logical_device
                .device
                .destroy_command_pool(self.pool, None)
        };
    }
}
