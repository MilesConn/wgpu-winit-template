use std::{ops::Deref, sync::Arc};

use parking_lot::Mutex;
use winit::window::Window;

use super::App;

pub struct GpuConfig {
    pub backends: wgpu::Backends,
    pub device_features: wgpu::Features,
    pub device_limits: wgpu::Limits,
}

impl Default for GpuConfig {
    fn default() -> Self {
        Self {
            backends: wgpu::Backends::all(),
            device_features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
            device_limits: if cfg!(target_arch = "wasm32") {
                wgpu::Limits::downlevel_webgl2_defaults()
            } else {
                wgpu::Limits::default()
            },
        }
    }
}

#[derive(Clone, Copy)]
struct SurfaceSize {
    height: u32,
    width: u32,
}

impl SurfaceSize {
    fn new(width: u32, height: u32) -> Self {
        SurfaceSize { height, width }
    }
}

impl Default for SurfaceSize {
    fn default() -> Self {
        SurfaceSize {
            height: 600,
            width: 800,
        }
    }
}

pub struct Gpu {
    pub instance: wgpu::Instance,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter: wgpu::Adapter,
    pub surface_description: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface<'static>,
    pub command_buffers: Mutex<Vec<wgpu::CommandBuffer>>,
    format: wgpu::TextureFormat,
    surface_size: Mutex<SurfaceSize>,
}

impl Gpu {
    pub(crate) async fn new(window: Arc<Window>, gpu_config: GpuConfig) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: gpu_config.backends,
            dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
            ..Default::default()
        });

        // Important: Request surface before adapter!
        // TODO: Shouldthis be clone?
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Invalid Graphics Backend!");

        let config = Self::default_config(&surface, &adapter, &window);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: gpu_config.device_features,
                    required_limits: gpu_config.device_limits.using_resolution(adapter.limits()),
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
            .expect("Cannot find device!");

        let adapter_info = adapter.get_info();
        log::info!("Using GPU: {}", adapter_info.name);
        log::info!("Using WGPU backend: {:?}", adapter_info.backend);

        let format = config.format;

        log::info!("Using Present mode: {:?}", config.present_mode);

        let gpu = Self {
            // config: Mutex::new(config),
            surface,
            instance,
            queue,
            device,
            adapter,
            surface_description: config,
            command_buffers: Mutex::new(Default::default()),
            format,

            // These get initialized below
            surface_size: Mutex::new(SurfaceSize::default()),
        };

        // gpu.resume(&window);

        gpu
    }

    pub(crate) fn compute_surface_size(window: &Window) -> SurfaceSize {
        let window_size = window.inner_size();
        let width = window_size.width.max(1);
        let height = window_size.height.max(1);
        SurfaceSize { width, height }
    }

    pub(crate) fn default_config(
        surface: &wgpu::Surface,
        adapter: &wgpu::Adapter,
        window: &Window,
    ) -> wgpu::SurfaceConfiguration {
        let surface_size = Self::compute_surface_size(window);
        surface
            .get_default_config(adapter, surface_size.width, surface_size.height)
            .expect("Surface isn't supported by the adapter.")
    }

    // pub(crate) fn resume(&self, window: &Window) {
    //     let config = Self::default_config(&self.surface, &self.adapter, window);
    //     self.surface.configure(&self.device, &config);
    //     *self.surface_size.lock() = SurfaceSize::new(config.width, config.height);
    //     self.config = config;
    // }

    /// Resize the surface, making sure to not resize to zero.
    // pub(crate) fn resize(&self, size: SurfaceSize) {
    //     let mut config = self.config.lock();
    //     config.width = size.width.max(1);
    //     config.height = size.height.max(1);
    //     *self.surface_size.lock() = SurfaceSize {
    //         width: config.width,
    //         height: config.height,
    //     };
    //     self.surface.configure(&self.device, &config);
    // }

    // pub(crate) fn start_frame(&self, gpu: &Gpu) -> () {
    //     let config = self.config.lock();
    //     let surface_texture = match self.surface.get_current_texture() {
    //         Ok(frame) => frame,
    //         // If we timed out, just try again
    //         Err(wgpu::SurfaceError::Timeout) => self.surface
    //             .get_current_texture()
    //             .expect("Failed to acquire next surface texture!"),
    //         Err(
    //             // If the surface is outdated, or was lost, reconfigure it.
    //             wgpu::SurfaceError::Outdated
    //             | wgpu::SurfaceError::Lost
    //             // If OutOfMemory happens, reconfiguring may not help, but we might as well try
    //             | wgpu::SurfaceError::OutOfMemory,
    //         ) => {
    //             self.surface.configure(&gpu.device, &config);
    //             self.surface
    //                 .get_current_texture()
    //                 .expect("Failed to acquire next surface texture!")
    //         }
    //     };

    //     todo!()
    //     // return SurfaceRenderTarget {
    //     //     target_view: surface_texture.texture.create_view(&Default::default()),
    //     //     surface_texture,
    //     // };
    // }

    pub fn block(&self, handle: wgpu::SubmissionIndex) {
        self.device
            .poll(wgpu::MaintainBase::WaitForSubmissionIndex(handle));
    }

    pub fn submit(&self) -> wgpu::SubmissionIndex {
        let mut command_buffers = self.command_buffers.lock();
        let command_buffers = std::mem::take(&mut *command_buffers);
        self.queue.submit(command_buffers)
    }

    pub(super) fn paint(&mut self, state: &mut App) {
        // perform the rendering
    }
}
