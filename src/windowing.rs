
pub struct WindowConfig {
  pub title: String,
  pub size: (u32, u32),
  pub resizable: bool,
}

impl Default for WindowConfig {
  fn default() -> Self {
    Self {
      title: "Hello, Grafik!".to_string(),
      size: (800, 600),
      resizable: false,
    }
  }
}