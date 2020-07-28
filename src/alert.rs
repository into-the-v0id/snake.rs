use crate::config;
use tetra::Context;
use tetra::graphics;
use tetra::graphics::text::Font;
use tetra::math::Vec2;
use crate::color::Color;
use tetra::graphics::DrawParams;

pub struct Alert {
	pub title: &'static str,
	pub description: Option<&'static str>,

	font_builder: graphics::text::VectorFontBuilder,
	title_font: Option<Font>,
	description_font: Option<Font>,
}

impl Alert {
	pub fn try_new<T: Into<Option<&'static str>>>(title: &'static str, description: T) -> tetra::Result<Alert> {
		Ok(Alert {
			title,
			description: description.into(),

			font_builder: graphics::text::VectorFontBuilder::new("./assets/fonts/digitalt/digitalt.ttf")?,
			title_font: None,
			description_font: None,
		})
	}

	pub fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
		self.draw_background(ctx)?;

		if self.title_font.is_none() {
			self.title_font = Some(self.font_builder.with_size(ctx, 56.0)?);
		}
		if let Some(title_font) = &self.title_font {
			Alert::draw_text(ctx, self.title, title_font.clone(), 100.0)?;
		}

		if let Some(description) = self.description {
			if self.description_font.is_none() {
				self.description_font = Some(self.font_builder.with_size(ctx, 16.0)?);
			}
			if let Some(description_font) = &self.description_font {
				Alert::draw_text(ctx, description, description_font.clone(), 165.0)?;
			}
		}

		Ok(())
	}

	fn draw_background(&self, ctx: &mut Context) -> tetra::Result {
		let rectangle = graphics::Texture::from_rgba(ctx, 1, 1, &[255, 255, 255, 255])?;
		graphics::draw(
			ctx,
			&rectangle,
			DrawParams::new()
				.scale(Vec2::new(
					crate::WINDOW_SIZE_X as f32,
					crate::WINDOW_SIZE_Y as f32
				))
				.position(Vec2::new(0.0, 0.0))
				.color(Color::from(config::ALERT_BACKGROUND_COLOR).into())
		);

		Ok(())
	}

	fn draw_text(ctx: &mut Context, content: &'static str, font: Font, y: f32) -> tetra::Result {
		let text = graphics::text::Text::new(content, font);

		let bounds = text.get_bounds(ctx)
			.expect("Unable to calculate bounds of text");

		graphics::draw(
			ctx,
			&text,
			DrawParams::new()
				.color(Color::from(config::ALERT_FONT_COLOR).into())
				.position(Vec2::new(
					(crate::WINDOW_SIZE_X / 2) as f32 - bounds.width / 2.0,
					y
				))
		);

		Ok(())
	}
}
