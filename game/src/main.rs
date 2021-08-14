mod player;
mod ui;
mod world;

use std::error::Error;
use tetra::{Context, ContextBuilder, State, Event};
use tetra::graphics::{self, Color, Camera};
use tetra::input::Key;
use tetra::math::Vec2;
use simplelog::{TermLogger, LevelFilter, Config, TerminalMode, ColorChoice};
use log::info;

use player::Player;
use world::World;
use ui::UiState;

pub const WIDTH: i32 = 1280;
pub const HEIGHT: i32 = 720;

fn main() -> Result<(), Box<dyn Error>> {
  TermLogger::init(
    LevelFilter::Debug,
    Config::default(),
    TerminalMode::Stdout,
    ColorChoice::Auto,
  )?;
  ContextBuilder::new("game", WIDTH, HEIGHT)
    .quit_on_escape(true) //
    .vsync(false)
    .build()?
    .run(GameState::new)?;
  Ok(())
}

struct GameState {
  camera: Camera,
  player: Player,
  world: World,
  ui: UiState,
}

impl GameState {
  fn new(ctx: &mut Context) -> tetra::Result<Self> {
    let info = graphics::get_device_info(ctx);
    info!("OpenGL Vendor:   {}", info.vendor);
    info!("OpenGL Renderer: {}", info.renderer);
    info!("OpenGL Version:  {}", info.opengl_version);
    info!("GLSL Version:    {}", info.glsl_version);

    let mut camera = Camera::with_window_size(ctx);
    camera.scale = Vec2::new(4.0, 4.0);
    camera.update();

    Ok(Self {
      camera,
      player: Player::new(ctx)?,
      world: World::new(ctx)?,
      ui: UiState::new(ctx)?,
    })
  }
}

impl State for GameState {
  fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
    graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));
    graphics::set_transform_matrix(ctx, self.camera.as_matrix());

    self.world.draw_base(ctx);
    self.player.draw(ctx);
    self.world.draw_overlay(ctx);

    graphics::reset_transform_matrix(ctx);
    self.ui.draw(ctx)?;
    Ok(())
  }

  fn update(&mut self, ctx: &mut Context) -> tetra::Result {
    self.player.movement(ctx);
    let pos = self.player.position + self.player.velocity;
    if !self.world.is_player_colliding(pos) {
      self.player.position = pos;

      self.camera = self.world.get_camera(self.camera.clone(), pos);
      self.camera.update();
    }
    Ok(())
  }

  fn event(&mut self, _: &mut Context, event: Event) -> tetra::Result {
    match event {
      Event::KeyPressed { key: Key::F3 } => self.ui.toggle_debug(),
      Event::Resized { width, height } => {
        self.camera.set_viewport_size(width as f32, height as f32)
      }
      _ => {}
    }
    Ok(())
  }
}
