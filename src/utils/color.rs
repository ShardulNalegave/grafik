
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

impl Color {
  pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
    Self { r, g, b, a }
  }

  pub fn rgb(r: u8, g: u8, b: u8) -> Self {
    Self { r, g, b, a: 255 }
  }

  pub fn hex(mut hex: u32) -> Self {
    hex = hex & 0xFFFFFF;
    let r = ((hex >> 16) & 0xFF) as u8;
    let g = ((hex >> 8) & 0xFF) as u8;
    let b = (hex & 0xFF) as u8;

    Self { r, g, b, a: 255 }
  }
}

impl From<wgpu::Color> for Color {
  fn from(value: wgpu::Color) -> Self {
    Self {
      r: (value.r * 255.0) as u8,
      g: (value.g * 255.0) as u8,
      b: (value.b * 255.0) as u8,
      a: (value.a * 255.0) as u8,
    }
  }
}

impl Into<wgpu::Color> for Color {
  fn into(self) -> wgpu::Color {
    wgpu::Color {
      r: (self.r as f64) / 255.0,
      g: (self.g as f64) / 255.0,
      b: (self.b as f64) / 255.0,
      a: (self.a as f64) / 255.0,
    }
  }
}