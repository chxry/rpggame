use std::io::{Error, Read, Write};
use std::convert::TryInto;
use std::path::Path;
use std::fs::File;
use tetra::{Context};
use tetra::graphics::{Texture, DrawParams, Rectangle};
use tetra::math::Vec2;

pub type MapLayer = Vec<Vec<(u8, u8)>>;

pub struct Map {
  pub base: MapLayer,
  pub collide: MapLayer,
  pub decor: MapLayer,
  pub overlay: MapLayer,
}

impl Map {
  pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
    let mut file = File::open(path)?;

    let width = read_u8(&mut file)?;
    let height = read_u8(&mut file)?;

    let base = load_layer(&mut file, width, height)?;
    let collide = load_layer(&mut file, width, height)?;
    let decor = load_layer(&mut file, width, height)?;
    let overlay = load_layer(&mut file, width, height)?;
    Ok(Map {
      base,
      collide,
      decor,
      overlay,
    })
  }

  pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
    let mut file = File::create(path)?;
    let width = self.get_width();
    let height = self.get_height();

    file.write(&width.to_be_bytes())?;
    file.write(&height.to_be_bytes())?;

    save_layer(&mut file, &self.base)?;
    save_layer(&mut file, &self.collide)?;
    save_layer(&mut file, &self.decor)?;
    save_layer(&mut file, &self.overlay)?;

    Ok(())
  }

  pub fn add_row(&mut self, layer: &LayerType, tile: (u8, u8)) {
    let width = self.get_width().into();
    self.get_layer(layer).push(vec![tile; width])
  }

  pub fn add_col(&mut self, layer: &LayerType, tile: (u8, u8)) {
    for row in self.get_layer(layer) {
      row.push(tile);
    }
  }

  pub fn get_layer(&mut self, layer: &LayerType) -> &mut MapLayer {
    match layer {
      LayerType::Base => &mut self.base,
      LayerType::Collide => &mut self.collide,
      LayerType::Decor => &mut self.decor,
      LayerType::Overlay => &mut self.overlay,
    }
  }

  pub fn get_width(&self) -> u8 {
    self.base[0].len().try_into().unwrap()
  }

  pub fn get_height(&self) -> u8 {
    self.base.len().try_into().unwrap()
  }
}

pub fn draw_layer(ctx: &mut Context, tiles: &MapLayer, tilemap: &Texture, scale: f32) {
  for (row, cols) in tiles.iter().enumerate() {
    for (col, tile) in cols.iter().enumerate() {
      draw_tile(
        ctx,
        Vec2::new(col as f32 * 16.0 * scale, row as f32 * 16.0 * scale),
        *tile,
        tilemap,
        scale,
      )
    }
  }
}

pub fn draw_tile(
  ctx: &mut Context,
  position: Vec2<f32>,
  tile: (u8, u8),
  tilemap: &Texture,
  scale: f32,
) {
  if tile.0 != 0 {
    tilemap.draw_region(
      ctx,
      Rectangle::new((tile.0 - 1) as f32 * 16.0, tile.1 as f32 * 16.0, 16.0, 16.0),
      DrawParams::new()
        .position(position)
        .scale(Vec2::new(scale, scale)),
    );
  }
}

fn load_layer<R: Read>(reader: &mut R, width: u8, height: u8) -> Result<MapLayer, Error> {
  let mut layer = Vec::new();
  for _ in 0..height {
    let mut row = Vec::new();
    for _ in 0..width {
      let x = read_u8(reader)?;
      let y = read_u8(reader)?;
      row.push((x, y));
    }
    layer.push(row);
  }
  Ok(layer)
}

fn save_layer<W: Write>(writer: &mut W, layer: &MapLayer) -> Result<(), Error> {
  for row in layer {
    for col in row {
      writer.write(&col.0.to_be_bytes())?;
      writer.write(&col.1.to_be_bytes())?;
    }
  }
  Ok(())
}

fn read_u8<R: Read>(reader: &mut R) -> Result<u8, Error> {
  let mut buf = [0];
  reader.read_exact(&mut buf)?;
  Ok(u8::from_be_bytes(buf))
}

#[derive(Debug)]
pub enum LayerType {
  Base,
  Collide,
  Decor,
  Overlay,
}

impl LayerType {
  pub fn higher(&self) -> LayerType {
    match self {
      LayerType::Base => LayerType::Collide,
      LayerType::Collide => LayerType::Decor,
      LayerType::Decor => LayerType::Overlay,
      LayerType::Overlay => LayerType::Overlay,
    }
  }

  pub fn lower(&self) -> LayerType {
    match self {
      LayerType::Base => LayerType::Base,
      LayerType::Collide => LayerType::Base,
      LayerType::Decor => LayerType::Collide,
      LayerType::Overlay => LayerType::Decor,
    }
  }
}
