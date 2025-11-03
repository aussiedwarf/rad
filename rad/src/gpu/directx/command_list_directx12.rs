use crate::gpu::command_list::*;

pub struct CommandListDirectX12 {
    
}

impl CommandListDirectX12{
    pub fn new() -> Box<CommandListDirectX12>{
        Box::new(Self{})
    }
}

impl CommandList for CommandListDirectX12 {
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
