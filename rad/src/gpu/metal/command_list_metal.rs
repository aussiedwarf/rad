
use crate::gpu::command_list::*;

pub struct CommandListMetal {
    
}

impl CommandListMetal{
    pub fn new() -> Box<CommandListMetal>{
        Box::new(Self{})
    }
}

impl CommandList for CommandListMetal {
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
