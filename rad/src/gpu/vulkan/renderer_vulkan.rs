use ash::{vk, Entry};
use glam::*;
use std::rc::Rc;

use crate::core::unsafe_send::UnsafeSend;
use super::device::{PhysicalDevice, LogicalDevice};
use super::command_pool::CommandPool;
use super::fence::Fence;
use super::instance::Instance;
use super::render_pass::RenderPass;
use super::semaphore::Semaphore;
use super::surface::Surface;
use super::swapchain::Swapchain;
use crate::gpu::renderer::*;
use crate::gpu::renderer_types::*;
use crate::gpu::material::*;
use crate::gpu::camera::*;
use crate::gpu::uniforms::*;
use crate::gpu::image::*;
use std::ffi::CString;
use std::sync::Mutex;
use std::sync::Arc;

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
  pub module: ash::vk::ShaderModule,
  pub logical_device: std::rc::Rc<LogicalDevice>
}

impl Shader for ShaderVulkan {
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

impl Drop for ShaderVulkan {
  fn drop(&mut self){
    unsafe {self.logical_device.device.destroy_shader_module(self.module, None)};
  }
}

pub struct ProgramVulkan {
  pub pipeline: ash::vk::Pipeline,
  pub pipeline_layout: ash::vk::PipelineLayout,
  pub logical_device: std::rc::Rc<LogicalDevice>
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

impl Drop for ProgramVulkan {
  fn drop(&mut self){
    unsafe{ 
      self.logical_device.device.destroy_pipeline(self.pipeline, None);
      self.logical_device.device.destroy_pipeline_layout(self.pipeline_layout, None);
    }
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

  window: Arc<Mutex<UnsafeSend<sdl2::video::Window>>>,
  
  // Order matters here so that instance is destroyed last
  framebuffer_format: ash::vk::SurfaceFormatKHR,
  image_available_semaphores: std::vec::Vec::<Semaphore>,
  render_finished_semaphores: std::vec::Vec::<Semaphore>,
  render_fences: std::vec::Vec<Fence>,
  command_buffers: std::vec::Vec<ash::vk::CommandBuffer>,
  command_pool: CommandPool,
  swapchain: Swapchain,
  render_pass: RenderPass,
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

    let mut attempt = true;
    while attempt && self.swapchain.extent.width > 0 && self.swapchain.extent.height > 0{
      attempt = false;

      let (image_index, _suboptimal) = unsafe { match self.swapchain.swapchain.swapchain_loader.acquire_next_image(
        self.swapchain.swapchain.swapchain, 
        u64::MAX, 
        self.image_available_semaphores[current_frame].semaphore, 
        ash::vk::Fence::null()) {
          Ok(res) => res,
          Err(res) if res == ash::vk::Result::ERROR_OUT_OF_DATE_KHR => {
            self.recreate_swapchain();
            attempt = true;
            (0,true)
          },
          Err(res) => {
            println!("Error: reset_command_buffer: {}", res);
            return
          }
      }};

      self.image_index = image_index;
    }

    if self.swapchain.extent.width > 0 && self.swapchain.extent.height > 0{
      
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
      let render_pass_info = ash::vk::RenderPassBeginInfo::builder()
        .render_pass(self.render_pass.render_pass)
        .framebuffer(self.swapchain.framebuffers[self.image_index as usize].framebuffer)
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
  }

  fn end_frame(&mut self){
    let current_frame = self.current_frame as usize;
    let mut recreate = false;

    if self.swapchain.extent.width > 0 && self.swapchain.extent.height > 0{
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

      let swapchains = [self.swapchain.swapchain.swapchain];
      let image_indices = [self.image_index];

      let present_info = ash::vk::PresentInfoKHR::builder()
        .wait_semaphores(&signal_semaphores)
        .swapchains(&swapchains)
        .image_indices(&image_indices)
        .build();

      let window_size = self.window.lock().unwrap().inner.size();
      let window_resize = self.swapchain.extent.width != window_size.0 || self.swapchain.extent.height != window_size.1;
      // TODO handle result
      recreate = match unsafe { self.swapchain.swapchain.swapchain_loader.queue_present(self.logical_device.queue, &present_info) }{
        Ok(_) => window_resize,
        Err(res) if res == ash::vk::Result::ERROR_OUT_OF_DATE_KHR || res == ash::vk::Result::SUBOPTIMAL_KHR=> true,
        Err(res) => {println!("Error: queue_present {}", res); false} 
      };

      self.current_frame = (self.current_frame + 1) % Self::MAX_FRAMES;
    }
    else {
      recreate = true;
    }

    if recreate{
      self.recreate_swapchain();
    }
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
    return Err(RendererError::Unimplemented)
  }

  fn load_shader_intermediate(&mut self, _shader_type: ShaderType, a_source: &std::vec::Vec::<u8>) -> Result<Box<dyn Shader>, RendererError>{
    
    if a_source.len() % 4 != 0{
      return Err(RendererError::Error)
    }

    let slice: &[u32] = unsafe {
      std::slice::from_raw_parts(a_source.as_ptr() as *const u32, a_source.len() / 4)
    };
    
    let create_info = ash::vk::ShaderModuleCreateInfo::builder()
      .code(slice)
      .build();

    let module = match unsafe { self.logical_device.device.create_shader_module(&create_info, None) } {
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    };

    Ok(Box::new(ShaderVulkan{module: module, logical_device: self.logical_device.clone()}))
  }

  fn load_program_vert_frag(&mut self, a_shader_vert: Box<dyn Shader>, a_shader_frag: Box<dyn Shader>) -> Result<Box<dyn Program>, RendererError>{
    let vertex_module = match a_shader_vert.any().downcast_ref::<ShaderVulkan>() {
      Some(res) => res,
      None => return Err(RendererError::InvalidCast)
    };

    let frag_module = match a_shader_frag.any().downcast_ref::<ShaderVulkan>() {
      Some(res) => res,
      None => return Err(RendererError::InvalidCast)
    };

    let main_function_name = CString::new("main").unwrap();

    let vertex_info = ash::vk::PipelineShaderStageCreateInfo::builder()
      .stage(ash::vk::ShaderStageFlags::VERTEX)
      .module(vertex_module.module)
      .name(main_function_name.as_c_str())
      .build();

    let frag_info = ash::vk::PipelineShaderStageCreateInfo::builder()
      .stage(ash::vk::ShaderStageFlags::FRAGMENT)
      .module(frag_module.module)
      .name(main_function_name.as_c_str())
      .build();

    let shader_stages = [vertex_info, frag_info];

    let vertex_state_info: vk::PipelineVertexInputStateCreateInfo = ash::vk::PipelineVertexInputStateCreateInfo::builder()
      //.vertex_binding_descriptions(vertex_binding_descriptions)
      //.vertex_attribute_descriptions(vertex_attribute_descriptions)
      .build();

    let vertex_assembly_info = ash::vk::PipelineInputAssemblyStateCreateInfo::builder()
      .primitive_restart_enable(false)
      .topology(ash::vk::PrimitiveTopology::TRIANGLE_LIST)
      .build();

		let viewports = [ash::vk::Viewport{
      x: 0.0,
      y: 0.0,
      width: self.swapchain.extent.width as f32,
      height: self.swapchain.extent.height as f32,
      min_depth: 0.0,
      max_depth: 1.0}];

    let scissors = [vk::Rect2D {
      offset: vk::Offset2D { x: 0, y: 0 },
      extent: self.swapchain.extent,
    }];

    let viewport_state_info = ash::vk::PipelineViewportStateCreateInfo::builder()
      .viewports(&viewports)
      .scissors(&scissors)
      .build();

    let rasterization_state_info = ash::vk::PipelineRasterizationStateCreateInfo::builder()
      .depth_clamp_enable(false)
      .cull_mode(ash::vk::CullModeFlags::BACK)
      .front_face(vk::FrontFace::CLOCKWISE)
      .line_width(1.0)
      .polygon_mode(ash::vk::PolygonMode::FILL)
      .rasterizer_discard_enable(false)
      .depth_bias_clamp(0.0)
      .depth_bias_constant_factor(0.0)
      .depth_bias_enable(false)
      .depth_bias_slope_factor(0.0)
      .build();

    let multisample_state_create_info = ash::vk::PipelineMultisampleStateCreateInfo::builder()
      .rasterization_samples(ash::vk::SampleCountFlags::TYPE_1)
      .sample_shading_enable(false)
      .min_sample_shading(0.0)
      .alpha_to_one_enable(false)
      .alpha_to_coverage_enable(false)
      .build();

    let stencil_state = ash::vk::StencilOpState::builder()
      .fail_op(ash::vk::StencilOp::KEEP)
      .pass_op(ash::vk::StencilOp::KEEP)
      .depth_fail_op(ash::vk::StencilOp::KEEP)
      .compare_op(ash::vk::CompareOp::ALWAYS)
      .compare_mask(0)
      .write_mask(0)
      .reference(0)
      .build();

    let depth_state_create_info = ash::vk::PipelineDepthStencilStateCreateInfo::builder()
      .depth_test_enable(false)
      .depth_write_enable(false)
      .depth_compare_op(ash::vk::CompareOp::LESS_OR_EQUAL)
      .depth_bounds_test_enable(false)
      .stencil_test_enable(false)
      .front(stencil_state)
      .back(stencil_state)
      .max_depth_bounds(1.0)
      .min_depth_bounds(0.0)
      .build();

    let color_blend_attachment_states = [ash::vk::PipelineColorBlendAttachmentState::builder()
      .blend_enable(false)
      .color_write_mask(ash::vk::ColorComponentFlags::RGBA)
      .src_color_blend_factor(ash::vk::BlendFactor::ONE)
      .dst_color_blend_factor(ash::vk::BlendFactor::ZERO)
      .color_blend_op(ash::vk::BlendOp::ADD)
      .src_alpha_blend_factor(ash::vk::BlendFactor::ONE)
      .dst_alpha_blend_factor(ash::vk::BlendFactor::ZERO)
      .alpha_blend_op(ash::vk::BlendOp::ADD)
      .build()];

    let color_blend_state = ash::vk::PipelineColorBlendStateCreateInfo::builder()
      .logic_op_enable(false)
      .logic_op(ash::vk::LogicOp::COPY)
      .attachments(&color_blend_attachment_states)
      .blend_constants([0.0, 0.0, 0.0, 0.0])
      .build();

    /*
		let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];

    let dynamic_state_info = ash::vk::PipelineDynamicStateCreateInfo::builder()
      .dynamic_states(&dynamic_states)
      .build();
    */

    let pipeline_layout_info = ash::vk::PipelineLayoutCreateInfo::builder()
      .build();

    let pipeline_layout = match unsafe { self.logical_device.device.create_pipeline_layout(&pipeline_layout_info, None) }{
      Ok(res) => res,
      Err(_res) => return Err(RendererError::ShaderCompile)
    };

    let create_info = ash::vk::GraphicsPipelineCreateInfo::builder()
      .stages(&shader_stages)
      .vertex_input_state(&vertex_state_info)
      .input_assembly_state(&vertex_assembly_info)
      .viewport_state(&viewport_state_info)
      .rasterization_state(&rasterization_state_info)
      .multisample_state(&multisample_state_create_info)
      .depth_stencil_state(&depth_state_create_info)
      .color_blend_state(&color_blend_state)
      .layout(pipeline_layout)
      .render_pass(self.render_pass.render_pass)
      .subpass(0)
      .base_pipeline_index(-1)
      .build();

    let pipeline_infos = [create_info];

    let graphics_pipelines = match unsafe { self.logical_device.device.create_graphics_pipelines(ash::vk::PipelineCache::null(), &pipeline_infos, None) }{
      Ok(res) => res,
      Err(_res) => return Err(RendererError::ShaderCompile)
    };

    Ok(Box::new(ProgramVulkan{
      pipeline: graphics_pipelines[0], 
      pipeline_layout: pipeline_layout,
      logical_device: self.logical_device.clone()}))
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
  fn draw_mesh(&mut self, _camera: &Camera, a_mesh: &mut Box<Mesh>){
    // let geometry = match a_mesh.geometry.any().downcast_ref::<GeometryVulkan>() {
    //   Some(res) => res,
    //   None => return
    // };

    let program = match a_mesh.material.get_program().any().downcast_ref::<ProgramVulkan>(){
      Some(res) => res,
      None => return
    };

    unsafe { 
      // TODO Only set pipeline if not already set
      self.logical_device.device.cmd_bind_pipeline(
      self.command_buffers[self.current_frame as usize],
      ash::vk::PipelineBindPoint::GRAPHICS, 
      program.pipeline);

      self.logical_device.device.cmd_draw(self.command_buffers[self.current_frame as usize], 3, 1, 0, 0);
    }
  }

  fn read_render_buffer(&mut self) -> Image{
    return Image{width: 0, height: 0, pitch: 0, pixels: std::vec::Vec::<u8>::new()}
  }
}

impl RendererVulkan{
  pub const MAX_FRAMES: u32 = 2;

  pub fn new(a_window: Arc<Mutex<UnsafeSend<sdl2::video::Window>>>, a_enable_validation_layers: bool) -> Result<Self, RendererError>{
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

    let surface = match Surface::new(&a_window.lock().unwrap().inner, &entry, &instance) {
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

    let format = match surface.pick_format(&physical_device){
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    };

    let render_pass = match RenderPass::new(logical_device.clone(), format.format){
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    };

    let window_size = a_window.lock().unwrap().inner.size();
    let extent = ash::vk::Extent2D{width: window_size.0, height: window_size.1};

    let swapchain = match Swapchain::new(
      logical_device.clone(), 
      &instance, 
      &surface, 
      &physical_device, 
      &render_pass,
      &format,
      extent){
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
      window: a_window,
      framebuffer_format: format,
      image_available_semaphores: image_available_semaphores,
      render_finished_semaphores: render_finished_semaphores,
      render_fences: render_fences,
      command_buffers: command_buffers,
      command_pool: command_pool,
      swapchain: swapchain,
      render_pass: render_pass,
      logical_device: logical_device,
      physical_device: physical_device,
      surface: surface,
      instance: instance,
    })
  }

  fn recreate_swapchain(&mut self) -> Result<(), RendererError>{
    let window_size = self.window.lock().unwrap().inner.size();
    let extent = ash::vk::Extent2D{width: window_size.0, height: window_size.1};

    match unsafe{self.logical_device.device.device_wait_idle()}{
      Ok(_) =>{},
      Err(res) => println!("Error: device_wait_idle: {}", res)
    };

    //recreate
    self.swapchain.clear();
    self.swapchain = match Swapchain::new(
      self.logical_device.clone(),
      &self.instance,
      &self.surface,
      &self.physical_device,
      &self.render_pass,
      &self.framebuffer_format,
      extent
    ){
      Ok(res) => res,
      Err(res) => {
        println!("Error: Swapchain::new: {}", res);
        return Err(RendererError::Error)
      }
    };
    return Ok(())
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
