extern crate sdl2;

use crate::gpu::renderer::*;
use crate::gpu::renderer_types::*;
use crate::gpu::material::*;
use crate::gpu::camera::*;
use crate::gpu::uniforms::*;
use crate::gpu::directx::renderer_common::*;

use std::result::Result;
use std::rc::Rc;
use std::vec::Vec;

use glam::*;

#[cfg(windows)]
use windows::core::*;
#[cfg(windows)]
use windows::Win32::Graphics::Dxgi::*;
#[cfg(windows)]
use windows::Win32::Graphics::Dxgi::Common::*;
#[cfg(windows)]
use windows::Win32::Graphics::Direct3D::*;
#[cfg(windows)]
use windows::Win32::Graphics::Direct3D12::*;

pub struct VerticesDirectX12 {
}

impl Vertices for VerticesDirectX12 {
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

#[allow(dead_code)]
pub struct UniformDirectX12 {
  name: UniformName,
  data: UniformData,
  modified: bool,
}

#[allow(dead_code)]
impl UniformDirectX12 {
  pub fn new<T: 'static + GetType>(a_name: &str, a_data: T) -> UniformDirectX12{
    UniformDirectX12{
      name: UniformName::new(a_name), 
      data: UniformData::new::<T>(a_data),
      modified: true
    }
  }
}

#[allow(dead_code)]
impl Uniform for UniformDirectX12 {
  fn any(&mut self) -> &mut dyn std::any::Any{
    self
  }

  fn set_f32(&mut self, a: f32){
    self.data.set::<f32>(a);
  }

  fn get_f32(&self) -> f32{
    self.data.get::<f32>()
  }
  
  fn get_name(&self) -> &str{
    &self.name.get_name()
  }
  
  fn set_name(&mut self, a_name: &str){
    self.name.set_name(a_name);
  }
}


#[allow(dead_code)]
pub struct UniformShaderDirectX12 {
  name: UniformName,
}

impl UniformShader for UniformShaderDirectX12 {
  fn any(&mut self) -> &mut dyn std::any::Any{
    self
  }
}

pub struct GeometryDirectX12 {
}

impl Geometry for GeometryDirectX12 {
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

#[allow(dead_code)]
pub struct TextureDirectX12 {
  width: u32,
  height: u32
}

impl Texture for TextureDirectX12 {
  fn any(&self) -> &dyn std::any::Any{
    self
  }
}

pub struct SamplerDirectX12{
  name: String,
  texture: Rc<dyn Texture>,
}

impl Sampler for SamplerDirectX12 {
  fn any(&self) -> &dyn std::any::Any{
    self
  }

  fn set_name(&mut self, a_name: &str){
    self.name = String::from(a_name);
  }
}

pub struct RendererDirectX12 {
  #[cfg(windows)]
  device: ID3D12Device,
  #[cfg(windows)]
  swap_chain: IDXGISwapChain3,

  clear_color: Vec4,
  clear_depth: f32,
  clear_stencil: i32,

  viewport_pos: IVec2,
  viewport_size: IVec2,
}

#[allow(dead_code)]
impl Renderer for RendererDirectX12 {
  fn name(&self) -> String{
    String::from("DirectX12")
  }

  fn get_type(&self) -> RendererType{
    RendererType::DirectX
  }

  fn begin_frame(&mut self, _a_clear: RendererClearType){

  }

  fn end_frame(&mut self){

  }

  fn clear(&mut self, _a_clear: RendererClearType){

  }

  fn set_clear_color(&mut self, _a_color: Vec4){

  }

  fn set_clear_depth(&mut self, _a_depth: f32){

  }
  fn set_clear_stencil(&mut self, _a_stencil: i32){

  }

  fn get_clear_color(&self) -> Vec4{
    return self.clear_color
  }

  fn get_clear_depth(&self) -> f32{
    return self.clear_depth
  }

  fn get_clear_stencil(&self) -> i32{
    return self.clear_stencil
  }

  fn set_viewport(&mut self, _a_pos: IVec2, _a_size: IVec2){

  }

  fn get_viewport_pos(&self) -> IVec2{
    return self.viewport_pos
  }

  fn get_viewport_size(&self) -> IVec2{
    return self.viewport_size
  }
  
  fn load_shader(&mut self, _a_shader_type: ShaderType, _a_source: &str) -> Result<Box<dyn Shader>, RendererError>{
    return Err(RendererError::Unimplemented)
  }

  fn load_program_vert_frag(&mut self, _a_shader_vert: Box<dyn Shader>, _a_shader_frag: Box<dyn Shader>) -> Result<Box<dyn Program>, RendererError>{
    return Err(RendererError::Unimplemented)
  }

