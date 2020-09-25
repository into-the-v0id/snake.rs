use std::fmt;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	pub fn opposite(dir: &Direction) -> Direction {
		match dir {
			Direction::Up => Direction::Down,
			Direction::Down => Direction::Up,
			Direction::Left => Direction::Right,
			Direction::Right => Direction::Left,
		}
	}
}

impl fmt::Display for Direction {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.to_string())
	}
}

impl fmt::Debug for Direction {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let str = match self {
			Direction::Up => "Up",
			Direction::Down => "Down",
			Direction::Left => "Left",
			Direction::Right => "Right",
		};

		write!(f, "Direction::{}", str)
	}
}
