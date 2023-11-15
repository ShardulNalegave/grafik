
// ===== Imports =====
use grafik::prelude::*;
// ===================

#[tokio::main]
async fn main() {
  let mut app = Grafik::new(Default::default()).await.unwrap();
  app.set_on_quit(quit);
  app.run();
}

fn quit() {
  println!("Quitting");
}