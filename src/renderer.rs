
// ===== Imports =====
use winit::window::Window;
// ===================

pub(crate) struct RendererState {
  surface: wgpu::Surface,
  device: wgpu::Device,
  queue: wgpu::Queue,
  surface_cfg: wgpu::SurfaceConfiguration,
}

impl RendererState {
  pub(crate) async fn new(win: &Window) -> Self {
    let size = win.inner_size();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::all(),
      dx12_shader_compiler: Default::default(),
      flags: Default::default(),
      gles_minor_version: Default::default(),
    });
    let surface = unsafe { instance.create_surface(&win) }.unwrap();

    let adapter = instance.request_adapter(
      &wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
      },
    ).await.unwrap();

    let (device, queue) = adapter.request_device(
      &wgpu::DeviceDescriptor {
        features: wgpu::Features::empty(),
        limits: wgpu::Limits::default(),
        label: None,
      },
      None,
    ).await.unwrap();

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps.formats.iter()
      .copied()
      .find(|f| f.is_srgb())            
      .unwrap_or(surface_caps.formats[0]);
    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width: size.width,
      height: size.height,
      present_mode: surface_caps.present_modes[0],
      alpha_mode: surface_caps.alpha_modes[0],
      view_formats: vec![],
    };
    surface.configure(&device, &config);

    Self {
      surface,
      surface_cfg: config,
      device,
      queue,
    }
  }

  pub(crate) fn resize(&mut self, new_size: (u32, u32)) {
    if new_size.0 > 0 && new_size.1 > 0 {
      self.surface_cfg.width = new_size.0;
      self.surface_cfg.height = new_size.1;
      self.surface.configure(&self.device, &self.surface_cfg);
    }
  }

  pub(crate) fn render(&mut self) {
    //
  }
}