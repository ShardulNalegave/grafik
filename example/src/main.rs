
// ===== Imports =====
use grafik::prelude::*;
// ===================

#[tokio::main]
async fn main() {
  let mut app = Grafik::new(Default::default()).await.unwrap();
  app.set_on_draw(draw);
  app.set_on_quit(quit);
  app.run();
}

fn quit() {
  println!("Quitting");
}

fn draw(ctx: &mut Context) {
  ctx.background_color = Color::hex(0x000000);
}