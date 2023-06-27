use tetra::{Context, Event, graphics};
use super::{Screen, Drawable, Updatable, EventHandler};
use tetra::math::Vec2;
use rand::Rng;
use tetra::input::{MouseButton, Key};
use crate::alert::Alert;
use crate::drawable_collection::DrawableCollection;
use crate::lazy_drawable::LazyDrawable;
use background::Background;
use direction::Direction;
use game_over_alert::GameOverAlert;
use snake::Snake;
use tile::Tile;
use crate::color::Color;
use crate::{config, CurrentScreen, PLAYGROUND_HEIGHT, PLAYGROUND_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};

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
	pub apples: LazyDrawable<DrawableCollection<Tile>>,
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
				DrawableCollection::new(),
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

		state.reset();

		Ok(state)
	}

	pub fn spawn_apple(&mut self) -> Option<&Tile> {
		let apple = Tile {
			position: self.choose_apple_position()?,
			color: config::APPLE_COLOR,
		};

		self.apples.push(apple);

		self.apples.last()
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
		let tail_positions = self.snake.tail.iter()
			.map(|tile| tile.position);

		let apple_positions = self.apples.iter()
			.map(|tile| tile.position);

		let mut blacklist = vec![
			self.snake.head.position,
			self.snake.get_next_head_position()
		];
		blacklist.extend(tail_positions);
		blacklist.extend(apple_positions);

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

	fn position_is_out_of_bounds(&self, position: &Vec2<i32>) -> bool {
		position.x < 0 || position.x > (config::TILE_COUNT_X - 1) as i32
			|| position.y < 0 || position.y > (config::TILE_COUNT_Y - 1) as i32
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

		self.reset();

		self.game_over_alert.score = 0;
		self.game_over_alert.updated = true;
	}

	fn reset(&mut self) {
		self.snake.inner = Snake::new();
		let snake_start_size = std::env::var("SNAKE_START_SIZE")
			.unwrap_or("0".to_string())
			.parse::<u32>()
			.expect("Invalid SNAKE_START_SIZE");
		for _ in 0..snake_start_size {
			self.snake.grow_tail();
		}
		self.snake.updated = true;

		self.apples.clear();
		let apple_count = std::env::var("APPLE_COUNT")
			.unwrap_or("1".to_string())
			.parse::<u32>()
			.expect("Invalid APPLE_COUNT");
		for _ in 0..apple_count {
			if self.spawn_apple().is_none() {
				break;
			}
		}
		self.apples.updated = true;
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
					dir != self.snake.direction && dir != self.snake.direction.opposite()
				});

			if let Some((index, &dir)) = dir_match {
				self.snake.direction = dir;
				self.snake_direction_queue = Vec::from(&self.snake_direction_queue[(index + 1)..]);
			} else {
				self.snake_direction_queue.clear();
			}
		}

		let next_head_pos = self.snake.get_next_head_position();

		let collided_apple_index = self.apples.iter()
			.position(|apple| apple.position == next_head_pos);

		if let Some(index) = collided_apple_index {
			let new_position = self.choose_apple_position();

			if let Some(new_position) = new_position {
				let apple = self.apples.get_mut(index)
					.expect("Could not find apple");
				apple.position = new_position;
			} else {
				self.apples.remove(index);
			}
			self.apples.updated = true;

			self.snake.grow_tail();
			self.snake.updated = true;
		}

		if self.position_is_out_of_bounds(&next_head_pos)
			|| self.snake.position_collides(&next_head_pos) {
			self.game_over();
			return;
		}

		self.snake.move_forward();
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
