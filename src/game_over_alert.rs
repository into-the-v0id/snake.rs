use tetra::Context;
use tetra::graphics;
use tetra::graphics::text::Font;
use crate::alert::Alert;
use crate::Drawable;

#[derive(Clone)]
pub struct GameOverAlert {
	pub base_alert: Alert,

	pub score: u16,
	pub score_label: String,

	font_builder: graphics::text::VectorFontBuilder,
	score_font: Option<Font>,
	score_label_font: Option<Font>,
}

impl GameOverAlert {
	pub fn try_new<T: Into<String>>(base_alert: Alert, score: u16, score_label: T) -> tetra::Result<GameOverAlert> {
		Ok(GameOverAlert {
			base_alert,

			score,
			score_label: score_label.into(),

			font_builder: graphics::text::VectorFontBuilder::new("./assets/fonts/digitalt/digitalt.ttf")?,
			score_font: None,
			score_label_font: None,
		})
	}
}

impl Drawable for GameOverAlert {
	fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
		self.base_alert.draw(ctx)?;

		if self.score_font.is_none() {
			self.score_font = Some(self.font_builder.with_size(ctx, 64.0)?);
		}
		if let Some(score_font) = &self.score_font {
			Alert::draw_text(ctx, self.score.to_string(), score_font.clone(), 290.0)?;
		}

		if self.score_label_font.is_none() {
			self.score_label_font = Some(self.font_builder.with_size(ctx, 16.0)?);
		}
		if let Some(score_label_font) = &self.score_label_font {
			Alert::draw_text(ctx, &self.score_label, score_label_font.clone(), 270.0)?;
		}

		Ok(())
	}
}
