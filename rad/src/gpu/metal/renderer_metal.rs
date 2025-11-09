use glam::*;
//use std::cell::RefCell;
use std::ffi::CString;
//use std::rc::Rc;
use std::sync::Arc;

#[cfg(target_vendor = "apple")]
use metal::foreign_types::ForeignType;
#[cfg(target_vendor = "apple")]
use metal::*;
#[cfg(target_vendor = "apple")]
use objc::msg_send;
#[cfg(target_vendor = "apple")]
use objc::sel;
#[cfg(target_vendor = "apple")]
use objc::sel_impl;

use super::command_list_metal::*;
use crate::gpu::camera::*;
use crate::gpu::command_list::*;
use crate::gpu::image::*;
use crate::gpu::material::*;
use crate::gpu::mesh::*;
use crate::gpu::renderer::*;
use crate::gpu::renderer_types::*;
use crate::gpu::uniforms::*;
use crate::gui::window::Window;

pub struct SamplerMetal {
    name: String,
    texture: Arc<dyn Texture>,
}

impl Sampler for SamplerMetal {
    fn any(&self) -> &dyn std::any::Any {
        self
    }

    fn set_name(&mut self, a_name: &str) {
        self.name = String::from(a_name);
    }
}

pub struct ProgramMetal {}

impl Program for ProgramMetal {
    fn any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_uniform(&self, a_name: &str, a_data: UniformData) -> Box<dyn Uniform> {
        let _c_str = match CString::new(a_name) {
            Ok(res) => res,
            Err(_res) => panic!("Invalid text cast"),
        };

        Box::new(UniformMetal {
            name: UniformName::new(a_name),
            data: a_data,
            modified: true,
        })
    }
}

pub struct ShaderMetal {}

impl Shader for ShaderMetal {
    fn any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct VerticesMetal {}

impl Vertices for VerticesMetal {
    fn any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct GeometryMetal {}

impl Geometry for GeometryMetal {
    fn any(&self) -> &dyn std::any::Any {
        self
    }
}

#[allow(dead_code)]
pub struct TextureMetal {
    width: u32,
    height: u32,
}

impl Texture for TextureMetal {
    fn any(&self) -> &dyn std::any::Any {
        self
    }
}

#[allow(dead_code)]
pub struct UniformMetal {
    name: UniformName,
    data: UniformData,
    modified: bool,
}

#[allow(dead_code)]
impl UniformMetal {
    pub fn new<T: 'static + GetType>(a_name: &str, a_data: T) -> UniformMetal {
        UniformMetal {
            name: UniformName::new(a_name),
            data: UniformData::new::<T>(a_data),
            modified: true,
        }
    }
}

#[allow(dead_code)]
impl Uniform for UniformMetal {
    fn any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn set_f32(&mut self, a: f32) {
        self.data.set::<f32>(a);
    }

    fn get_f32(&self) -> f32 {
        self.data.get::<f32>()
    }

    fn get_name(&self) -> &str {
        &self.name.get_name()
    }

    fn set_name(&mut self, a_name: &str) {
        self.name.set_name(a_name);
    }
}

#[allow(dead_code)]
pub struct UniformShaderMetal {
    name: UniformName,
}

impl UniformShader for UniformShaderMetal {
    fn any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct RendererMetal {
    window: Arc<Window>,

    #[cfg(target_vendor = "apple")]
    device: Device,
    #[cfg(target_vendor = "apple")]
    metal_view: *mut core::ffi::c_void, //TODO destroy with SDL_Metal_DestroyView
    //metal_layer: CAMetalLayer,
    clear_color: Vec4,
    clear_depth: f32,
    clear_stencil: i32,

    viewport_pos: IVec2,
    viewport_size: IVec2,
}

#[allow(dead_code)]
impl Renderer for RendererMetal {
    fn name(&self) -> String {
        String::from("Metal")
    }

    fn get_type(&self) -> RendererType {
        RendererType::Metal
    }

    fn begin_frame(&mut self, a_clear: RendererClearType) {
        self.clear(a_clear);
    }

    fn end_frame(&mut self) {}

    //clear immediatly
    fn clear(&mut self, _a_clear: RendererClearType) {}

    // Get and set clear values may be called before BeginFrame
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

    fn set_viewport(&mut self, a_pos: IVec2, a_size: IVec2) {
        self.viewport_pos = a_pos;
        self.viewport_size = a_size;
    }

    fn get_viewport_pos(&self) -> IVec2 {
        self.viewport_pos
    }
    fn get_viewport_size(&self) -> IVec2 {
        self.viewport_size
    }

    fn load_shader(
        &mut self,
        _a_shader_type: ShaderType,
        _a_source: &str,
    ) -> Result<Arc<dyn Shader>, RendererError> {
        Ok(Arc::new(ShaderMetal {}))
    }

    fn load_shader_intermediate(
        &mut self,
        _a_shader_type: ShaderType,
        _a_source: &std::vec::Vec<u8>,
    ) -> Result<Arc<dyn Shader>, RendererError> {
        return Err(RendererError::Unimplemented);
    }

