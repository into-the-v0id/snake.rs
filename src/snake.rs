use crate::tile::Tile;
use crate::direction::Direction;
use crate::{config, Drawable};
use tetra::Context;
use crate::color::Color;

#[derive(Clone)]
pub struct Snake {
	pub head: Tile,
	pub tail: Vec<Tile>,
	pub direction: Direction,
}

impl Snake {
	pub fn new() -> Snake {
		Snake {
			head: Tile::new(
				((config::TILE_COUNT_X - 1) as f32 * 0.5).floor() as i32,
				((config::TILE_COUNT_Y - 1) as f32 * 0.65).floor() as i32,
				Color::from(config::SNAKE_HEAD_COLOR)
			),
			tail: Vec::new(),
			direction: Direction::Up,
		}
	}

	pub fn move_forward(&mut self) -> tetra::Result {
		let head = &mut self.head;
		let mut tiles = self.tail.iter_mut().rev().peekable();
		while let Some(mut tile) = tiles.next() {
			let prev_tile = tiles.peek().unwrap_or(&head);
			tile.position = prev_tile.position;
		}

		match self.direction {
			Direction::Up => self.head.position.y -= 1,
			Direction::Down => self.head.position.y += 1,
			Direction::Left => self.head.position.x -= 1,
			Direction::Right => self.head.position.x += 1,
		}

		Ok(())
	}

	pub fn grow_tail(&mut self) {
		let last_tile = self.tail.last().unwrap_or(&self.head);

		let mut x = last_tile.position.x;
		let mut y = last_tile.position.y;

		match self.direction {
			Direction::Up => y += 1,
			Direction::Down => y -= 1,
			Direction::Left => x += 1,
			Direction::Right => x -= 1,
		}

		let tile = Tile::new(x, y, Color::from(config::SNAKE_TAIL_COLOR));
		self.tail.push(tile);
	}

	pub fn head_collides(&self) -> bool {
		self.head_is_out_of_bounds() || self.head_is_on_tail()
	}

	fn head_is_out_of_bounds(&self) -> bool {
		let x = self.head.position.x;
		let y = self.head.position.y;

		x < 0 || x > (config::TILE_COUNT_X - 1) as i32
			|| y < 0 || y > (config::TILE_COUNT_Y - 1) as i32
	}

	fn head_is_on_tail(&self) -> bool {
		for tile in self.tail.iter() {
			if tile.position == self.head.position {
				return true;
			}
		}

		false
	}
}

impl Drawable for Snake {
	fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
		for tile in self.tail.iter_mut() {
			tile.draw(ctx)?;
		}

		self.head.draw(ctx)?;

		Ok(())
	}
}
