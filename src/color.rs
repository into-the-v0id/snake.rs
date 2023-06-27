use tetra::graphics::Color as TetraColor;

#[derive(Clone, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 1.0 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: f32) -> Color {
        Color { r, g, b, a }
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
            tetra_color.a,
        )
    }
}

impl From<Color> for TetraColor {
    fn from(color: Color) -> Self {
        let mut tetra_color = TetraColor::rgb8(color.r, color.g, color.b);
        tetra_color.a = color.a;

        tetra_color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_rgb() {
        let color = Color::rgb(1, 2, 3);

        assert_eq!(color.r, 1);
        assert_eq!(color.g, 2);
        assert_eq!(color.b, 3);
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn test_create_rgba() {
        let color = Color::rgba(255, 254, 253, 0.45);

        assert_eq!(color.r, 255);
        assert_eq!(color.g, 254);
        assert_eq!(color.b, 253);
        assert_eq!(color.a, 0.45);
    }

    #[test]
    fn test_create_transparent() {
        let color = Color::transparent();

        assert_eq!(color.a, 0.0);
    }

    #[test]
    fn test_create_from_tetra() {
        let tetra_color = TetraColor::rgb8(100, 150, 200);
        let color = Color::from(tetra_color);

        assert_eq!(color.r, 100);
        assert_eq!(color.g, 150);
        assert_eq!(color.b, 200);
        assert_eq!(color.a, 1.0);

        let tetra_color = TetraColor::rgba(255.0 / 255.0, 0.0 / 255.0, 128.0 / 255.0, 0.25);
        let color = Color::from(tetra_color);

        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 128);
        assert_eq!(color.a, 0.25);
    }

    #[test]
    fn test_as_tetra() {
        let color = Color::rgba(10, 20, 30, 0.8);
        let tetra_color = color.as_tetra();

        assert_eq!(tetra_color.r, 10.0 / 255.0);
        assert_eq!(tetra_color.g, 20.0 / 255.0);
        assert_eq!(tetra_color.b, 30.0 / 255.0);
        assert_eq!(tetra_color.a, 0.8);
    }

    #[test]
    fn test_into_tetra() {
        let color = Color::rgba(10, 20, 30, 0.8);
        let tetra_color: TetraColor = color.into();

        assert_eq!(tetra_color.r, 10.0 / 255.0);
        assert_eq!(tetra_color.g, 20.0 / 255.0);
        assert_eq!(tetra_color.b, 30.0 / 255.0);
        assert_eq!(tetra_color.a, 0.8);
    }
}
