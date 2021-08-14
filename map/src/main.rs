use tetra::{Result, Context, ContextBuilder, State, Event};
use tetra::graphics::{self, Color, Texture, Rectangle, DrawParams};
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::text::{Font, Text};
use tetra::input::{MouseButton, Key, get_mouse_position};
use tetra::math::Vec2;

use map::{Map, LayerType, draw_tile};

pub const SCALE: f32 = 4.0;
pub const PATH: &'static str = "./res/world.map";

fn main() -> Result {
  ContextBuilder::new("Map Editor", 1280, 720)
    .quit_on_escape(true)
    .vsync(false)
    .show_mouse(true)
    .build()?
    .run(GameState::new)?;
  Ok(())
}

struct GameState {
  ui: UiState,
  tilemap: Texture,
  map: Map,
  cursor: Vec2<u16>,
  layer: LayerType,
  tile: (u8, u8),
}

impl GameState {
  fn new(ctx: &mut Context) -> Result<Self> {
    Ok(Self {
      ui: UiState::new(ctx)?,
      tilemap: Texture::new(ctx, "./res/world.png")?,
      map: Map::load(PATH).unwrap(),
      cursor: Vec2::new(0, 0),
      layer: LayerType::Base,
      tile: (1, 0),
    })
  }
}

impl State for GameState {
  fn draw(&mut self, ctx: &mut Context) -> Result {
    graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));

    map::draw_layer(ctx, &self.map.base, &self.tilemap, SCALE);
    map::draw_layer(ctx, &self.map.collide, &self.tilemap, SCALE);
    map::draw_layer(ctx, &self.map.decor, &self.tilemap, SCALE);
    map::draw_layer(ctx, &self.map.overlay, &self.tilemap, SCALE);

    self.ui.draw(ctx, self.cursor, self.tile, &self.tilemap);

    Ok(())
  }

  fn update(&mut self, ctx: &mut Context) -> Result {
    let position = get_mouse_position(ctx);
    self.cursor = Vec2::new(
      (position.x / SCALE) as u16 / 16,
      (position.y / SCALE) as u16 / 16,
    );

    self.ui.update(self.tile, &self.layer);
    Ok(())
  }

  fn event(&mut self, _: &mut Context, event: Event) -> Result {
    match event {
      Event::MouseButtonPressed { button } => {
        let cursor = self.cursor;
        match button {
          MouseButton::Left => {
            self.map.get_layer(&self.layer)[cursor.y as usize][cursor.x as usize] = self.tile
          }
          MouseButton::Middle => {
            self.tile = self.map.get_layer(&self.layer)[cursor.y as usize][cursor.x as usize];
          }
          MouseButton::Right => {
            self.map.get_layer(&self.layer)[cursor.y as usize][cursor.x as usize] = (0, 0)
          }
          _ => {}
        }
      }
      Event::KeyPressed { key } => match key {
        Key::Left => {
          if self.tile.0 > 1 {
            self.tile.0 -= 1
          }
        }
        Key::Right => self.tile.0 += 1,
        Key::Up => {
          if self.tile.1 > 0 {
            self.tile.1 -= 1
          }
        }
        Key::Down => self.tile.1 += 1,
        Key::PageDown => self.layer = self.layer.lower(),
        Key::PageUp => self.layer = self.layer.higher(),
        Key::S => {
          self.map.save(PATH).unwrap();
          self.ui.notify(format!("Saved:\n{}", PATH));
        }
        _ => {}
      },
      _ => {}
    }
    Ok(())
  }
}

struct UiState {
  hightlight: Mesh,
  bg: Mesh,
  title: Text,
  tile: Text,
  layer: Text,
  notify: Text,
  notify_time: u16,
}

impl UiState {
  fn new(ctx: &mut Context) -> Result<Self> {
    let font = Font::vector(ctx, "./res/font.ttf", 16.0)?;

    Ok(Self {
      hightlight: Mesh::rectangle(
        ctx,
        ShapeStyle::Fill,
        Rectangle::new(0.0, 0.0, 16.0 * SCALE, 16.0 * SCALE),
      )?,
      bg: Mesh::rectangle(
        ctx,
        ShapeStyle::Fill,
        Rectangle::new(0.0, 0.0, 300.0, 720.0),
      )?,
      title: Text::new("Map Editor:", font.clone()),
      tile: Text::new("Tile:\n(1, 0)", font.clone()),
      layer: Text::new("Layer: Base", font.clone()),
      notify: Text::new(format!("Loaded:\n{}", PATH), font.clone()),
      notify_time: 0,
    })
  }

  fn draw(&mut self, ctx: &mut Context, cursor: Vec2<u16>, tile: (u8, u8), tilemap: &Texture) {
    self.hightlight.draw(
      ctx,
      DrawParams::new()
        .position(Vec2::new(cursor.x.into(), cursor.y.into()) * 16.0 * SCALE)
        .color(Color::rgba8(255, 255, 255, 128)),
    );

    self
      .bg
      .draw(ctx, DrawParams::new().color(Color::rgba8(0, 0, 0, 128)));
    self.title.draw(ctx, Vec2::new(32.0, 32.0));
    self.tile.draw(ctx, Vec2::new(32.0, 80.0));
    self.layer.draw(ctx, Vec2::new(32.0, 148.0));
    if self.notify_time < 80 {
      self.notify.draw(ctx, Vec2::new(32.0, 640.0));
    }
    draw_tile(ctx, Vec2::new(148.0, 64.0), tile, tilemap, SCALE);
  }

  fn update(&mut self, tile: (u8, u8), layer: &LayerType) {
    self.notify_time += 1;
    self.tile.set_content(format!("Tile: \n{:?}", tile));
    self.layer.set_content(format!("Layer: {:?}", layer));
  }

  fn notify(&mut self, content: String) {
    self.notify.set_content(content);
    self.notify_time = 0;
  }
}
