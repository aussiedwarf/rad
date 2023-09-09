use ash::{vk, Entry};
use glam::*;
use sdl2::video::VkSurfaceKHR;
use std::rc::Rc;

use super::device::{PhysicalDevice, LogicalDevice};
use super::command_pool::CommandPool;
use super::fence::Fence;
use super::instance::Instance;
use super::semaphore::Semaphore;
use super::surface::Surface;
use super::swapchain::Swapchain;
use crate::gpu::renderer::*;
use crate::gpu::renderer_types::*;
use crate::gpu::material::*;
use crate::gpu::camera::*;
use crate::gpu::uniforms::*;
use crate::gpu::image::*;
use libc::{c_char};

pub struct SamplerVulkan{
  name: String,
}

impl Sampler for SamplerVulkan {
  fn any(&self) -> &dyn std::any::Any{
    self
  }

  fn set_name(&mut self, a_name: &str){
    self.name = String::from(a_name);
  }
}

#[allow(dead_code)]
pub struct VerticesVulkan {
  id: i32
}

impl Vertices for VerticesVulkan {
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

#[allow(dead_code)]
pub struct GeometryVulkan {
  id: i32
}

impl Geometry for GeometryVulkan {
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

#[allow(dead_code)]
pub struct TextureVulkan {
  id: i32
}

impl Texture for TextureVulkan {
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

pub struct ShaderVulkan {

}

impl Shader for ShaderVulkan {
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

pub struct ProgramVulkan {
}

impl Program for ProgramVulkan {
  fn any(&self) -> &dyn std::any::Any{
    self
  }

  fn get_uniform(&self, a_name: &str, a_data: UniformData) -> Box<dyn Uniform>{
    Box::new(UniformVulkan{
      name: UniformName::new(a_name),
      data: a_data,
    })
  }
}

#[allow(dead_code)]
pub struct UniformVulkan {
  name: UniformName,
  data: UniformData,
}

impl Uniform for UniformVulkan {
  fn any(&mut self) -> &mut dyn std::any::Any{
    self
  }

  fn set_f32(&mut self, _a: f32){
  }

  fn get_f32(&self) -> f32{
    0.0
  }
 
  fn get_name(&self) -> &str{
    &self.name.get_name()
  }
  
  fn set_name(&mut self, a_name: &str){
    self.name.set_name(a_name);
  }
}

#[allow(dead_code)]
pub struct UniformShaderVulkan {
  name: UniformName,
  id: i32 //todo check type
}

impl UniformShader for UniformShaderVulkan {
  fn any(&mut self) -> &mut dyn std::any::Any{
    self
  }
}

pub struct RendererVulkan {
  pub version_major: i32,
  pub version_minor: i32,
  pub version_patch: i32,

  clear_color: Vec4,
  clear_depth: f32,
  clear_stencil: i32,

  current_frame: u32,
  image_index: u32,
  
  // Order matters here so that instance is destroyed last
  image_available_semaphores: std::vec::Vec::<Semaphore>,
  render_finished_semaphores: std::vec::Vec::<Semaphore>,
  render_fences: std::vec::Vec<Fence>,
  command_buffers: std::vec::Vec<ash::vk::CommandBuffer>,
  command_pool: CommandPool,
  swapchain: Swapchain,
  logical_device: Rc<LogicalDevice>,
  physical_device: PhysicalDevice,
  surface: Surface,
  instance: Instance,
}

#[allow(dead_code)]
impl Renderer for RendererVulkan {
  fn name(&self) -> String{
    String::from("Vulkan")
  }

  fn get_type(&self) -> RendererType{
    RendererType::Vulkan
  }

  fn begin_frame(&mut self, _clear: RendererClearType){
    let current_frame = self.current_frame as usize;

    let (image_index, suboptimal) = unsafe { match self.swapchain.swapchain_loader.acquire_next_image(
      self.swapchain.swapchain, 
      u64::MAX, 
      self.image_available_semaphores[current_frame].semaphore, 
      ash::vk::Fence::null()) {
        Ok(res) => res,
        Err(res) => {
          println!("Error: reset_command_buffer {}", res);
          return
        }
    }};

    self.image_index = image_index;

    match unsafe {self.logical_device.device.reset_command_buffer(self.command_buffers[current_frame], ash::vk::CommandBufferResetFlags::empty())}{
      Ok(_) => {},
      Err(res) => {println!("Error: reset_command_buffer {}", res)}
    };

    let begin_info = ash::vk::CommandBufferBeginInfo::builder()
      .build();

    match unsafe { self.logical_device.device.begin_command_buffer(self.command_buffers[current_frame], &begin_info) }{
      Ok(_) => {},
      Err(res) => {println!("Error: begin_command_buffer {}", res)}
    };

    let clear_values = [ash::vk::ClearValue {
      color: ash::vk::ClearColorValue{float32: self.clear_color.to_array()}}];
    let mut render_pass_info = ash::vk::RenderPassBeginInfo::builder()
      .render_pass(self.swapchain.render_pass.render_pass)
      .framebuffer(self.swapchain.framebuffers[image_index as usize].framebuffer)
      .render_area(ash::vk::Rect2D{offset: ash::vk::Offset2D{x:0,y:0}, extent: self.swapchain.extent})
      .clear_values(&clear_values)
      .build();

    unsafe { self.logical_device.device.cmd_begin_render_pass(
      self.command_buffers[current_frame], 
      &render_pass_info, 
      ash::vk::SubpassContents::INLINE)  };

    let viewports = [ash::vk::Viewport::builder()
      .x(0.0)
      .y(0.0)
      .width(self.swapchain.extent.width as f32)
      .height(self.swapchain.extent.height as f32)
      .min_depth(0.0)
      .max_depth(0.0)
      .build()];

    unsafe { self.logical_device.device.cmd_set_viewport(self.command_buffers[current_frame], 0, &viewports) };

    let scissors = [ash::vk::Rect2D{offset: ash::vk::Offset2D{x:0,y:0}, extent: self.swapchain.extent}];
    unsafe { self.logical_device.device.cmd_set_scissor(
      self.command_buffers[current_frame], 
      0, 
      &scissors)};
  }

