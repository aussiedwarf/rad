use ash::{vk, Entry};
use glam::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::ffi::CString;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

use super::command_list_vulkan::*;
use super::command_pool::CommandPool;
use super::device::{LogicalDevice, PhysicalDevice};
use super::fence::Fence;
use super::instance::Instance;
use super::render_pass::RenderPass;
use super::semaphore::Semaphore;
use super::shader::*;
use super::surface::Surface;
use super::swapchain::Swapchain;
use crate::core::unsafe_send::UnsafeSend;
use crate::gpu::camera::*;
use crate::gpu::command_list::*;
use crate::gpu::image::*;
use crate::gpu::material::*;
use crate::gpu::mesh::*;
use crate::gpu::renderer::*;
use crate::gpu::renderer_types::*;
use crate::gpu::resource::*;
use crate::gpu::uniforms::*;
use crate::gpu::vulkan::semaphore::SemaphoreResource;

struct FrameInFlight {
    image_ready_semaphore: Rc<Semaphore>,
    last_semaphore: Option<Rc<Semaphore>>,  // last semaphore used
    last_submit_fence: Option<Rc<Fence>>, // fence of the last command buffer submitted
    resources: std::vec::Vec<Rc<RefCell<dyn Resource>>>,
    command_lists: Vec<Box<dyn CommandList>>,
    image_index: u32,
}

impl FrameInFlight {
    pub fn new(
        a_logical_device: Rc<LogicalDevice>,
    ) -> FrameInFlight {
        Self {
            image_ready_semaphore: Rc::new(match Semaphore::new(a_logical_device.clone()){
                Ok(res) => res,
                Err(_res) => panic!("Unable to create vulkan semaphore"),
            }),
            last_semaphore: None,
            last_submit_fence: None,
            resources: Vec::new(),
            command_lists: Vec::new(),
            image_index: 0,
        }
    }
}

pub struct RendererVulkan {
    pub version_major: i32,
    pub version_minor: i32,
    pub version_patch: i32,

    clear_color: Vec4,
    clear_depth: f32,
    clear_stencil: i32,

    renderer_ready: bool,

    window: Arc<Mutex<UnsafeSend<sdl2::video::Window>>>,

    // Order matters here so that instance is destroyed last
    framebuffer_format: ash::vk::SurfaceFormatKHR,
    frames_in_flight: std::collections::VecDeque<FrameInFlight>,
    current_frame: FrameInFlight,
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
    fn name(&self) -> String {
        String::from("Vulkan")
    }

    fn get_type(&self) -> RendererType {
        RendererType::Vulkan
    }

    fn begin_frame(&mut self, _clear: RendererClearType) {
        self.renderer_ready = false;

        while (self.frames_in_flight.len() as u32) >= Self::MAX_FRAMES {
            let mut front = self.frames_in_flight.pop_front().expect("missing value in option");

            if let Some(fence) = front.last_submit_fence
            {
                let fences = [ fence.as_ref().fence ];
                unsafe {
                    self.logical_device
                        .device
                        .wait_for_fences(&fences, true, u64::MAX)
                        .expect("Failed to wait for fences");

                    // fence reset happens in release_command_list
                }
            }
            for command_list in front.command_lists.drain(..)
            {
                let cmd: Box<CommandListVulkan> = command_list
                    .into_any()
                    .downcast::<CommandListVulkan>()
                    .unwrap_or_else(|_| panic!("Invalid cast from command list to vulkan"));
                self.command_pool.release_command_list(cmd);
            }
        }

        // need to attempt to acquire multiple times incase it fails and we need to recreate the swapchain
        let mut attempt = true;
        let mut num_attempts: i32 = 2;
        while num_attempts > 0
            && attempt
            && self.swapchain.extent.width > 0
            && self.swapchain.extent.height > 0
        {
            attempt = false;
            num_attempts = num_attempts - 1;

            let (image_index, _suboptimal) = unsafe {
                match self
                    .swapchain
                    .swapchain
                    .swapchain_loader
                    .acquire_next_image(
                        self.swapchain.swapchain.swapchain,
                        u64::MAX,
                        self.current_frame.image_ready_semaphore.semaphore,
                        ash::vk::Fence::null(),
                    ) {
                    Ok(res) => res,
                    Err(res)
                        if res == ash::vk::Result::ERROR_OUT_OF_DATE_KHR
                            || res == ash::vk::Result::SUBOPTIMAL_KHR =>
                    {
                        self.recreate_swapchain();
                        attempt = true;
                        (0, false)
                    }
                    Err(res) => {
                        println!("Error: acquire_next_image: {}", res);
                        return;
                    }
                }
            };

            self.current_frame.image_index = image_index;
        }

        if num_attempts < 0 {
            return;
        }

        if self.swapchain.extent.width > 0 && self.swapchain.extent.height > 0 {
            self.current_frame.last_semaphore = Some(self.current_frame.image_ready_semaphore.clone());

            self.renderer_ready = true;
        }
    }

