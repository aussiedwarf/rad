use crate::gpu::camera::*;
use crate::gpu::command_list::*;
use crate::gpu::mesh::*;

use std::cell::RefCell;
use std::rc::Rc;

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

    fn draw_mesh(&mut self, _camera: &Camera, _a_mesh: Rc<RefCell<Mesh>>) {}
}
