
// ===== Imports =====
use grafik::prelude::*;
// ===================

#[tokio::main]
async fn main() {
  let mut app = Grafik::new(Default::default()).await.unwrap();
  app.subscribe_to(Event::Quit, on_quit);
  app.run();
}

fn on_quit() {
  println!("Quitting");
}