    fn end_frame(&mut self) {
        let recreate;

        if self.renderer_ready
            && self.swapchain.extent.width > 0
            && self.swapchain.extent.height > 0
        {
            self.renderer_ready = false;

            let swapchains = [self.swapchain.swapchain.swapchain];
            let image_indices = [self.current_frame.image_index];

            let semaphore: Rc<Semaphore> = self.current_frame.last_semaphore.as_ref().expect("No semaphore found").clone();
            let wait_semaphores = [semaphore.semaphore];

            let present_info = ash::vk::PresentInfoKHR::default()
                .wait_semaphores(&wait_semaphores)
                .swapchains(&swapchains)
                .image_indices(&image_indices);

            let window_size = self.window.lock().unwrap().inner.size();
            let window_resize = self.swapchain.extent.width != window_size.0
                || self.swapchain.extent.height != window_size.1;
            // TODO handle result
            recreate = match unsafe {
                self.swapchain
                    .swapchain
                    .swapchain_loader
                    .queue_present(self.logical_device.queue, &present_info)
            } {
                Ok(_) => window_resize,
                Err(res)
                    if res == ash::vk::Result::ERROR_OUT_OF_DATE_KHR
                        || res == ash::vk::Result::SUBOPTIMAL_KHR =>
                {
                    true
                }
                Err(res) => {
                    println!("Error: queue_present {}", res);
                    false
                }
            };
            
            let finished = std::mem::replace(&mut self.current_frame, FrameInFlight::new(self.logical_device.clone()));
            self.frames_in_flight.push_back(finished);
        } else {
            self.renderer_ready = false;
            recreate = true;
        }

        if recreate {
            self.recreate_swapchain();
        }
    }

    //clear immediatly
    //= RendererClearColor | RendererClearDepth | RendererClearStencil
    fn clear(&mut self, _clear: RendererClearType) {}

    //Get and set clear values may be called before BeginFrame
    fn set_clear_color(&mut self, a_color: Vec4) {
        self.clear_color = a_color;
    }
    fn set_clear_depth(&mut self, a_depth: f32) {
        self.clear_depth = a_depth;
    }
    fn set_clear_stencil(&mut self, a_stencil: i32) {
        self.clear_stencil = a_stencil;
    }
    fn get_clear_color(&self) -> Vec4 {
        self.clear_color
    }
    fn get_clear_depth(&self) -> f32 {
        self.clear_depth
    }

    fn get_clear_stencil(&self) -> i32 {
        self.clear_stencil
    }

    fn set_viewport(&mut self, _pos: IVec2, _size: IVec2) {}

    fn get_viewport_pos(&self) -> IVec2 {
        IVec2::new(0, 0)
    }
    fn get_viewport_size(&self) -> IVec2 {
        IVec2::new(0, 0)
    }

    fn load_shader(
        &mut self,
        _shader_type: ShaderType,
        _source: &str,
    ) -> Result<Box<dyn Shader>, RendererError> {
        return Err(RendererError::Unimplemented);
    }

    fn load_shader_intermediate(
        &mut self,
        _shader_type: ShaderType,
        a_source: &std::vec::Vec<u8>,
    ) -> Result<Box<dyn Shader>, RendererError> {
        if a_source.len() % 4 != 0 {
            return Err(RendererError::Error);
        }

        let slice: &[u32] = unsafe {
            std::slice::from_raw_parts(a_source.as_ptr() as *const u32, a_source.len() / 4)
        };

        let create_info = ash::vk::ShaderModuleCreateInfo::default()
            .code(slice);

        let module = match unsafe {
            self.logical_device
                .device
                .create_shader_module(&create_info, None)
        } {
            Ok(res) => res,
            Err(_res) => return Err(RendererError::Error),
        };

        Ok(Box::new(ShaderVulkan {
            module: module,
            logical_device: self.logical_device.clone(),
        }))
    }

