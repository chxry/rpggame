use std::time::Duration;
use tetra::Context;
use tetra::graphics::{Texture, Rectangle, DrawParams};
use tetra::graphics::animation::Animation;
use tetra::input::{is_key_down, Key};
use tetra::math::Vec2;

pub struct Player {
  assets: PlayerAssets,
  stats: PlayerStats,
  pub position: Vec2<f32>,
  pub velocity: Vec2<f32>,
}

impl Player {
  pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
    Ok(Self {
      assets: PlayerAssets::load(ctx)?,
      stats: PlayerStats::default(),
      position: Vec2::new(32.0, 32.0),
      velocity: Vec2::new(0.0, 0.0),
    })
  }

  pub fn movement(&mut self, ctx: &mut Context) {
    self.velocity = Vec2::new(0.0, 0.0);
    let mut speed = self.stats.speed;
    if is_key_down(ctx, Key::LeftShift) {
      speed += self.stats.sprint_speed;
    }
    if is_key_down(ctx, Key::W) {
      self.velocity.y -= 1.0;
    }
    if is_key_down(ctx, Key::A) {
      self.velocity.x -= 1.0;
    }
    if is_key_down(ctx, Key::S) {
      self.velocity.y += 1.0;
    }
    if is_key_down(ctx, Key::D) {
      self.velocity.x += 1.0;
    }

    self.velocity = self
      .velocity
      .try_normalized()
      .unwrap_or(Vec2::new(0.0, 0.0))
      * speed;
  }

  pub fn draw(&mut self, ctx: &mut Context) {
    if !self.velocity.is_approx_zero() {
      self.get_current_animation().advance(ctx);
    } else {
      self.get_current_animation().restart();
    }
    let position = self.position;
    self.get_current_animation().draw(
      ctx,
      DrawParams::new()
        .position(position)
        .origin(Vec2::new(8.0, 12.0)),
    );
  }

  fn get_current_animation(&mut self) -> &mut Animation {
    if self.velocity.y > 0.0 && self.velocity.y.abs() > self.velocity.x.abs() {
      &mut self.assets.down
    } else if self.velocity.x < 0.0 && self.velocity.y.abs() < self.velocity.x.abs() {
      &mut self.assets.left
    } else if self.velocity.y < 0.0 && self.velocity.y.abs() > self.velocity.x.abs() {
      &mut self.assets.up
    } else if self.velocity.x > 0.0 && self.velocity.y.abs() < self.velocity.x.abs() {
      &mut self.assets.right
    } else {
      &mut self.assets.down
    }
  }
}

struct PlayerAssets {
  down: Animation,
  right: Animation,
  up: Animation,
  left: Animation,
}

impl PlayerAssets {
  fn load(ctx: &mut Context) -> tetra::Result<Self> {
    let texture = Texture::new(ctx, "./res/player.png")?;

    Ok(Self {
      down: Self::create_anim(texture.clone(), 4.0),
      right: Self::create_anim(texture.clone(), 36.0),
      up: Self::create_anim(texture.clone(), 68.0),
      left: Self::create_anim(texture, 100.0),
    })
  }

  fn create_anim(texture: Texture, offset: f32) -> Animation {
    Animation::new(
      texture,
      Rectangle::row(0.0, offset, 16.0, 24.0).take(4).collect(),
      Duration::from_secs_f64(0.25),
    )
  }
}

struct PlayerStats {
  speed: f32,
  sprint_speed: f32,
}

impl PlayerStats {
  fn default() -> Self {
    Self {
      speed: 0.5,
      sprint_speed: 0.5,
    }
  }
}
