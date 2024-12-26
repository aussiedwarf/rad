use crate::gpu::command_list::*;

pub struct CommandListVulkan {
    
}

impl CommandListVulkan{
    pub fn new() -> Box<CommandListVulkan>{
        Box::new(Self{})
    }
}

impl CommandList for CommandListVulkan {

}
