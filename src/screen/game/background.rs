use crate::{config, Drawable, Color};
use tetra::Context;
use tetra::graphics;
use tetra::math::Vec2;
use tetra::graphics::DrawParams;

#[derive(Clone)]
pub struct Background;

impl Drawable for Background {
	fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
		graphics::clear(ctx, Color::from(config::PLAYGROUND_WALL_COLOR).into());

		let rectangle = graphics::Texture::from_rgba(ctx, 1, 1, &[255, 255, 255, 255])?;
		graphics::draw(
			ctx,
			&rectangle,
			DrawParams::new()
				.scale(Vec2::new(
					crate::PLAYGROUND_SIZE_X as f32,
					crate::PLAYGROUND_SIZE_Y as f32
				))
				.position(Vec2::new(
					config::PLAYGROUND_WALL_WIDTH as f32,
					config::PLAYGROUND_WALL_WIDTH as f32
				))
				.color(Color::from(config::PLAYGROUND_GROUND_COLOR).into())
		);

		Ok(())
	}
}
