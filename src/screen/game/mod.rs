use tetra::{Context, Event};
use super::{Screen, Drawable, Updatable, EventHandler};
use crate::*;
use self::background::*;
use self::tile::*;
use self::snake::*;
use self::direction::*;
use self::game_over_alert::*;
use tetra::math::Vec2;
use rand::Rng;
use tetra::input::{MouseButton, Key};

mod background;
mod tile;
mod snake;
mod direction;
mod game_over_alert;

pub struct GameScreen {
	pub is_locked: bool,
	pub is_paused: bool,
	pub is_game_over: bool,

	pub background: LazyDrawable<Background>,
	pub snake: LazyDrawable<Snake>,
	snake_direction_queue: Vec<Direction>,
	pub apples: LazyDrawable<DrawableGroup<Tile>>,
	pub pause_alert: LazyDrawable<Alert>,
	pub game_over_alert: LazyDrawable<GameOverAlert>,
}

impl GameScreen {
	pub fn try_new(ctx: &mut Context) -> tetra::Result<GameScreen> {
		let mut state = GameScreen {
			is_locked: false,
			is_paused: false,
			is_game_over: false,

			background: LazyDrawable::new(
				Background,
				graphics::Canvas::new(ctx, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)?,
				None
			),
			snake: LazyDrawable::new(
				Snake::new(),
				graphics::Canvas::new(ctx, PLAYGROUND_WIDTH as i32, PLAYGROUND_HEIGHT as i32)?,
				Vec2::new(config::PLAYGROUND_WALL_WIDTH as f32, config::PLAYGROUND_WALL_WIDTH as f32)
			),
			snake_direction_queue: Vec::new(),
			apples: LazyDrawable::new(
				DrawableGroup::new(),
				graphics::Canvas::new(ctx, PLAYGROUND_WIDTH as i32, PLAYGROUND_HEIGHT as i32)?,
				Vec2::new(config::PLAYGROUND_WALL_WIDTH as f32, config::PLAYGROUND_WALL_WIDTH as f32)
			),
			pause_alert: LazyDrawable::new(
				Alert::try_new("Paused", "Press 'ESC' to resume")?,
				graphics::Canvas::new(ctx, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)?,
				None
			),
			game_over_alert: LazyDrawable::new(
				GameOverAlert::try_new(
					Alert::try_new("Game over", "Press 'R' to restart")?,
					0,
					"Score"
				)?,
				graphics::Canvas::new(ctx, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)?,
				None
			),
		};

		state.spawn_apple();

		Ok(state)
	}

	pub fn spawn_apple(&mut self) -> Option<&Tile> {
		let apple = Tile {
			position: self.choose_apple_position()?,
			color: Color::from(config::APPLE_COLOR),
		};

		self.apples.items.push(apple);

		self.apples.items.last()
	}

	pub fn choose_apple_position(&self) -> Option<Vec2<i32>> {
		let possible_positions = self.all_possible_apple_positions();
		if possible_positions.is_empty() {
			return None;
		}

		let index = rand::thread_rng()
			.gen_range(0, possible_positions.len());

		possible_positions.get(index).cloned()
	}

	pub fn all_possible_apple_positions(&self) -> Vec<Vec2<i32>> {
		let mut tail_positions: Vec<Vec2<i32>> = self.snake.tail.iter()
			.map(|tile| tile.position)
			.collect();

		let mut apple_positions: Vec<Vec2<i32>> = self.apples.items.iter()
			.map(|tile| tile.position)
			.collect();

		let mut blacklist = vec![
			self.snake.head.position,
			self.snake.get_next_head_position()
		];
		blacklist.append(&mut tail_positions);
		blacklist.append(&mut apple_positions);

		let mut possible_positions: Vec<Vec2<i32>> = Vec::new();
		for x in 0..config::TILE_COUNT_X {
			for y in 0..config::TILE_COUNT_Y {
				let pos = Vec2::new(x as i32, y as i32);

				if blacklist.contains(&pos) {
					continue;
				}

				possible_positions.push(pos);
			}
		}

		possible_positions
	}

	pub fn pause(&mut self) {
		self.is_paused = true;
		self.is_locked = true;
	}

	pub fn resume(&mut self) {
		self.is_paused = false;
		self.is_locked = false;
	}

