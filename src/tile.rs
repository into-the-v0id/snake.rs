use tetra::math::Vec2;
use tetra::graphics;
use tetra::graphics::DrawParams;
use tetra::Context;
use crate::config;
use crate::color::Color;

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

	pub fn draw(&self, ctx: &mut Context) -> tetra::Result {
		let rectangle = graphics::Texture::from_rgba(ctx, 1, 1, &[255, 255, 255, 255])?;

		let space_size = config::TILE_SIZE as f32;
		let tile_size = (space_size * 0.9).round();
		let margin_size = ((space_size - tile_size) / 2.0).round();

		graphics::draw(
			ctx,
			&rectangle,
			DrawParams::new()
				.scale(Vec2::new(tile_size, tile_size))
				.position(Vec2::new(
					(self.position.x * config::TILE_SIZE as i32) as f32 + margin_size,
					(self.position.y * config::TILE_SIZE as i32) as f32 + margin_size
				))
				.color((&self.color).into())
		);

		Ok(())
	}
}