    fn load_program_vert_frag(
        &mut self,
        a_shader_vert: Box<dyn Shader>,
        a_shader_frag: Box<dyn Shader>,
    ) -> Result<Box<dyn Program>, RendererError> {
        let vertex_module = match a_shader_vert.any().downcast_ref::<ShaderVulkan>() {
            Some(res) => res,
            None => return Err(RendererError::InvalidCast),
        };

        let frag_module = match a_shader_frag.any().downcast_ref::<ShaderVulkan>() {
            Some(res) => res,
            None => return Err(RendererError::InvalidCast),
        };

        let main_function_name = CString::new("main").unwrap();

        let vertex_info = ash::vk::PipelineShaderStageCreateInfo::default()
            .stage(ash::vk::ShaderStageFlags::VERTEX)
            .module(vertex_module.module)
            .name(main_function_name.as_c_str());

        let frag_info = ash::vk::PipelineShaderStageCreateInfo::default()
            .stage(ash::vk::ShaderStageFlags::FRAGMENT)
            .module(frag_module.module)
            .name(main_function_name.as_c_str());

        let shader_stages = [vertex_info, frag_info];

        let vertex_state_info: vk::PipelineVertexInputStateCreateInfo =
            ash::vk::PipelineVertexInputStateCreateInfo::default();
                //.vertex_binding_descriptions(vertex_binding_descriptions)
                //.vertex_attribute_descriptions(vertex_attribute_descriptions)

        let vertex_assembly_info = ash::vk::PipelineInputAssemblyStateCreateInfo::default()
            .primitive_restart_enable(false)
            .topology(ash::vk::PrimitiveTopology::TRIANGLE_LIST);

        let viewports = [ash::vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: self.swapchain.extent.width as f32,
            height: self.swapchain.extent.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        }];