	pub fn game_over(&mut self) {
		self.is_game_over = true;
		self.is_locked = true;

		self.game_over_alert.score = self.snake.tail.len() as u16;
		self.game_over_alert.updated = true;
	}

	pub fn restart(&mut self) {
		self.is_game_over = false;
		self.is_paused = false;
		self.is_locked = false;

		self.snake.inner = Snake::new();
		self.snake.updated = true;

		self.apples.items.clear();
		self.spawn_apple();
		self.apples.updated = true;

		self.game_over_alert.score = 0;
		self.game_over_alert.updated = true;
	}
}

impl Updatable for GameScreen {
	fn update(&mut self, _screen: &mut CurrentScreen) {
		if self.is_locked {
			return;
		}

		if ! self.snake_direction_queue.is_empty() {
			let dir_match = self.snake_direction_queue.iter().enumerate()
				.rfind(|(_index, &dir)| {
					dir != Direction::opposite(&self.snake.direction)
						&& dir != self.snake.direction
				});

			if let Some((index, &dir)) = dir_match {
				self.snake.direction = dir;
				self.snake_direction_queue = Vec::from(&self.snake_direction_queue[(index + 1)..]);
			} else {
				self.snake_direction_queue.clear();
			}
		}

		let next_head_pos = self.snake.get_next_head_position();

		let collided_apple_index = self.apples.items.iter()
			.enumerate()
			.find(|(_index, apple)| apple.position == next_head_pos)
			.map(|(index, _apple)| index);

		if let Some(index) = collided_apple_index {
			let new_position = self.choose_apple_position();

			if let Some(new_position) = new_position {
				let apple = self.apples.items.get_mut(index)
					.expect("Could not find apple");
				apple.position = new_position;
			} else {
				self.apples.items.remove(index);
			}
			self.apples.updated = true;

			self.snake.grow_tail();
			self.snake.updated = true;
		}

		let mut moved_snake = self.snake.inner.clone();
		moved_snake.move_forward();

		if moved_snake.head_collides() {
			self.game_over();
			return;
		}

		self.snake.inner = moved_snake;
		self.snake.updated = true;
	}
}

impl Drawable for GameScreen {
	fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
		graphics::clear(ctx, Color::rgba(0, 0, 0, 1.0).into());

		self.background.draw(ctx)?;
		self.apples.draw(ctx)?;
		self.snake.draw(ctx)?;
		if self.is_game_over { self.game_over_alert.draw(ctx)?; }
		if self.is_paused { self.pause_alert.draw(ctx)?; }

		Ok(())
	}
}

impl EventHandler for GameScreen {
	fn event(&mut self, _screen: &mut CurrentScreen, event: Event) {
		if ! self.is_locked {
			match event {
				Event::KeyPressed { key } => {
					match key {
						Key::W | Key::Up => {
							self.snake_direction_queue.push(Direction::Up);
						}
						Key::S | Key::Down => {
							self.snake_direction_queue.push(Direction::Down);
						}
						Key::A | Key::Left => {
							self.snake_direction_queue.push(Direction::Left);
						}
						Key::D | Key::Right => {
							self.snake_direction_queue.push(Direction::Right);
						}
						Key::Escape | Key::P => { self.pause(); }
						_ => {}
					}
				}
				Event::FocusLost => { self.pause(); }
				_ => {}
			}

			return;
		}

		if self.is_paused {
			match event {
				Event::KeyPressed { key } => {
					match key {
						Key::Escape | Key::P | Key::Space | Key::Enter | Key::NumPadEnter => {
							self.resume();
						}
						_ => {}
					}
				}
				Event::MouseButtonPressed { button } => {
					match button {
						MouseButton::Left => { self.resume(); }
						_ => {}
					}
				}
				_ => {}
			}

			return;
		}

		if self.is_game_over {
			match event {
				Event::KeyPressed { key } => {
					match key {
						Key::R | Key::Space | Key::Enter | Key::NumPadEnter => {
							self.restart();
						}
						_ => {}
					}
				}
				Event::MouseButtonPressed { button } => {
					match button {
						MouseButton::Left => { self.restart(); }
						_ => {}
					}
				}
				_ => {}
			}

			return;
		}
	}
}

impl Screen for GameScreen {}
