
pub mod config;

// ===== Imports =====
use winit::{
  event_loop::{
    EventLoop,
    ControlFlow
  },
  window::WindowBuilder,
  dpi::PhysicalSize,
  event::{
    WindowEvent,
    Event
  }
};

use crate::events;
// ===================

pub struct Window {
  win: winit::window::Window,
  event_loop: EventLoop<()>,
}

impl Window {
  pub fn new(cfg: config::WindowConfig) -> Self {
    let event_loop = EventLoop::new().unwrap();
    let win = WindowBuilder::new()
      .with_title(cfg.title)
      .with_inner_size(PhysicalSize::new(cfg.size.0, cfg.size.1))
      .with_resizable(cfg.resizable)
      .build(&event_loop)
      .unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    Self {
      win,
      event_loop,
    }
  }

  pub fn run(self, event_dispatcher: events::EventDispatcher) {
    let win = self.win;
    let event_loop = self.event_loop;

    event_loop.run(move |event, elwt| match event {
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::CloseRequested => {
          event_dispatcher.dispatch(events::Event::Quit);
          elwt.exit();
        },
        WindowEvent::RedrawRequested => {
          event_dispatcher.dispatch(events::Event::Draw);
        },
        _ => {},
      },
      Event::AboutToWait => win.request_redraw(),
      _ => {}
    }).unwrap();
  }
}