        let scissors = [vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: self.swapchain.extent,
        }];

        let viewport_state_info = ash::vk::PipelineViewportStateCreateInfo::default()
            .viewports(&viewports)
            .scissors(&scissors);

        let rasterization_state_info = ash::vk::PipelineRasterizationStateCreateInfo::default()
            .depth_clamp_enable(false)
            .cull_mode(ash::vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::CLOCKWISE)
            .line_width(1.0)
            .polygon_mode(ash::vk::PolygonMode::FILL)
            .rasterizer_discard_enable(false)
            .depth_bias_clamp(0.0)
            .depth_bias_constant_factor(0.0)
            .depth_bias_enable(false)
            .depth_bias_slope_factor(0.0);

        let multisample_state_create_info = ash::vk::PipelineMultisampleStateCreateInfo::default()
            .rasterization_samples(ash::vk::SampleCountFlags::TYPE_1)
            .sample_shading_enable(false)
            .min_sample_shading(0.0)
            .alpha_to_one_enable(false)
            .alpha_to_coverage_enable(false);

        let stencil_state = ash::vk::StencilOpState::default()
            .fail_op(ash::vk::StencilOp::KEEP)
            .pass_op(ash::vk::StencilOp::KEEP)
            .depth_fail_op(ash::vk::StencilOp::KEEP)
            .compare_op(ash::vk::CompareOp::ALWAYS)
            .compare_mask(0)
            .write_mask(0)
            .reference(0);

        let depth_state_create_info = ash::vk::PipelineDepthStencilStateCreateInfo::default()
            .depth_test_enable(false)
            .depth_write_enable(false)
            .depth_compare_op(ash::vk::CompareOp::LESS_OR_EQUAL)
            .depth_bounds_test_enable(false)
            .stencil_test_enable(false)
            .front(stencil_state)
            .back(stencil_state)
            .max_depth_bounds(1.0)
            .min_depth_bounds(0.0);

        let color_blend_attachment_states = [ash::vk::PipelineColorBlendAttachmentState::default()
            .blend_enable(false)
            .color_write_mask(ash::vk::ColorComponentFlags::RGBA)
            .src_color_blend_factor(ash::vk::BlendFactor::ONE)
            .dst_color_blend_factor(ash::vk::BlendFactor::ZERO)
            .color_blend_op(ash::vk::BlendOp::ADD)
            .src_alpha_blend_factor(ash::vk::BlendFactor::ONE)
            .dst_alpha_blend_factor(ash::vk::BlendFactor::ZERO)
            .alpha_blend_op(ash::vk::BlendOp::ADD)];

        let color_blend_state = ash::vk::PipelineColorBlendStateCreateInfo::default()
            .logic_op_enable(false)
            .logic_op(ash::vk::LogicOp::COPY)
            .attachments(&color_blend_attachment_states)
            .blend_constants([0.0, 0.0, 0.0, 0.0]);

        /*
            let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];

        let dynamic_state_info = ash::vk::PipelineDynamicStateCreateInfo::builder()
          .dynamic_states(&dynamic_states)
          .build();
        */

        let pipeline_layout_info = ash::vk::PipelineLayoutCreateInfo::default();

        let pipeline_layout = match unsafe {
            self.logical_device
                .device
                .create_pipeline_layout(&pipeline_layout_info, None)
        } {
            Ok(res) => res,
            Err(_res) => return Err(RendererError::ShaderCompile),
        };

        let create_info = ash::vk::GraphicsPipelineCreateInfo::default()
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
            .base_pipeline_index(-1);

        let pipeline_infos = [create_info];

        let graphics_pipelines = match unsafe {
            self.logical_device.device.create_graphics_pipelines(
                ash::vk::PipelineCache::null(),
                &pipeline_infos,
                None,
            )
        } {
            Ok(res) => res,
            Err(_res) => return Err(RendererError::ShaderCompile),
        };

        Ok(Box::new(ProgramVulkan {
            pipeline: graphics_pipelines[0],
            pipeline_layout: pipeline_layout,
            logical_device: self.logical_device.clone(),
        }))
    }

    fn get_uniform(
        &mut self,
        _shader: &mut Box<dyn Program>,
        a_name: &str,
    ) -> Box<dyn UniformShader> {
        Box::new(UniformShaderVulkan {
            name: UniformName::new(a_name),
            id: 0,
        })
    }

    //fn set_uniform(&mut self, a_uniform: &Box<dyn Uniform>){}

    //fn set_texture(&mut self, a_texture: &Box<dyn Texture>){}

    fn gen_command_list(&mut self) -> Box<dyn CommandList> {
        // TODO validate begin_render has been called first
        
        let command_list = match self.command_pool.get_command_list() {
            Some(cmd_lst) => cmd_lst,
            None => panic!("Unable to get vulkan command list"),
        };

        match unsafe {
            self.logical_device.device.reset_command_buffer(
                command_list.command_buffer,
                ash::vk::CommandBufferResetFlags::empty(),
            )
        } {
            Ok(_) => {}
            Err(res) => {
                println!("Error: reset_command_buffer {}", res)
            }
        };

        let begin_info = ash::vk::CommandBufferBeginInfo::default();

        match unsafe {
            self.logical_device
                .device
                .begin_command_buffer(command_list.command_buffer, &begin_info)
        } {
            Ok(_) => {}
            Err(res) => {
                println!("Error: begin_command_buffer {}", res)
            }
        };

        let clear_values = [ash::vk::ClearValue {
            color: ash::vk::ClearColorValue {
                float32: self.clear_color.to_array(),
            },
        }];
        let render_pass_info = ash::vk::RenderPassBeginInfo::default()
            .render_pass(self.render_pass.render_pass)
            .framebuffer(self.swapchain.framebuffers[self.current_frame.image_index as usize].framebuffer)
            .render_area(ash::vk::Rect2D {
                offset: ash::vk::Offset2D { x: 0, y: 0 },
                extent: self.swapchain.extent,
            })
            .clear_values(&clear_values);
        unsafe {
            self.logical_device.device.cmd_begin_render_pass(
                command_list.command_buffer,
                &render_pass_info,
                ash::vk::SubpassContents::INLINE,
            )
        };

        let viewports = [ash::vk::Viewport::default()
            .x(0.0)
            .y(0.0)
            .width(self.swapchain.extent.width as f32)
            .height(self.swapchain.extent.height as f32)
            .min_depth(0.0)
            .max_depth(0.0)];

        unsafe {
            self.logical_device.device.cmd_set_viewport(
                command_list.command_buffer,
                0,
                &viewports,
            )
        };

        let scissors = [ash::vk::Rect2D {
            offset: ash::vk::Offset2D { x: 0, y: 0 },
            extent: self.swapchain.extent,
        }];
        unsafe {
            self.logical_device.device.cmd_set_scissor(
                command_list.command_buffer,
                0,
                &scissors,
            )
        };

        command_list
    }

    fn submit_command_list(&mut self, a_command_list: Box<dyn CommandList>) {
        let mut command_list: Box<CommandListVulkan> = a_command_list
            .into_any()
            .downcast::<CommandListVulkan>()
            .unwrap_or_else(|_| panic!("Invalid cast from command list to vulkan"));

        unsafe {
            self.logical_device
                .device
                .cmd_end_render_pass(command_list.command_buffer)
        };

        // TODO handle result
        match unsafe {
            self.logical_device
                .device
                .end_command_buffer(command_list.command_buffer)
        } {
            Ok(_) => {}
            Err(res) => {
                println!("Error: end_command_buffer {}", res)
            }
        };

        let wait_semaphores = [self.current_frame.last_semaphore.as_ref().unwrap().semaphore];
        let wait_stages = [ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let signal_semaphores = [command_list.semaphore.semaphore];
        let command_buffers = [command_list.command_buffer];
        self.current_frame.last_semaphore = Some(command_list.semaphore.clone());

        let submit_info = ash::vk::SubmitInfo::default()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages)
            .signal_semaphores(&signal_semaphores)
            .command_buffers(&command_buffers);

        let submits = [submit_info];

        // TODO handle result
        match unsafe {
            self.logical_device.device.queue_submit(
                self.logical_device.queue,
                &submits,
                command_list.fence.fence,
            )
        } {
            Ok(_) => {}
            Err(res) => {
                println!("Error: queue_submit {}", res)
            }
        };

        command_list.is_submitted = true;

        // probably dont need to add this ad command list is stored
        //self.current_frame.resources.extend(command_list.resources.iter().cloned());
        //self.current_frame.resources.push(Rc::new(RefCell::new(SemaphoreResource{semaphore: command_list.semaphore.clone()})));
        self.current_frame.last_submit_fence = Some(command_list.fence.clone());

        self.current_frame.command_lists.push(command_list);

        //self.command_pool.release_command_list(command_list);
    }

    fn gen_buffer_vertex(&mut self, _verts: &std::vec::Vec<f32>) -> Box<dyn Vertices> {
        Box::new(VerticesVulkan { id: 0 })
    }

    fn gen_geometry(&mut self, _buffer: &Box<dyn Vertices>) -> Box<dyn Geometry> {
        Box::new(GeometryVulkan { id: 0 })
    }

    fn gen_mesh(
        &mut self,
        a_geometry: Box<dyn Geometry>,
        a_material: Box<dyn Material>,
    ) -> Rc<RefCell<Mesh>> {
        Rc::new(RefCell::new(Mesh {
            geometry: a_geometry,
            material: a_material,
        }))
    }

    fn gen_buffer_texture(&mut self) -> Box<dyn Texture> {
        Box::new(TextureVulkan { id: 0 })
    }

    fn gen_sampler(&mut self, _texture: Rc<dyn Texture>) -> Box<dyn Sampler> {
        Box::new(SamplerVulkan {
            name: String::from(""),
        })
    }

    fn load_texture(&mut self, _image: &image::DynamicImage, _texture: &mut Box<dyn Texture>) {}

    fn use_program(&mut self, _program: &Box<dyn Program>) {}

    fn draw_geometry(&mut self, _geometry: &Box<dyn Geometry>) {}
    fn draw_mesh(
        &mut self,
        _camera: &Camera,
        a_mesh: Rc<RefCell<Mesh>>,
        a_command_list: &mut Box<dyn CommandList>,
    ) {
        if !self.renderer_ready {
            return;
        }

        let command_list = match a_command_list.any_mut().downcast_mut::<CommandListVulkan>() {
            Some(res) => res,
            None => panic!("Invalid cast of CommandList to CommandListVulkan"),
        };

        // let geometry = match a_mesh.geometry.any().downcast_ref::<GeometryVulkan>() {
        //   Some(res) => res,
        //   None => return
        // };
        let mesh = a_mesh.borrow();
        let program_rc = mesh.material.get_program();
        let program = match program_rc.any().downcast_ref::<ProgramVulkan>() {
            Some(res) => res,
            None => panic!("Invalid cast of Program to ProgramVulkan"),
        };

        unsafe {
            // TODO Only set pipeline if not already set
            self.logical_device.device.cmd_bind_pipeline(
                command_list.command_buffer,
                ash::vk::PipelineBindPoint::GRAPHICS,
                program.pipeline,
            );

            self.logical_device
                .device
                .cmd_draw(command_list.command_buffer, 3, 1, 0, 0);
        }

        command_list.resources.push(a_mesh.clone());
    }

    fn read_render_buffer(&mut self) -> Image {
        return Image {
            width: 0,
            height: 0,
            pitch: 0,
            pixels: std::vec::Vec::<u8>::new(),
        };
    }
}

