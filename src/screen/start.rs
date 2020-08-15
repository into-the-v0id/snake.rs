use tetra::{Context, Event};
use super::{Screen, Drawable, Updatable, EventHandler};
use crate::{CurrentScreen, ScreenName};
use tetra::input::{Key, MouseButton};

pub struct StartScreen {}

impl StartScreen {
	pub fn new(_ctx: &mut Context) -> StartScreen {
		StartScreen {}
	}
}

impl Drawable for StartScreen {
	fn draw(&mut self, _ctx: &mut Context) -> tetra::Result {
		//

		Ok(())
	}
}

impl Updatable for StartScreen {
	fn update(&mut self, _screen: &mut CurrentScreen) {
		//
	}
}

impl EventHandler for StartScreen {
	fn event(&mut self, screen: &mut CurrentScreen, event: Event) {
		match event {
			Event::KeyPressed { key } => {
				match key {
					Key::Space | Key::Enter | Key::NumPadEnter => {
						screen.use_screen(ScreenName::Game);
					}
					_ => {}
				}
			}
			Event::MouseButtonPressed { button } => {
				match button {
					MouseButton::Left => {
						screen.use_screen(ScreenName::Game);
					}
					_ => {}
				}
			}
			_ => {}
		};
	}
}

impl Screen for StartScreen {}
