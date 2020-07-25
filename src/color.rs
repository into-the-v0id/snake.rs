use tetra::graphics::Color as TetraColor;
use crate::config;

pub struct Color(TetraColor);

impl Color {
	#[allow(dead_code)]
	pub fn new(color: TetraColor) -> Color {
		Color(color)
	}

	pub fn rgb(r: u8, g: u8, b: u8) -> Color {
		Color(TetraColor::rgb8(r, g, b))
	}

	pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Color {
		Color(TetraColor::rgba8(r, g, b, (a * 255.0).round() as u8))
	}

	pub fn transparent() -> Color {
		Color::rgba(0, 0, 0, 0.0)
	}
}

impl From<config::Color> for Color {
	fn from(color: (u8, u8, u8, f32)) -> Self {
		Color::rgba(color.0, color.1, color.2, color.3)
	}
}

impl From<TetraColor> for Color {
	fn from(color: TetraColor) -> Self {
		Color(color)
	}
}

impl Into<TetraColor> for Color {
	fn into(self) -> TetraColor {
		self.0
	}
}

impl Into<TetraColor> for &Color {
	fn into(self) -> TetraColor {
		self.0
	}
}