impl RendererVulkan {
    pub const MAX_FRAMES: u32 = 2;

    pub fn new(
        a_window: Arc<Mutex<UnsafeSend<sdl2::video::Window>>>,
        a_enable_validation_layers: bool,
    ) -> Result<Self, RendererError> {
        /*
        let extensions = match a_window.vulkan_instance_extensions(){
          Ok(res) => res,
          Err(_res) => return Err(RendererError::Error)
        };
        */

        let entry = unsafe {
            match Entry::load() {
                Ok(res) => res,
                Err(_res) => return Err(RendererError::Error),
            }
        };

        let instance = match Instance::new(&entry, a_enable_validation_layers) {
            Ok(res) => res,
            Err(_res) => return Err(RendererError::Error),
        };

        let (major, minor, patch) = unsafe {
            match entry.try_enumerate_instance_version().unwrap() {
                Some(v) => (
                    vk::api_version_major(v) as i32,
                    vk::api_version_minor(v) as i32,
                    vk::api_version_patch(v) as i32,
                ),
                None => (1, 0, 0),
            }
        };

        let surface = match Surface::new(&a_window.lock().unwrap().inner, &entry, &instance) {
            Ok(res) => res,
            Err(_res) => return Err(RendererError::Error),
        };

        let extensions = std::vec!["VK_KHR_swapchain"];

        let physical_device = match PhysicalDevice::new(&instance, &extensions) {
            Ok(res) => res,
            Err(_res) => return Err(RendererError::Error),
        };

        let logical_device = match LogicalDevice::new(&instance, &physical_device, &extensions) {
            Ok(res) => Rc::new(res),
            Err(_res) => return Err(RendererError::Error),
        };

        let format = match surface.pick_format(&physical_device) {
            Ok(res) => res,
            Err(_res) => return Err(RendererError::Error),
        };

        let render_pass = match RenderPass::new(logical_device.clone(), format.format) {
            Ok(res) => res,
            Err(_res) => return Err(RendererError::Error),
        };

        let window_size = a_window.lock().unwrap().inner.size();
        let extent = ash::vk::Extent2D {
            width: window_size.0,
            height: window_size.1,
        };

        let swapchain = match Swapchain::new(
            logical_device.clone(),
            &instance,
            &surface,
            &physical_device,
            &render_pass,
            &format,
            extent,
        ) {
            Ok(res) => res,
            Err(_res) => return Err(RendererError::Error),
        };

        let num_command_buffers = 64;

        let command_pool =
            match CommandPool::new(logical_device.clone(), physical_device.queue_family as u32, num_command_buffers) {
                Ok(res) => res,
                Err(_res) => return Err(RendererError::Error),
            };

        // let command_buffers = match command_pool.allocate_command_buffer(Self::MAX_FRAMES) {
        //     Ok(res) => res,
        //     Err(_res) => return Err(RendererError::Error),
        // };

        Ok(Self {
            version_major: major,
            version_minor: minor,
            version_patch: patch,
            clear_color: Vec4::new(0.0, 0.0, 0.0, 0.0),
            clear_depth: 1.0,
            clear_stencil: 0,
            current_frame: FrameInFlight::new(logical_device.clone()),
            renderer_ready: false,
            frames_in_flight: VecDeque::new(),
            window: a_window,
            framebuffer_format: format,
            command_pool: command_pool,
            swapchain: swapchain,
            render_pass: render_pass,
            logical_device: logical_device,
            physical_device: physical_device,
            surface: surface,
            instance: instance,
        })
    }

    fn recreate_swapchain(&mut self) -> Result<(), RendererError> {
        let window_size = self.window.lock().unwrap().inner.size();
        let extent = ash::vk::Extent2D {
            width: window_size.0,
            height: window_size.1,
        };

        match unsafe { self.logical_device.device.device_wait_idle() } {
            Ok(_) => {}
            Err(res) => println!("Error: device_wait_idle: {}", res),
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
            extent,
        ) {
            Ok(res) => res,
            Err(res) => {
                println!("Error: Swapchain::new: {}", res);
                return Err(RendererError::Error);
            }
        };
        return Ok(());
    }
}

impl Drop for RendererVulkan {
    fn drop(&mut self) {
        match unsafe { self.logical_device.device.device_wait_idle() } {
            Ok(_) => {}
            Err(res) => println!("Error: device_wait_idle: {}", res),
        };
    }
}