    fn load_program_vert_frag(
        &mut self,
        _a_shader_vert: Arc<dyn Shader>,
        _a_shader_frag: Arc<dyn Shader>,
    ) -> Result<Arc<dyn Program>, RendererError> {
        Ok(Arc::new(ProgramMetal {}))
    }

    fn get_uniform(
        &mut self,
        _a_shader: &mut Arc<dyn Program>,
        a_name: &str,
    ) -> Box<dyn UniformShader> {
        Box::new(UniformShaderMetal {
            name: UniformName::new(a_name),
        })
    }

    fn gen_command_list(&mut self) -> Box<dyn CommandList> {
        CommandListMetal::new()
    }

    fn submit_command_list(&mut self, _a_command_list: Box<dyn CommandList>) {}

    fn gen_buffer_vertex(&mut self, _a_verts: &std::vec::Vec<f32>) -> Arc<dyn Vertices> {
        Arc::new(VerticesMetal {})
    }

    fn gen_geometry(&mut self, _a_buffer: Arc<dyn Vertices>) -> Arc<dyn Geometry> {
        Arc::new(GeometryMetal {})
    }

    fn gen_mesh(
        &mut self,
        a_geometry: Arc<dyn Geometry>,
        a_material: Arc<dyn Material>,
    ) -> Arc<Mesh> {
        Arc::new(Mesh {
            geometry: a_geometry,
            material: a_material,
        })
    }

    fn gen_buffer_texture(&mut self) -> Arc<dyn Texture> {
        Arc::new(TextureMetal {
            width: 0,
            height: 0,
        })
    }

    fn gen_sampler(&mut self, a_texture: Arc<dyn Texture>) -> Box<dyn Sampler> {
        let sampler = SamplerMetal {
            name: String::from(""),
            texture: a_texture,
        };

        Box::new(sampler)
    }

    fn load_texture(&mut self, _a_image: &image::DynamicImage, _a_texture: Arc<dyn Texture>) {}

    fn draw_mesh(&mut self, _camera: &Camera, _a_mesh: Arc<Mesh>, _a_command_list: &mut Box<dyn CommandList>) {}

    fn read_render_buffer(&mut self) -> Image {
        Image {
            width: self.window.width,
            height: self.window.height,
            pitch: self.window.height * 4,
            pixels: vec![0u8; (self.window.width * self.window.height * 4) as usize],
        }
    }
}

#[allow(dead_code)]
impl RendererMetal {
    #[cfg(not(target_vendor = "apple"))]
    pub fn new(
        _a_video_subsystem: &sdl3::VideoSubsystem,
        _a_window: Arc<Window>,
    ) -> Result<Self, RendererError> {
        Err(RendererError::UnsupportedAPI)
    }

    #[cfg(target_vendor = "apple")]
    pub fn new(
        a_video_subsystem: &sdl3::VideoSubsystem,
        a_window: Arc<Window>,
    ) -> Result<Self, RendererError> {
        let is_main_thread: bool = unsafe { objc::msg_send![objc::class!(NSThread), isMainThread] };
        if !is_main_thread {
            panic!("setDevice must be called on the main thread.");
        }

        let devices = Device::all();
        assert!(!devices.is_empty(), "No Metal devices found!");

        for (i, device) in devices.iter().enumerate() {
            println!("Device {}: {}", i, device.name());
        }

        let device = Device::system_default().expect("Failed to create Metal device");

        println!("Metal Device Name: {}", device.name());
        println!(
            "Metal Device Supports Feature Set: {:?}",
            device.supports_feature_set(metal::MTLFeatureSet::iOS_GPUFamily1_v1)
        );

        // TODO Enable device selection
        // let selected_device = devices
        //   .into_iter()
        //   .find(|device| !device.is_low_power())
        //   .unwrap_or_else(|| Device::system_default().expect("Failed to create Metal device"));

        // println!("Selected Device: {}", selected_device.name());

        let view = RendererMetal::create_metal_view(&a_window.window.lock().unwrap().inner);
        let layer = RendererMetal::get_metal_layer(view) as *mut objc::runtime::Object;

        let is_metal_layer: bool =
            unsafe { objc::msg_send![layer, isKindOfClass: objc::class!(CAMetalLayer)] };
        if !is_metal_layer {
            panic!("The provided layer is not a CAMetalLayer.");
        }

        println!("CAMetalLayer Pointer: {:?}", layer);

        // unsafe {
        //   let _: () = msg_send![layer, setDevice: device];
        //   let _: () = msg_send![layer, setPixelFormat: MTLPixelFormat::BGRA8Unorm];
        //   let _: () = msg_send![layer, setFramebufferOnly: true];
        // }

        RendererMetal::configure_metal_layer(
            layer as *mut core::ffi::c_void,
            device.as_ptr() as *mut objc::runtime::Object,
        );
        //RendererMetal::configure_metal_layer(layer as *mut core::ffi::c_void, &device);

        // let ns_view = RendererMetal::get_nsview(&a_window.window.lock().unwrap().inner) as id;

        // let metal_layer: CAMetalLayer = unsafe { msg_send![ns_view, layer] };

        // metal_layer.set_device(&device);
        // metal_layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        // metal_layer.set_framebuffer_only(true);

        Ok(Self {
            window: a_window,
            device: device,
            //metal_layer: metal_layer,
            metal_view: view,
            clear_color: Vec4::new(0.0, 0.0, 0.0, 0.0),
            clear_depth: 1.0,
            clear_stencil: 0,
            viewport_pos: IVec2::new(0, 0),
            viewport_size: IVec2::new(0, 0),
        })
    }

