
// ===== Imports =====
use grafik::prelude::*;
// ===================

fn main() {
  let window = Window::new(WindowConfig::default());
  let mut event_dispatcher = EventDispatcher::new();

  event_dispatcher.subscribe(Event::Quit, on_quit);

  window.run(event_dispatcher);
}

fn on_quit() {
  println!("Quitting");
}