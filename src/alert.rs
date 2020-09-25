use crate::{config, Drawable, Color};
use tetra::Context;
use tetra::graphics;
use tetra::graphics::text::Font;
use tetra::math::Vec2;
use tetra::graphics::DrawParams;

#[derive(Clone)]
pub struct Alert {
	pub title: String,
	pub description: Option<String>,

	font_builder: graphics::text::VectorFontBuilder,
	title_font: Option<Font>,
	description_font: Option<Font>,
}

impl Alert {
	pub fn try_new<S: Into<String>, O: Into<Option<S>>>(title: S, description: O) -> tetra::Result<Alert> {
		Ok(Alert {
			title: title.into(),
			description: description.into().map(|desc| desc.into()),

			font_builder: graphics::text::VectorFontBuilder::new("./assets/fonts/digitalt/digitalt.ttf")?,
			title_font: None,
			description_font: None,
		})
	}

	fn draw_background(ctx: &mut Context) -> tetra::Result {
		let rectangle = graphics::Texture::from_rgba(ctx, 1, 1, &[255, 255, 255, 255])?;
		graphics::draw(
			ctx,
			&rectangle,
			DrawParams::new()
				.scale(Vec2::new(
					crate::WINDOW_WIDTH as f32,
					crate::WINDOW_HEIGHT as f32
				))
				.position(Vec2::new(0.0, 0.0))
				.color(Color::from(config::ALERT_BACKGROUND_COLOR).into())
		);

		Ok(())
	}

	pub fn draw_text<T: Into<String>>(ctx: &mut Context, content: T, font: Font, y: f32) -> tetra::Result {
		let text = graphics::text::Text::new(content.into(), font);

		let bounds = text.get_bounds(ctx)
			.expect("Unable to calculate bounds of text");

		graphics::draw(
			ctx,
			&text,
			DrawParams::new()
				.color(Color::from(config::ALERT_FONT_COLOR).into())
				.position(Vec2::new(
					(crate::WINDOW_WIDTH / 2) as f32 - bounds.width / 2.0,
					y
				))
		);

		Ok(())
	}
}

impl Drawable for Alert {
	fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
		Alert::draw_background(ctx)?;

		if self.title_font.is_none() {
			self.title_font = Some(self.font_builder.with_size(ctx, 56.0)?);
		}
		if let Some(title_font) = &self.title_font {
			Alert::draw_text(ctx, &self.title, title_font.clone(), 100.0)?;
		}

		if let Some(description) = &self.description {
			if self.description_font.is_none() {
				self.description_font = Some(self.font_builder.with_size(ctx, 16.0)?);
			}
			if let Some(description_font) = &self.description_font {
				Alert::draw_text(ctx, description, description_font.clone(), 165.0)?;
			}
		}

		Ok(())
	}
}
