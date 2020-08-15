use tetra::{Context, Event};
use super::{Screen, Drawable, Updatable, EventHandler};
use crate::CurrentScreen;

pub struct StartScreen {}

impl StartScreen {
	pub fn new(_ctx: &mut Context) -> StartScreen {
		StartScreen {}
	}
}

impl Drawable for StartScreen {
	fn draw(&mut self, _ctx: &mut Context) -> tetra::Result {
		println!("start_screen: draw");

		Ok(())
	}
}

impl Updatable for StartScreen {
	fn update(&mut self, _screen: &mut CurrentScreen) {
		println!("start_screen: update");
	}
}

impl EventHandler for StartScreen {
	fn event(&mut self, _screen: &mut CurrentScreen, _event: Event) {
		println!("start_screen: event");
	}
}

impl Screen for StartScreen {}