  fn end_frame(&mut self){
    
    let current_frame = self.current_frame as usize;

    unsafe { self.logical_device.device.cmd_end_render_pass(self.command_buffers[current_frame])};
    
    // TODO handle result
    match unsafe { self.logical_device.device.end_command_buffer(self.command_buffers[current_frame]) } {
      Ok(_) => {},
      Err(res) => {println!("Error: end_command_buffer {}", res)}
    };

    let wait_semaphores = [self.image_available_semaphores[current_frame].semaphore ];

    let wait_stages = [ ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT ];

    let signal_semaphores = [ self.render_finished_semaphores[current_frame].semaphore ];

    let submit_info = ash::vk::SubmitInfo::builder()
      .wait_semaphores(&wait_semaphores)
      .wait_dst_stage_mask(&wait_stages)
      .signal_semaphores(&signal_semaphores)
      .command_buffers(&[self.command_buffers[current_frame]])
      .build();

    let submits = [submit_info];

    // TODO handle result
    match unsafe { self.logical_device.device.queue_submit(self.logical_device.queue, &submits, self.render_fences[current_frame].fence) }{
      Ok(_) => {},
      Err(res) => {println!("Error: queue_submit {}", res)}
    };

    let fences = [self.render_fences[current_frame].fence];
    // TODO handle result
    match unsafe { self.logical_device.device.wait_for_fences(&fences, true, u64::MAX) }{
      Ok(_) => {},
      Err(res) => {println!("Error: wait_for_fences {}", res)}
    };
    // TODO handle result
    match unsafe { self.logical_device.device.reset_fences(&fences) }{
      Ok(_) => {},
      Err(res) => {println!("Error: reset_fences {}", res)}
    };

    let swapchains = [self.swapchain.swapchain];
    let image_indices = [self.image_index];

    let present_info = ash::vk::PresentInfoKHR::builder()
      .wait_semaphores(&signal_semaphores)
      .swapchains(&swapchains)
      .image_indices(&image_indices)
      .build();

    // TODO handle result
    match unsafe { self.swapchain.swapchain_loader.queue_present(self.logical_device.queue, &present_info) }{
      Ok(_) => {},
      Err(res) => {println!("Error: queue_present {}", res)}
    };

    self.current_frame = (self.current_frame + 1) % Self::MAX_FRAMES;
  }

  //clear immediatly
  //= RendererClearColor | RendererClearDepth | RendererClearStencil
  fn clear(&mut self, _clear: RendererClearType){}

  //Get and set clear values may be called before BeginFrame
  fn set_clear_color(&mut self, a_color: Vec4){
    self.clear_color = a_color;
  }
  fn set_clear_depth(&mut self, a_depth: f32){
    self.clear_depth = a_depth;
  }
  fn set_clear_stencil(&mut self, a_stencil: i32){
    self.clear_stencil = a_stencil;
  }
  fn get_clear_color(&self) -> Vec4{
    self.clear_color
  }
  fn get_clear_depth(&self) -> f32{
    self.clear_depth
  }

  fn get_clear_stencil(&self) -> i32{
    self.clear_stencil
  }

  fn set_viewport(&mut self, _pos: IVec2, _size: IVec2){}

  fn get_viewport_pos(&self) -> IVec2{
    IVec2::new(0,0)
  }
  fn get_viewport_size(&self) -> IVec2{
    IVec2::new(0,0)
  }

  fn load_shader(&mut self, _shader_type: ShaderType, _source: &str) -> Result<Box<dyn Shader>, RendererError>{
    // Err(RendererError::Unimplemented)
    Ok(Box::new(ShaderVulkan{}))
  }

  fn load_program_vert_frag(&mut self, _shader_vert: Box<dyn Shader>, _shader_frag: Box<dyn Shader>) -> Result<Box<dyn Program>, RendererError>{
    //Err(RendererError::Unimplemented)
    Ok(Box::new(ProgramVulkan{}))
  }

  fn get_uniform(&mut self, _shader: &mut Box<dyn Program>, a_name: &str) -> Box<dyn UniformShader>{
    Box::new(UniformShaderVulkan{name: UniformName::new(a_name), id: 0})
  }

