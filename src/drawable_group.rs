use crate::Drawable;
use tetra::Context;

pub struct DrawableGroup<I>
	where I: Drawable
{
	pub items: Vec<I>
}

impl<I> DrawableGroup<I>
	where I: Drawable
{
	pub fn new() -> DrawableGroup<I> {
		DrawableGroup {
			items: Vec::new()
		}
	}
}

impl<I> Default for DrawableGroup<I>
	where I: Drawable
{
	fn default() -> Self {
		Self::new()
	}
}

impl<I> Drawable for DrawableGroup<I>
	where I: Drawable
{
	fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
		for item in self.items.iter_mut() {
			item.draw(ctx)?;
		}

		Ok(())
	}
}
