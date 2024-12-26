use crate::gpu::command_list::*;

pub struct CommandListOpenGL {
    
}

impl CommandListOpenGL{
    pub fn new() -> Box<CommandListOpenGL>{
        Box::new(Self{})
    }
}

impl CommandList for CommandListOpenGL {

}
