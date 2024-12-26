use crate::gpu::command_list::*;

pub struct CommandListMetal {
    
}

impl CommandListMetal{
    pub fn new() -> Box<CommandListMetal>{
        Box::new(Self{})
    }
}

impl CommandList for CommandListMetal {

}
