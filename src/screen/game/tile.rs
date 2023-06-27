use crate::color::Color;
use crate::config;
use crate::screen::Drawable;
use tetra::math::Vec2;
use tetra::{graphics, Context};

#[derive(Clone)]
pub struct Tile {
    pub position: Vec2<i32>,
    pub color: Color,
}

impl Tile {
    pub fn new(x: i32, y: i32, color: Color) -> Tile {
        Tile {
            position: Vec2::new(x, y),
            color,
        }
    }
}

const TILE_SIZE: f32 = config::TILE_SIZE as f32 * 0.9;
const MARGIN_SIZE: f32 = (config::TILE_SIZE as f32 - TILE_SIZE) / 2.0;

impl Drawable for Tile {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        let rectangle = graphics::Texture::from_rgba(ctx, 1, 1, &[255, 255, 255, 255])?;

        graphics::draw(
            ctx,
            &rectangle,
            graphics::DrawParams::new()
                .scale(Vec2::new(TILE_SIZE, TILE_SIZE))
                .position(Vec2::new(
                    (self.position.x * config::TILE_SIZE as i32) as f32 + MARGIN_SIZE,
                    (self.position.y * config::TILE_SIZE as i32) as f32 + MARGIN_SIZE,
                ))
                .color(self.color.as_tetra()),
        );

        Ok(())
    }
}
