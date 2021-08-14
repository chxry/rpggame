use tetra::Context;
use tetra::graphics::{Texture, Camera};
use tetra::math::Vec2;

use map::{Map, draw_layer};

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
    draw_layer(ctx, &self.map.base, &self.tilemap, 1.0);
    draw_layer(ctx, &self.map.collide, &self.tilemap, 1.0);
    draw_layer(ctx, &self.map.decor, &self.tilemap, 1.0);
  }

  pub fn draw_overlay(&mut self, ctx: &mut Context) {
    draw_layer(ctx, &self.map.overlay, &self.tilemap, 1.0);
  }

  pub fn is_player_colliding(&mut self, position: Vec2<f32>) -> bool {
    self.is_colliding(Vec2::new(position.x - 4.0, position.y + 10.0))
      || self.is_colliding(Vec2::new(position.x + 4.0, position.y + 10.0))
      || self.is_colliding(Vec2::new(position.x - 4.0, position.y))
      || self.is_colliding(Vec2::new(position.x + 4.0, position.y))
  }

  pub fn get_camera(&self, mut camera: Camera, position: Vec2<f32>) -> Camera {
    camera.position = position;
    while camera.visible_rect().left() < 0.0 {
      camera.position.x += 0.1;
    }
    while camera.visible_rect().right() > self.map.get_width() as f32 * 16.0 {
      camera.position.x -= 0.1;
    }
    while camera.visible_rect().top() < 0.0 {
      camera.position.y += 0.1;
    }
    while camera.visible_rect().bottom() > self.map.get_height() as f32 * 16.0 {
      camera.position.y -= 0.1;
    }
    camera
  }

  fn is_colliding(&mut self, position: Vec2<f32>) -> bool {
    let x = position.x as usize / 16;
    let y = position.y as usize / 16;
    self.map.collide[y][x].0 != 0
  }
}
