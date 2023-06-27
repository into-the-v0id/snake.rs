use crate::config;
use crate::screen::Drawable;
use tetra::graphics;
use tetra::graphics::DrawParams;
use tetra::math::Vec2;
use tetra::Context;

#[derive(Clone)]
pub struct Background;

impl Drawable for Background {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, config::PLAYGROUND_WALL_COLOR.as_tetra());

        let rectangle = graphics::Texture::from_rgba(ctx, 1, 1, &[255, 255, 255, 255])?;
        graphics::draw(
            ctx,
            &rectangle,
            DrawParams::new()
                .scale(Vec2::new(
                    crate::PLAYGROUND_WIDTH as f32,
                    crate::PLAYGROUND_HEIGHT as f32,
                ))
                .position(Vec2::new(
                    config::PLAYGROUND_WALL_WIDTH as f32,
                    config::PLAYGROUND_WALL_WIDTH as f32,
                ))
                .color(config::PLAYGROUND_GROUND_COLOR.as_tetra()),
        );

        Ok(())
    }
}
