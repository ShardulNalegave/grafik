
/// Prelude Module
pub mod prelude;

/// Events Module
pub mod events;

/// Windowing Module
pub mod windowing;

/// Renderer Module
pub mod renderer;

/// Utils module
pub mod utils;

// ===== Imports =====
use renderer::{
  RendererState,
  context::Context,
};
use windowing::WindowConfig;
use winit::{
  event_loop::EventLoop,
  window::{WindowBuilder, Window},
  dpi::PhysicalSize,
  event::{
    WindowEvent,
    Event
  }
};
// ===================

#[derive(Debug, Clone, Copy)]
pub enum GrafikError {
  CouldntCreateRenderer,
  CouldntBuildWindow,
}

/// # Grafik
/// The top-level struct to be used by end-users.
/// Provides all necessary APIs to the user.
pub struct Grafik {
  /// Window Event-Loop
  event_loop: EventLoop<()>,
  /// The Window Instance
  window: Window,
  /// Renderer State
  renderer_state: RendererState,
  /// On Quit event handler
  on_quit: Option<events::QuitHandler>,
  /// On Draw event handler
  on_draw: Option<events::DrawHandler>,
}

impl Grafik {

  /// # Constructor
  /// Constructs a new instance of a Grafik application.
  /// 
  /// ### Arguments
  /// * window_cfg => Application window config
  pub async fn new(window_cfg: WindowConfig) -> Result<Self, GrafikError> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
      .with_title(window_cfg.title)
      .with_inner_size(PhysicalSize::new(window_cfg.size.0, window_cfg.size.1))
      .with_resizable(window_cfg.resizable)
      .build(&event_loop)
      .map_err(|_| GrafikError::CouldntBuildWindow)?;

    let renderer_state = RendererState::new(&window)
      .await.map_err(|_| GrafikError::CouldntCreateRenderer)?;

    Ok(Self {
      event_loop,
      window,
      renderer_state,
      on_quit: None,
      on_draw: None,
    })
  }

  /// # Run
  /// Runs the application
  pub fn run(self) {
    let window = self.window;
    let event_loop = self.event_loop;
    let mut renderer_state = self.renderer_state;
    
    let on_quit = self.on_quit.unwrap_or(|| {});
    let on_draw = self.on_draw.unwrap_or(|_| {});
    
    let mut context = Context::default();

    event_loop.run(move |event, _, control_flow| {
      match event {
        Event::WindowEvent { event, .. } => match event {
          WindowEvent::CloseRequested => {
            on_quit();
            control_flow.set_exit();
          },
          WindowEvent::Resized(physical_size) => {
            renderer_state.resize((physical_size.width, physical_size.height));
          },
          WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
            renderer_state.resize((new_inner_size.width, new_inner_size.height));
          },
          _ => {},
        },
        Event::MainEventsCleared => window.request_redraw(),
        Event::RedrawRequested(_) => {
          on_draw(&mut context);
          renderer_state.render(&context);
        },
        _ => {}
      }
    });
  }

  /// # Set On-Quit
  /// Set on quit event handler.
  pub fn set_on_quit(&mut self, handler: events::QuitHandler) {
    self.on_quit = Some(handler);
  }

  /// # Set On-Draw
  /// Set on draw event handler.
  pub fn set_on_draw(&mut self, hahndler: events::DrawHandler) {
    self.on_draw = Some(hahndler);
  }
}