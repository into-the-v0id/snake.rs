use crate::tile::Tile;
use crate::config;
use tetra::Context;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

pub struct Snake {
	pub head: Tile,
	pub tail: Vec<Tile>,
	pub direction: Direction,
}

impl Snake {
	pub fn new() -> Snake {
		Snake {
			head: Tile::new(
				((config::TILE_COUNT_X - 1) as f32 / 2.0).floor() as u16,
				((config::TILE_COUNT_Y - 1) as f32 / 2.0).floor() as u16
			),
			tail: Vec::new(),
			direction: Direction::Up,
		}
	}

	pub fn update(&mut self) -> tetra::Result {
		match self.direction {
			Direction::Up => self.head.position.y -= 1,
			Direction::Down => self.head.position.y += 1,
			Direction::Left => self.head.position.x -= 1,
			Direction::Right => self.head.position.x += 1,
		}

		Ok(())
	}

	pub fn draw(&self, ctx: &mut Context) -> tetra::Result {
		self.head.draw(ctx)?;

		Ok(())
	}
}
