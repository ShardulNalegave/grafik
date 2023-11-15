
/// # Window Config
/// Config options for application window
pub struct WindowConfig {
  /// Window title
  pub title: String,
  /// Window size
  pub size: (u32, u32),
  /// Is the window resizable?
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