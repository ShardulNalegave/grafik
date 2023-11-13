
/// Prelude Module
pub mod prelude;

/// Events Module
pub mod events;

/// Windowing Module
pub mod windowing;

/// Renderer Module
pub mod renderer;

// ===== Imports =====
use events::EventDispatcher;
use renderer::RendererState;
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
}

/// # Grafik
/// The top-level struct to be used by end-users.
/// Provides all necessary APIs to the user.
pub struct Grafik {
  /// Window Event-Loop
  event_loop: EventLoop<()>,
  /// The Window Instance
  window: Window,
  /// Event-Dispatcher for the Application
  event_dispatcher: EventDispatcher,
  /// Renderer State
  renderer_state: RendererState,
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
      .unwrap();

    let renderer_state = RendererState::new(&window)
      .await.map_err(|_| GrafikError::CouldntCreateRenderer)?;

    Ok(Self {
      event_loop,
      window,
      event_dispatcher: EventDispatcher::new(),
      renderer_state,
    })
  }

  /// # Run
  /// Runs the application
  pub fn run(self) {
    let window = self.window;
    let event_loop = self.event_loop;
    let event_dispatcher = self.event_dispatcher;
    let mut renderer_state = self.renderer_state;

    event_loop.run(move |event, _, control_flow| {
      match event {
        Event::WindowEvent { event, .. } => match event {
          WindowEvent::CloseRequested => {
            event_dispatcher.dispatch(events::Event::Quit);
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
          event_dispatcher.dispatch(events::Event::Draw);
          renderer_state.render();
        },
        _ => {}
      }
    });
  }

  /// # Subscribe To
  /// Subscribes to the specified event with provided handler.
  /// 
  /// ### Arguments
  /// * event => Event to subscribe
  /// * handler => Event handler
  pub fn subscribe_to(&mut self, event: events::Event, handler: events::EventHandler) {
    self.event_dispatcher.subscribe(event, handler);
  }
}