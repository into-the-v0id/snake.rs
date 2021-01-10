use tetra::{graphics, Context, Event};
use super::{Screen, Drawable, Updatable, EventHandler};
use crate::{CurrentScreen, ScreenName, Alert, LazyDrawable, WINDOW_WIDTH, WINDOW_HEIGHT, Color};
use tetra::input::{Key, MouseButton};

pub struct StartScreen {
	pub intro_alert: LazyDrawable<Alert>,
}

impl StartScreen {
	pub fn try_new(ctx: &mut Context) -> tetra::Result<StartScreen> {
		Ok(StartScreen {
			intro_alert: LazyDrawable::new(
				Alert::try_new("Snake", "Press 'Space' to start")?,
				graphics::Canvas::new(ctx, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)?,
				None
			),
		})
	}
}

impl Drawable for StartScreen {
	fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
		graphics::clear(ctx, Color::rgba(0, 0, 0, 1.0).into());

		self.intro_alert.draw(ctx)?;

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
