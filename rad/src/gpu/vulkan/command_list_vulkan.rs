use crate::gpu::camera::*;
use crate::gpu::command_list::*;
use crate::gpu::mesh::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct CommandListVulkan {
    
}

impl CommandListVulkan{
    pub fn new() -> Box<CommandListVulkan>{
        Box::new(Self{})
    }
}

impl CommandList for CommandListVulkan {
    fn any(&self) -> &dyn std::any::Any {
        self
    }

    fn draw_mesh(&mut self, _camera: &Camera, _a_mesh: Rc<RefCell<Mesh>>) {}
}
