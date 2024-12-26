use crate::gpu::camera::*;
use crate::gpu::command_list::*;
use crate::gpu::mesh::*;

use std::cell::RefCell;
use std::rc::Rc;

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

    fn draw_mesh(&mut self, _camera: &Camera, _a_mesh: Rc<RefCell<Mesh>>) {}
}
