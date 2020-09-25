use tetra::{Context, Event};

mod start;
mod game;

pub use start::StartScreen;
pub use game::GameScreen;
use crate::CurrentScreen;

pub trait Drawable {
	fn draw(&mut self, _ctx: &mut Context) -> tetra::Result {
		Ok(())
	}
}

pub trait Updatable {
	fn update(&mut self, _screen: &mut CurrentScreen) {}
}

pub trait EventHandler {
	fn event(&mut self, _screen: &mut CurrentScreen, _event: Event) {}
}

pub trait Screen: Drawable + Updatable + EventHandler {}
