
// ===== Imports =====
use std::collections::HashMap;
// ===================

pub(crate) type EventHandler = fn();

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Event {
  Draw,
  Quit,
}

pub(crate) struct EventDispatcher {
  subs: HashMap<Event, Vec<EventHandler>>,
}

impl EventDispatcher {
  pub(crate) fn new() -> Self {
    let mut subs = HashMap::new();
    subs.insert(Event::Draw, vec![]);
    subs.insert(Event::Quit, vec![]);

    Self { subs }
  }

  pub(crate) fn subscribe(&mut self, event: Event, handler: EventHandler) {
    self.subs.get_mut(&event).unwrap().push(handler);
  }

  pub(crate) fn dispatch(&self, event: Event) {
    for handler in self.subs.get(&event).unwrap() {
      (*handler)();
    }
  }
}