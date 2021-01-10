use crate::{Drawable, Color};
use tetra::{graphics, Context};
use tetra::math::Vec2;
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct StatefulDrawable<T>
	where T: Drawable
{
	pub inner: T,
	pub updated: bool,
	canvas: graphics::Canvas,
	canvas_pos: Vec2<f32>,
}

impl <T> StatefulDrawable<T>
	where T: Drawable
{
	pub fn new<P: Into<Option<Vec2<f32>>>>(
		obj: T,
		canvas: graphics::Canvas,
		canvas_pos: P
	) -> StatefulDrawable<T> {
		StatefulDrawable {
			inner: obj,
			updated: true,
			canvas,
			canvas_pos: canvas_pos.into()
				.unwrap_or_else(|| Vec2::new(0.0, 0.0))
		}
	}
}

impl <T> Deref for StatefulDrawable<T>
	where T: Drawable
{
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl <T> DerefMut for StatefulDrawable<T>
	where T: Drawable
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl<T> Drawable for StatefulDrawable<T>
	where T: Drawable
{
	fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
		if self.updated {
			graphics::set_canvas(ctx, &self.canvas);
			graphics::clear(ctx, Color::transparent().into());
			self.inner.draw(ctx)?;
			graphics::reset_canvas(ctx);

			self.updated = false;
		}

		graphics::draw(ctx, &self.canvas, self.canvas_pos);

		Ok(())
	}
}
