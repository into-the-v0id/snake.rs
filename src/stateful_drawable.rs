use crate::{Drawable, Color};
use tetra::{graphics, Context};
use tetra::math::Vec2;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ContextState {
	Updated,
	Drawn,
}

#[derive(Clone)]
pub struct StatefulDrawable<T>
	where T: Drawable
{
	pub inner: T,
	pub state: ContextState,
	canvas: graphics::Canvas,
	canvas_pos: Vec2<f32>,
}

impl<T> StatefulDrawable<T>
	where T: Drawable
{
	pub fn new<P: Into<Option<Vec2<f32>>>>(
		obj: T,
		canvas: graphics::Canvas,
		canvas_pos: P
	) -> StatefulDrawable<T> {
		StatefulDrawable {
			inner: obj,
			state: ContextState::Updated,
			canvas,
			canvas_pos: canvas_pos.into().unwrap_or(Vec2::new(0.0, 0.0))
		}
	}
}

impl<T> Drawable for StatefulDrawable<T>
	where T: Drawable
{
	fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
		if self.state == ContextState::Updated {
			graphics::set_canvas(ctx, &self.canvas);
			graphics::clear(ctx, Color::transparent().into());
			self.inner.draw(ctx)?;
			graphics::reset_canvas(ctx);

			self.state = ContextState::Drawn;
		}

		graphics::draw(ctx, &self.canvas, self.canvas_pos);

		Ok(())
	}
}