  //fn set_uniform(&mut self, a_uniform: &Box<dyn Uniform>){}

  //fn set_texture(&mut self, a_texture: &Box<dyn Texture>){}

  fn gen_buffer_vertex(&mut self, _verts: &std::vec::Vec<f32>) -> Box<dyn Vertices>{
    Box::new(VerticesVulkan{id: 0})
  }

  fn gen_geometry(&mut self, _buffer: &Box<dyn Vertices>) -> Box<dyn Geometry>{
    Box::new(GeometryVulkan{id: 0})
  }

  fn gen_mesh(&mut self, a_geometry: Box<dyn Geometry>, a_material: Box<dyn Material>) -> Box<Mesh>{
    Box::new(Mesh{
      geometry: a_geometry,
      material: a_material
      })
  }


  fn gen_buffer_texture(&mut self) -> Box<dyn Texture>{
    Box::new(TextureVulkan{id: 0})
  }

  fn gen_sampler(&mut self, _texture: Rc<dyn Texture>) -> Box<dyn Sampler>{
    Box::new(SamplerVulkan{name: String::from("")})
  }

  fn load_texture(&mut self, _image: &image::DynamicImage, _texture: &mut Box<dyn Texture>){
  }

  fn use_program(&mut self, _program: &Box<dyn Program>){}

  fn draw_geometry(&mut self, _geometry: &Box<dyn Geometry>){}
  fn draw_mesh(&mut self, _camera: &Camera, _geometry: &mut Box<Mesh>){}

  fn read_render_buffer(&mut self) -> Image{
    return Image{width: 0, height: 0, pitch: 0, pixels: std::vec::Vec::<u8>::new()}
  }
}

impl RendererVulkan{
  pub const MAX_FRAMES: u32 = 2;

  pub fn new(a_window: &sdl2::video::Window, a_enable_validation_layers: bool) -> Result<Self, RendererError>{
    /*
    let extensions = match a_window.vulkan_instance_extensions(){
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    };
    */

    let entry = unsafe { match Entry::load()  {
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    }};

    let instance = match Instance::new(&entry, a_enable_validation_layers){
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    };

    let mut major = 1;
    let mut minor = 0;
    let mut patch = 0;
    
    match entry.try_enumerate_instance_version().unwrap() {
      Some(version) => {
        major = vk::api_version_major(version) as i32;
        minor = vk::api_version_minor(version) as i32;
        patch = vk::api_version_patch(version) as i32;
      },
      None => {},
    };

    let surface = match Surface::new(a_window, &entry, &instance) {
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    };

    let extensions = std::vec!["VK_KHR_swapchain"];

    let physical_device = match PhysicalDevice::new(&instance, &extensions){
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    };

    let logical_device = match LogicalDevice::new(&instance, &physical_device, &extensions){
      Ok(res) => Rc::new(res),
      Err(_res) => return Err(RendererError::Error)
    };

    let window_size = a_window.size();
    let extent = ash::vk::Extent2D{width: window_size.0, height: window_size.1};

    let swapchain = match Swapchain::new(logical_device.clone(), &instance, &surface, &physical_device, extent){
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    };

    let command_pool = match CommandPool::new(logical_device.clone(), physical_device.queue_family as u32){
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    };

    let command_buffers = match command_pool.allocate_command_buffer(Self::MAX_FRAMES){
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    };

    let mut image_available_semaphores = std::vec::Vec::<Semaphore>::new();
    let mut render_finished_semaphores = std::vec::Vec::<Semaphore>::new();
    let mut render_fences = std::vec::Vec::<Fence>::new();

    for _ in 0..Self::MAX_FRAMES{
      image_available_semaphores.push( match Semaphore::new(logical_device.clone()){
        Ok(res) => res,
        Err(_res) => return Err(RendererError::Error)
      });
      render_finished_semaphores.push( match Semaphore::new(logical_device.clone()){
        Ok(res) => res,
        Err(_res) => return Err(RendererError::Error)
      });
      render_fences.push( match Fence::new(logical_device.clone()){
        Ok(res) => res,
        Err(_res) => return Err(RendererError::Error)
      });
    }

    Ok(Self {
      version_major: major,
      version_minor: minor,
      version_patch: patch,
      clear_color: Vec4::new(0.0, 0.0, 0.0, 0.0),
      clear_depth: 1.0,
      clear_stencil: 0,
      current_frame: 0,
      image_index: 0,
      image_available_semaphores: image_available_semaphores,
      render_finished_semaphores: render_finished_semaphores,
      render_fences: render_fences,
      command_buffers: command_buffers,
      command_pool: command_pool,
      swapchain: swapchain,
      logical_device: logical_device,
      physical_device: physical_device,
      surface: surface,
      instance: instance,
    })
  }

}

impl Drop for RendererVulkan{
  fn drop(&mut self){
    match unsafe{self.logical_device.device.device_wait_idle()}{
      Ok(_) =>{},
      Err(res) => println!("Error: device_wait_idle: {}", res)
    };
  }
}
