
// ===== Imports =====
use crate::utils::Color;
// ===================

#[derive(Clone)]
pub struct Context {
  pub background_color: Color,
}

impl Default for Context {
  fn default() -> Self {
    Self {
      background_color: Color::hex(0xFFFFFF),
    }
  }
}

impl Context {
  pub(crate) fn build_encoder(
    &self,
    view: wgpu::TextureView,
    device: &wgpu::Device,
  ) -> wgpu::CommandBuffer {
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
      label: Some("Command Encoder"),
    });

    {
      let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: &view,
          resolve_target: None,
            ops: wgpu::Operations {
              load: wgpu::LoadOp::Clear(self.background_color.into()),
              store: wgpu::StoreOp::Store,
          },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: Default::default(),
        occlusion_query_set: Default::default(),
      });
    }

    encoder.finish()
  }
}