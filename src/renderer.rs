
/// Render-Context module
pub mod context;

// ===== Imports =====
use winit::window::Window;

use crate::renderer::context::Context;
// ===================

/// # Renderer-Error
/// Errors that can happen when using the Renderer (i.e. RendererState)
#[derive(Debug, Clone, Copy)]
pub enum RendererError {
  // Thrown when not able to create `wgpu::Surface`
  CouldntCreateSurface,
  // Thrown when not able to acquire `wgpu::Adapter`
  CouldntGetAdapter,
  // Thrown when not able to acquire `wgpu::Device`
  CouldntGetDevice,
}

/// # Renderer State
/// Renderer-State or simply the Renderer is responsible for drawing to the screen.
/// It is basically and abstraction over wgpu structs to make things simpler.
pub(crate) struct RendererState {
  surface: wgpu::Surface,
  device: wgpu::Device,
  queue: wgpu::Queue,
  surface_cfg: wgpu::SurfaceConfiguration,
}

impl RendererState {
  /// # Constructor
  /// Constructs a new RendererState
  /// 
  /// ### Arguments
  /// * win => Target window
  pub(crate) async fn new(win: &Window) -> Result<Self, RendererError> {
    // Get target window size
    let size = win.inner_size();

    // Create `wgpu::Instance`. It doesn't need to be stored, can be disposed after use.
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
      backends: wgpu::Backends::all(),
      ..Default::default()
    });

    // Create a surface
    let surface = unsafe { instance.create_surface(&win) }
      .map_err(|_| RendererError::CouldntCreateSurface)?;

    // Get the adapter
    let adapter = {
      let res = instance.request_adapter(
        &wgpu::RequestAdapterOptions {
          power_preference: wgpu::PowerPreference::default(),
          compatible_surface: Some(&surface),
          force_fallback_adapter: false,
        },
      ).await;
      
      match res {
        Some(adapter) => adapter,
        None => return Err(RendererError::CouldntGetAdapter),
      }
    };

    // Get device and queue
    let (device, queue) = adapter.request_device(
      &wgpu::DeviceDescriptor {
        features: wgpu::Features::empty(),
        limits: wgpu::Limits::default(),
        label: None,
      },
      None,
    ).await.map_err(|_| RendererError::CouldntGetDevice)?;

    // Create a SurfaceConfiguration and configure our created surface
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

    Ok(Self {
      surface,
      surface_cfg: config,
      device,
      queue,
    })
  }

  /// # Resize
  /// Resizes the wgpu surface
  pub(crate) fn resize(&mut self, new_size: (u32, u32)) {
    if new_size.0 > 0 && new_size.1 > 0 {
      self.surface_cfg.width = new_size.0;
      self.surface_cfg.height = new_size.1;
      self.surface.configure(&self.device, &self.surface_cfg);
    }
  }

  /// # Render
  /// Draws to the screen/window
  pub(crate) fn render(&mut self, ctx: &Context) {
    let output = self.surface.get_current_texture().unwrap();
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    
    let buffer = ctx.build_encoder(view, &self.device);
    self.queue.submit(std::iter::once(buffer));

    output.present();
  }
}