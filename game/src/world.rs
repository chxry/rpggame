use tetra::{Context};
use tetra::graphics::Texture;
use tetra::math::Vec2;

use map::{Map, draw_layer};
use crate::SCALE;

pub struct World {
  tilemap: Texture,
  map: Map,
}

impl World {
  pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
    Ok(Self {
      tilemap: Texture::new(ctx, "./res/world.png")?,
      map: Map::load("./res/world.map").unwrap(),
    })
  }

  pub fn draw_base(&self, ctx: &mut Context) {
    draw_layer(ctx, &self.map.base, &self.tilemap, SCALE);
    draw_layer(ctx, &self.map.collide, &self.tilemap, SCALE);
    draw_layer(ctx, &self.map.decor, &self.tilemap, SCALE);
  }

  pub fn draw_overlay(&mut self, ctx: &mut Context) {
    draw_layer(ctx, &self.map.overlay, &self.tilemap, SCALE);
  }

  pub fn is_player_colliding(&mut self, position: Vec2<f32>) -> bool {
    self.is_colliding(Vec2::new(position.x - 16.0, position.y + 48.0))
      || self.is_colliding(Vec2::new(position.x + 16.0, position.y + 48.0))
      || self.is_colliding(Vec2::new(position.x - 16.0, position.y))
      || self.is_colliding(Vec2::new(position.x + 16.0, position.y))
  }

  fn is_colliding(&mut self, position: Vec2<f32>) -> bool {
    let x = (position.x / SCALE) as usize / 16;
    let y = (position.y / SCALE) as usize / 16;
    self.map.collide[y][x].0 != 0
  }
}
