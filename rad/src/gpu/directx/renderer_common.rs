use crate::gpu::renderer_types::*;

#[cfg(windows)]
use windows::Win32::Graphics::Dxgi::*;
use std::result::Result;

#[cfg(windows)]
fn get_default_adapter(a_dxgi_factory: &IDXGIFactory6) -> Result<IDXGIAdapter1, RendererError> {
  for i in 0.. {
    match unsafe {a_dxgi_factory.EnumAdapters1(i)}{
      Ok(res) => {
        let mut desc = Default::default();
        let desc_result = unsafe { res.GetDesc1(&mut desc) };
        if desc_result.is_err(){
          return Err(RendererError::Error)
        }

        if (DXGI_ADAPTER_FLAG(desc.Flags) & DXGI_ADAPTER_FLAG_SOFTWARE) != DXGI_ADAPTER_FLAG_NONE {
          continue;
        }
        return Ok(res)
      },
      Err(res) => match res.code() {
        windows::Win32::Foundation::E_NOINTERFACE => return Err(RendererError::UnsupportedAPI),
        _ => break
      }
    }
  }

  Err(RendererError::Error)
}

#[cfg(windows)]
fn get_default_adapter_by_gpu_preference(a_dxgi_factory: &IDXGIFactory6, a_gpu_preference: DXGI_GPU_PREFERENCE) -> Result<IDXGIAdapter1, RendererError> {
  for i in 0.. {
    match unsafe {a_dxgi_factory.EnumAdapterByGpuPreference::<IDXGIAdapter1>(i, a_gpu_preference)}{
      Ok(res) => {
        let mut desc = Default::default();
        let desc_result = unsafe { res.GetDesc1(&mut desc) };
        if desc_result.is_err(){
          return Err(RendererError::Error)
        }

        if (DXGI_ADAPTER_FLAG(desc.Flags) & DXGI_ADAPTER_FLAG_SOFTWARE) != DXGI_ADAPTER_FLAG_NONE {
          continue;
        }
        return Ok(res)
      },
      Err(res) => match res.code() {
        windows::Win32::Foundation::E_NOINTERFACE => return Err(RendererError::UnsupportedAPI),
        _ => break
      }
    }
  }

  Err(RendererError::Error)
}

//Note  Do not mix the use of DXGI 1.0 (IDXGIFactory) and DXGI 1.1 (IDXGIFactory1) in an application. Use IDXGIFactory or IDXGIFactory1, but not both in an application.
//CreateDXGIFactory 
//CreateDXGIFactory1 Windows 7 [desktop apps | UWP apps] Windows Server 2008 R2
//CreateDXGIFactory2 Windows 8.1 [desktop apps | UWP apps], Windows Server 2012 R2
//IDXGIFactory1 Windows 7 [desktop apps | UWP apps] Windows Server 2008 R2
//IDXGIFactory2 Windows 8 and Platform Update for Windows 7 [desktop apps | UWP apps] Windows Server 2012 and Platform Update for Windows Server 2008 R2
//IDXGIFactory3 Windows 8.1 [desktop apps only] Windows Server 2012 R2
//IDXGIFactory6 Windows 10, version 1803 [desktop apps only] Windows Server, version 1709
//IDXGIFactory7 Windows 10, version 1809 [desktop apps only] Windows Server, version 1709
//IDXGIAdapter1 Windows 7 [desktop apps | UWP apps] Windows Server 2008 R2
//IDXGIAdapter2 Windows 8 and Platform Update for Windows 7 [desktop apps | UWP apps] Windows Server 2012 and Platform Update for Windows Server 2008 R2
#[cfg(windows)]
pub fn get_adapter(a_factory: &IDXGIFactory6, a_device_type: DeviceType) -> Result<IDXGIAdapter1, RendererError>{
  match a_device_type{
    DeviceType::Default => return get_default_adapter(&a_factory),
    DeviceType::HighPerformance => return get_default_adapter_by_gpu_preference(&a_factory, DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE),
    DeviceType::LowPower => return get_default_adapter_by_gpu_preference(&a_factory, DXGI_GPU_PREFERENCE_MINIMUM_POWER)
  }
}

#[cfg(windows)]
pub fn get_factory() -> Result<IDXGIFactory6, RendererError>{
  let dxgi_factory_flags = if cfg!(debug_assertions) {
    DXGI_CREATE_FACTORY_DEBUG
  } 
  else {
    0
  };

  match unsafe { CreateDXGIFactory2::<IDXGIFactory6>(dxgi_factory_flags) }{
    Ok(res) => return Ok(res),
    Err(res) => {
      if res.code() == windows::Win32::Foundation::E_NOINTERFACE{
        return Err(RendererError::UnsupportedAPI)
      }
      else{
        return Err(RendererError::Error)
      }
    }
  }
}
