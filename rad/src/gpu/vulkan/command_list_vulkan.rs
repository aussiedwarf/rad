use crate::gpu::camera::*;
use crate::gpu::command_list::*;
use crate::gpu::mesh::*;
use crate::gpu::resource::*;

use super::shader::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct CommandListVulkan {
    resources: std::vec::Vec<Rc<RefCell<dyn Resource>>>
}

impl CommandListVulkan{
    pub fn new() -> Box<CommandListVulkan>{
        Box::new(Self{resources: std::vec::Vec::<Rc<RefCell<dyn Resource>>>::new()})
    }
}

impl CommandList for CommandListVulkan {
    fn any(&self) -> &dyn std::any::Any {
        self
    }

    fn draw_mesh(&mut self, _camera: &Camera, a_mesh: Rc<RefCell<Mesh>>) {
        /*
        if !self.renderer_ready {
            return;
        }
        // let geometry = match a_mesh.geometry.any().downcast_ref::<GeometryVulkan>() {
        //   Some(res) => res,
        //   None => return
        // };
        let mesh = a_mesh.borrow();
        let program_rc = mesh.material.get_program();
        let program = match program_rc.any().downcast_ref::<ProgramVulkan>() {
            Some(res) => res,
            None => return,
        };

        let current_frame = self.current_frame as usize;

        unsafe {
            // TODO Only set pipeline if not already set
            self.logical_device.device.cmd_bind_pipeline(
                self.command_buffers[current_frame],
                ash::vk::PipelineBindPoint::GRAPHICS,
                program.pipeline,
            );

            self.logical_device
                .device
                .cmd_draw(self.command_buffers[current_frame], 3, 1, 0, 0);
        }

        self.resources[current_frame].push(a_mesh.clone());
        */
    }
}
