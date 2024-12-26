use crate::gpu::camera::*;
use crate::gpu::mesh::*;
use std::cell::RefCell;
use std::rc::Rc;

pub trait CommandList {
    fn any(&self) -> &dyn std::any::Any;

    fn draw_mesh(&mut self, a_camera: &Camera, a_mesh: Rc<RefCell<Mesh>>);
}
