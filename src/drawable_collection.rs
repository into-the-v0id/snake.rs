use crate::screen::Drawable;
use tetra::Context;
use std::ops::{Deref, DerefMut};

pub struct DrawableCollection<T> {
	items: Vec<T>
}

impl<T> DrawableCollection<T>
	where T: Drawable
{
	pub fn new() -> DrawableCollection<T> {
		DrawableCollection {
			items: Vec::new()
		}
	}
}

impl<T> Default for DrawableCollection<T>
	where T: Drawable
{
	fn default() -> Self {
		Self::new()
	}
}

impl <T> From<Vec<T>> for DrawableCollection<T>
	where T: Drawable
{
	fn from(items: Vec<T>) -> Self {
		DrawableCollection {
			items
		}
	}
}

impl <T> Deref for DrawableCollection<T>
{
	type Target = Vec<T>;

	fn deref(&self) -> &Self::Target {
		&self.items
	}
}

impl <T> DerefMut for DrawableCollection<T>
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.items
	}
}

impl<T> Drawable for DrawableCollection<T>
	where T: Drawable
{
	fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
		for item in self.items.iter_mut() {
			item.draw(ctx)?;
		}

		Ok(())
	}
}
