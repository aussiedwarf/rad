use crate::gpu::command_list::*;

pub struct CommandListDirectX12 {
    
}

impl CommandListDirectX12{
    pub fn new() -> Box<CommandListDirectX12>{
        Box::new(Self{})
    }
}

impl CommandList for CommandListDirectX12 {

}
