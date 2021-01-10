use tetra::graphics::Color as TetraColor;

#[derive(Clone, Default)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: f32
}

impl Color {
	pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
		Color {
			r, g, b,
			a: 1.0
		}
	}

	pub const fn rgba(r: u8, g: u8, b: u8, a: f32) -> Color {
		Color {
			r, g, b, a
		}
	}

	pub const fn transparent() -> Color {
		Color::rgba(0, 0, 0, 0.0)
	}

	pub fn as_tetra(&self) -> TetraColor {
		let mut tetra_color = TetraColor::rgb8(self.r, self.g, self.b);
		tetra_color.a = self.a;

		tetra_color
	}
}

impl From<TetraColor> for Color {
	fn from(tetra_color: TetraColor) -> Self {
		Color::rgba(
			(tetra_color.r * 255.0).round() as u8,
			(tetra_color.g * 255.0).round() as u8,
			(tetra_color.b * 255.0).round() as u8,
			tetra_color.a
		)
	}
}

impl Into<TetraColor> for Color {
	fn into(self) -> TetraColor {
		let mut tetra_color = TetraColor::rgb8(self.r, self.g, self.b);
		tetra_color.a = self.a;

		tetra_color
	}
}
