use tetra::{graphics, Context, Event};
use super::{Screen, Drawable, Updatable, EventHandler};
use crate::{CurrentScreen, ScreenName, Alert, StatefulDrawable, WINDOW_SIZE_X, WINDOW_SIZE_Y, Color};
use tetra::input::{Key, MouseButton};

pub struct StartScreen {
	pub intro_alert_wrapper: StatefulDrawable<Alert>,
}

impl StartScreen {
	pub fn try_new(ctx: &mut Context) -> tetra::Result<StartScreen> {
		Ok(StartScreen {
			intro_alert_wrapper: StatefulDrawable::new(
				Alert::try_new("Snake", "Press 'Space' to start")?,
				graphics::Canvas::new(ctx, WINDOW_SIZE_X as i32, WINDOW_SIZE_Y as i32)?,
				None
			),
		})
	}
}

impl Drawable for StartScreen {
	fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
		graphics::clear(ctx, Color::rgba(0, 0, 0, 1.0).into());

		self.intro_alert_wrapper.draw(ctx)?;

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
