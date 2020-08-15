use tetra::{Context, Event};
use super::{Screen, Drawable, Updatable, EventHandler};
use crate::CurrentScreen;

pub struct GameScreen {}

impl Drawable for GameScreen {
	fn draw(&mut self, _ctx: &mut Context) -> tetra::Result {
		println!("game_screen: draw");

		Ok(())
	}
}

impl Updatable for GameScreen {
	fn update(&mut self, _screen: &mut CurrentScreen) {
		println!("game_screen: update");
	}
}

impl EventHandler for GameScreen {
	fn event(&mut self, _screen: &mut CurrentScreen, _event: Event) {
		println!("game_screen: event");
	}
}

impl Screen for GameScreen {}

impl GameScreen {
	pub fn new(_ctx: &mut Context) -> GameScreen {
		GameScreen {}
	}
}
