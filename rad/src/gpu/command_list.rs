// use crate::gpu::camera::*;
// use crate::gpu::mesh::*;
// use std::cell::RefCell;
// use std::rc::Rc;

pub trait CommandList {
    fn any(&self) -> &dyn std::any::Any;
    fn any_mut(&mut self) -> &mut dyn std::any::Any;
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any>;
}