    #[cfg(target_vendor = "apple")]
    fn create_metal_view(window: &sdl3::video::Window) -> *mut core::ffi::c_void {
        unsafe {
            let raw_window = window.raw() as *mut sdl3::sys::SDL_Window;
            let metal_view = sdl3::sys::SDL_Metal_CreateView(raw_window);

            if metal_view.is_null() {
                panic!("Failed to create Metal view");
            }

            metal_view
        }
    }

    #[cfg(target_vendor = "apple")]
    fn get_metal_layer(metal_view: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe {
            let layer = sdl3::sys::SDL_Metal_GetLayer(metal_view);

            if layer.is_null() {
                panic!("Failed to retrieve CAMetalLayer");
            }

            layer
        }
    }

    #[cfg(target_vendor = "apple")]
    fn configure_metal_layer(
        layer: *mut core::ffi::c_void,
        device_ptr: *mut objc::runtime::Object, /*device: &metal::Device*/
    ) {
        let metal_layer: *mut objc::runtime::Object = layer as *mut objc::runtime::Object;

        unsafe {
            let _: () = msg_send![metal_layer, setDevice: device_ptr];
            let _: () = msg_send![metal_layer, setPixelFormat: MTLPixelFormat::BGRA8Unorm];
            let _: () = msg_send![metal_layer, setFramebufferOnly: true];
        }

        // unsafe {
        //   let _: () = msg_send![metal_layer, setDevice: device];
        //   let _: () = msg_send![metal_layer, setPixelFormat: MTLPixelFormat::BGRA8Unorm];
        //   let _: () = msg_send![metal_layer, setFramebufferOnly: true];
        // }
    }

    // attempt at getting view. can probably delete
    /*
    fn get_nsview(window: &sdl3::video::Window) -> *mut std::ffi::c_void {
        let mut wm_info: sdl3::sys::SDL_SysWMinfo = unsafe { std::mem::zeroed() };
        wm_info.version.major = sdl3::version::version().major;
        wm_info.version.minor = sdl3::version::version().minor;
        wm_info.version.patch = sdl3::version::version().patch;

        unsafe {
            if sdl3::sys::SDL_GetWindowWMInfo(
                window.raw() as *mut sdl3::sys::SDL_Window,
                &mut wm_info as *mut _,
            ) == sdl3::sys::SDL_bool::SDL_TRUE
            {
                // rust sdl package is missing apple and windows in SDL_SysWMinfo
                // bindgen supposedly adds it but sdl does not then compile
                match wm_info.subsystem {
                    sdl3::sys::SDL_SYSWM_TYPE::SDL_SYSWM_COCOA => {
                        // On macOS, the handle will be an NSView with a CAMetalLayer attached.
                        // wm_info.info.cocoa.window as *mut _
                        let inner_info = wm_info.info.dummy ;
                        let view_ptr = inner_info.as_ptr() as *const *mut std::ffi::c_void;
                        let view = std::ptr::read_unaligned(view_ptr) ;
                        if view.is_null() {
                            panic!("NSView pointer is null");
                        }
                        view
                    }
                    sdl3::sys::SDL_SYSWM_TYPE::SDL_SYSWM_UIKIT => {
                        // On iOS, the handle will be a UIView with a CAMetalLayer attached.
                        // wm_info.info.uikit.window as *mut _
                        let inner_info = wm_info.info.dummy;
                        let view_ptr = inner_info.as_ptr() as *const *mut std::ffi::c_void;
                        let view = std::ptr::read_unaligned(view_ptr);
                        view
                    }
                    _ => std::ptr::null_mut(),
                }
            } else {
                std::ptr::null_mut()
            }
        }
    }
    */

    pub fn update_uniform(&self, _a_uniform: &mut Box<dyn Uniform>) {}

    pub fn update_sampler(&self, _a_sampler: &Box<dyn Sampler>) {}
}

impl Drop for ShaderMetal {
    fn drop(&mut self) {}
}

impl Drop for ProgramMetal {
    fn drop(&mut self) {}
}

impl Drop for VerticesMetal {
    fn drop(&mut self) {}
}

impl Drop for TextureMetal {
    fn drop(&mut self) {}
}
