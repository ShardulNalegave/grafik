
// ===== Imports =====
use std::collections::HashMap;
// ===================

type EventHandler = fn();

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Event {
  Draw,
  Quit,
}

pub struct EventDispatcher {
  subs: HashMap<Event, Vec<EventHandler>>,
}

impl EventDispatcher {
  pub fn new() -> Self {
    let mut subs = HashMap::new();
    subs.insert(Event::Draw, vec![]);
    subs.insert(Event::Quit, vec![]);

    Self { subs }
  }

  pub fn subscribe(&mut self, event: Event, handler: EventHandler) {
    self.subs.get_mut(&event).unwrap().push(handler);
  }

  pub fn dispatch(&self, event: Event) {
    for handler in self.subs.get(&event).unwrap() {
      (*handler)();
    }
  }
}