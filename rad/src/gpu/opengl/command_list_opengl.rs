
use crate::gpu::command_list::*;

pub struct CommandListOpenGL {
    
}

impl CommandListOpenGL{
    pub fn new() -> Box<CommandListOpenGL>{
        Box::new(Self{})
    }
}

impl CommandList for CommandListOpenGL {
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
