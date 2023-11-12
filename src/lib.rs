
pub mod prelude;
pub mod events;
pub mod windowing;
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

pub struct Grafik {
  event_loop: EventLoop<()>,
  window: Window,
  event_dispatcher: EventDispatcher,
  renderer_state: RendererState,
}

impl Grafik {
  pub async fn new(win_cfg: WindowConfig) -> Self {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
      .with_title(win_cfg.title)
      .with_inner_size(PhysicalSize::new(win_cfg.size.0, win_cfg.size.1))
      .with_resizable(win_cfg.resizable)
      .build(&event_loop)
      .unwrap();
    let renderer_state = RendererState::new(&window).await;

    Self {
      event_loop,
      window,
      event_dispatcher: EventDispatcher::new(),
      renderer_state,
    }
  }

  pub fn run(self) {
    let win = self.window;
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
        Event::MainEventsCleared => win.request_redraw(),
        Event::RedrawRequested(_) => {
          event_dispatcher.dispatch(events::Event::Draw);
          renderer_state.render();
        },
        _ => {}
      }
    });
  }

  pub fn subscribe_to(&mut self, event: events::Event, handler: events::EventHandler) {
    self.event_dispatcher.subscribe(event, handler);
  }
}