  fn get_uniform(&mut self, a_shader: &mut Box<dyn Program>, a_name: &str) -> Box<dyn UniformShader>{
    Box::new(UniformShaderDirectX12{
      name: UniformName::new(a_name)
    })
  }

  fn gen_buffer_vertex(&mut self, _a_verts: &std::vec::Vec<f32>) -> Box<dyn Vertices>{
    Box::new(VerticesDirectX12{})
  }

  fn gen_geometry(&mut self, _a_buffer: &Box<dyn Vertices>) -> Box<dyn Geometry>{
    Box::new(GeometryDirectX12{})
  }

  fn gen_mesh(&mut self, a_geometry: Box<dyn Geometry>, a_material: Box<dyn Material>) -> Box<Mesh>{
    Box::new(Mesh{
      geometry: a_geometry,
      material: a_material
      })
  }

  fn gen_buffer_texture(&mut self) -> Box<dyn Texture>{
    Box::new(TextureDirectX12{
      width: 0,
      height: 0})
  }

  fn gen_sampler(&mut self, a_texture: Rc<dyn Texture>) -> Box<dyn Sampler>{
    Box::new(SamplerDirectX12{name: String::from(""), texture: a_texture})
  }

  fn load_texture(&mut self, _a_image: &image::DynamicImage, a_texture: &mut Box<dyn Texture>){

  }

  fn use_program(&mut self, _a_program: &Box<dyn Program>){

  }

  fn draw_geometry(&mut self, _a_geometry: &Box<dyn Geometry>){

  }

  fn draw_mesh(&mut self, _a_camera: &Camera, _a_mesh: &mut Box<Mesh>){

  }
}

fn print_type_of<T>(_: &T) {
  println!("{}", std::any::type_name::<T>())
}

#[allow(dead_code)]
impl RendererDirectX12 {
  #[cfg(not(windows))]
  pub fn new(a_video_subsystem: &sdl2::VideoSubsystem, a_window: &sdl2::video::Window) -> Result<Self, RendererError>{
    Err(RendererError::UnsupportedAPI)
  }

  #[cfg(windows)]
  pub fn new(a_video_subsystem: &sdl2::VideoSubsystem, a_window: &sdl2::video::Window) -> Result<Self, RendererError>{

    let factory = match get_factory(){
      Ok(res) => res,
      Err(res) => return Err(res)
    };

    let device: ID3D12Device = match RendererDirectX12::create_device(&factory){
      Ok(res) => res,
      Err(res) => return Err(res)
    };

    let mut info: sdl2::sys::SDL_SysWMinfo = unsafe {std::mem::zeroed()};
    //sdl2::sys::SDL_VERSION(&mut info.version);
    
    info.version.major = sdl2::version::version().major;
    info.version.minor = sdl2::version::version().minor;
    info.version.patch = sdl2::version::version().patch;
    unsafe {sdl2::sys::SDL_GetWindowWMInfo(a_window.raw() as *mut sdl2::sys::SDL_Window, &mut info)};

    // rust sdl package is missing win in SDL_SysWMinfo
    // bindgen supposedly adds it but sdl does not then compile
    let inner_info = unsafe{info.info.dummy};
    let hwnd_ptr = inner_info.as_ptr() as *const windows::Win32::Foundation::HWND;
    let hwnd = unsafe{std::ptr::read_unaligned(hwnd_ptr)};

    let frame_buffer_count = 2;

    let swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
      BufferDesc: Common::DXGI_MODE_DESC{
        Width: a_window.size().0,
        Height: a_window.size().1,
        //todo
        RefreshRate: DXGI_RATIONAL{
          Numerator: 0,
          Denominator: 1,
        },
        Format: DXGI_FORMAT_R8G8B8A8_UNORM,
        ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
        Scaling: DXGI_MODE_SCALING_UNSPECIFIED,
      },
      //multisampling
      SampleDesc: Common::DXGI_SAMPLE_DESC{
        Count: 1,
        Quality: 0,
      },
      BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
      BufferCount: frame_buffer_count,
      OutputWindow: hwnd,
      Windowed: true.into(),  // TODO 
      SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
      Flags: 0,
    };

    let command_queue_desc= D3D12_COMMAND_QUEUE_DESC{
      Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
      Priority: D3D12_COMMAND_QUEUE_PRIORITY_NORMAL.0,
      Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
      NodeMask: 0,
    };

    let queue = match unsafe {device.CreateCommandQueue::<ID3D12CommandQueue>(&command_queue_desc)}{
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    };

    let mut swap_chain: Option<IDXGISwapChain> = None;
    let result_swap_chain =  unsafe{factory.CreateSwapChain(&queue, &swap_chain_desc, &mut swap_chain)};

