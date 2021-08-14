use tetra::Context;
use tetra::graphics::{Rectangle, DrawParams, Color, get_device_info};
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::text::{Font, Text};
use tetra::math::Vec2;
use tetra::time::get_fps;

pub struct UiState {
  debug: Text,
  debug_bg: Mesh,
  show_debug: bool,
}

impl UiState {
  pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
    let mut debug = Text::new(
      get_debug_info(ctx),
      Font::vector(ctx, "./res/font.ttf", 16.0)?,
    );
    let bounds = debug.get_bounds(ctx).unwrap();

    Ok(Self {
      debug,
      debug_bg: Mesh::rectangle(
        ctx,
        ShapeStyle::Fill,
        Rectangle::new(0.0, 0.0, bounds.width + 32.0, bounds.height + 32.0),
      )?,
      show_debug: false,
    })
  }

  pub fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
    if self.show_debug {
      self.debug.set_content(get_debug_info(ctx));

      self
        .debug_bg
        .draw(ctx, DrawParams::new().color(Color::rgba8(0, 0, 0, 128)));
      self.debug.draw(ctx, Vec2::new(16.0, 0.0));
    }
    Ok(())
  }

  pub fn toggle_debug(&mut self) {
    self.show_debug = !self.show_debug;
  }
}

fn get_debug_info(ctx: &mut Context) -> String {
  format!(
    "
Version: {}
FPS: {:.0}
Renderer: {}",
    env!("CARGO_PKG_VERSION"),
    get_fps(ctx),
    get_device_info(ctx).renderer
  )
}
