
// ===== Imports =====
use crate::renderer::context::Context;
// ===================

pub type QuitHandler = fn();
pub type DrawHandler = fn(ctx: &mut Context);