    if result_swap_chain.is_err() {
      match result_swap_chain{
        DXGI_ERROR_INVALID_CALL => {
          print!("DXGI_ERROR_INVALID_CALL");
        },
        DXGI_STATUS_OCCLUDED => {
          print!("DXGI_STATUS_OCCLUDED");
        },
        E_OUTOFMEMORY => {
          print!("E_OUTOFMEMORY");
        },
        DXGI_ERROR_NOT_CURRENTLY_AVAILABLE => {
          print!("DXGI_ERROR_NOT_CURRENTLY_AVAILABLE");
        },
        _ => {}
      };

      return Err(RendererError::Error)
    }

    let swap_chain3 = match swap_chain.unwrap().cast::<IDXGISwapChain3>(){
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    };

    // Initialize the render target view heap description for the two back buffers.
    let render_target_view_heap_desc = D3D12_DESCRIPTOR_HEAP_DESC{
      Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
      NumDescriptors: 2,
      Flags: D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
      NodeMask: 0
    };

    // Create the render target view heap for the back buffers.
    let description_heap = match unsafe { device.CreateDescriptorHeap::<ID3D12DescriptorHeap>(&render_target_view_heap_desc)}{
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    };

    // Get a handle to the starting memory location in the render target view heap to identify where the render target views will be located for the two back buffers.
    let mut render_target_view_handle: D3D12_CPU_DESCRIPTOR_HANDLE = unsafe { description_heap.GetCPUDescriptorHandleForHeapStart()};

    // Get the size of the memory location for the render target view descriptors.
    let render_target_view_descriptor_size = unsafe {device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV)};

    let mut back_buffers = Vec::new();

    for i in 0..frame_buffer_count{
      // Get a pointer to the first back buffer from the swap chain.
      let back_buffer = match unsafe {swap_chain3.GetBuffer::<ID3D12Resource>(i)}{
        Ok(res) => res,
        Err(_res) => return Err(RendererError::Error)
      };

      // Create a render target view for the first back buffer.
      unsafe{device.CreateRenderTargetView(&back_buffer, None, render_target_view_handle)};

      // Increment the view handle to the next descriptor location in the render target view heap.
      render_target_view_handle.ptr += render_target_view_descriptor_size as usize;

      back_buffers.push(back_buffer);
    }

    let buffer_index = unsafe{ swap_chain3.GetCurrentBackBufferIndex() };

    let command_allocator = match unsafe{ device.CreateCommandAllocator::<ID3D12CommandAllocator>(D3D12_COMMAND_LIST_TYPE_DIRECT)}{
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    };

    /*
    let pipeline_state_description = D3D12_GRAPHICS_PIPELINE_STATE_DESC::default(); 

    let pipeline_initial_state = match unsafe{ device.CreateGraphicsPipelineState::<ID3D12PipelineState>(&pipeline_state_description)}{
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    };
    */


    // Create a basic command list.
    let command_list: ID3D12GraphicsCommandList = match unsafe {device.CreateCommandList //::<ID3D12CommandAllocator,ID3D12PipelineState,ID3D12GraphicsCommandList>
      (0, D3D12_COMMAND_LIST_TYPE_DIRECT, &command_allocator, None)}{
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    };

    // Initially we need to close the command list during initialization as it is created in a recording state.
    let result = unsafe {command_list.Close()};
    if result.is_err(){
      return Err(RendererError::Error)
    }

    // Create a fence for GPU synchronization.
    let fence: ID3D12Fence = match unsafe {device.CreateFence(0, D3D12_FENCE_FLAG_NONE)}{
      Ok(res) => res,
      Err(_res) => return Err(RendererError::Error)
    };

    // Create an event object for the fence.
    // m_fenceEvent = CreateEventEx(NULL, FALSE, FALSE, EVENT_ALL_ACCESS);
    // if (m_fenceEvent == NULL)
    // {
    //   return false;
    // }

    // Initialize the starting fence value. 
    let fence_value = 1u64;


    Ok(Self {
      device: device,
      swap_chain: swap_chain3,
      clear_color: Vec4::new(0.0, 0.0, 0.0, 0.0),
      clear_depth: 1.0,
      clear_stencil: 0,
      viewport_pos: IVec2::new(0,0),
      viewport_size: IVec2::new(0,0),
    })
  }

  #[cfg(windows)]
  fn create_device(a_factory: &IDXGIFactory6) -> Result<ID3D12Device, RendererError>{
    let feature_level = D3D_FEATURE_LEVEL_12_1;
    let mut device: Option<ID3D12Device> = None;

    let adapter = match get_adapter(&a_factory, DeviceType::Default){
      Ok(res) => res,
      Err(res) => return Err(res)
    };

    match unsafe {D3D12CreateDevice(&adapter, feature_level, &mut device)}{
      Ok(res) => {},
      Err(_res) => return Err(RendererError::Error)
    };

    Ok(device.unwrap())
  }

}
