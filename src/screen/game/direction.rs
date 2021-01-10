#[derive(Eq, PartialEq, Copy, Clone, Debug)]
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
