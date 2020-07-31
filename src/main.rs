use tetra::graphics;
use tetra::math::Vec2;
use tetra::input::Key;
use tetra::input::MouseButton;
use tetra::{Context, ContextBuilder, State as TetraState, Event};
use background::Background;
use snake::Snake;
use color::Color;
use tetra::time::Timestep;
use crate::snake::Direction;
use crate::alert::Alert;
use crate::game_over_alert::GameOverAlert;

mod config;
mod color;
mod background;
mod snake;
mod tile;
mod alert;
mod game_over_alert;

#[derive(Eq, PartialEq, Copy, Clone)]
enum ContextState {
    Updated,
    Drawn,
}

struct GameState {
    pub is_locked: bool,
    pub is_paused: bool,
    pub is_game_over: bool,

    pub background: Background,
    background_canvas: graphics::Canvas,
    background_state: ContextState,

    pub snake: Snake,
    snake_canvas: graphics::Canvas,
    snake_state: ContextState,
    snake_direction_queue: Option<Direction>,

    pub pause_alert: Alert,
    pause_alert_canvas: graphics::Canvas,
    pause_alert_state: ContextState,

    pub game_over_alert: GameOverAlert,
    game_over_alert_canvas: graphics::Canvas,
    game_over_alert_state: ContextState,
}

impl GameState {
    pub fn factory(ctx: &mut Context) -> tetra::Result<GameState> {
        tetra::Result::Ok(GameState {
            is_locked: false,
            is_paused: false,
            is_game_over: false,

            background: Background,
            background_canvas: graphics::Canvas::new(ctx, WINDOW_SIZE_X as i32, WINDOW_SIZE_Y as i32)?,
            background_state: ContextState::Updated,

            snake: Snake::new(),
            snake_canvas: graphics::Canvas::new(ctx, PLAYGROUND_SIZE_X as i32, PLAYGROUND_SIZE_Y as i32)?,
            snake_state: ContextState::Updated,
            snake_direction_queue: None,

            pause_alert: Alert::try_new("Paused", "Press 'ESC' to resume")?,
            pause_alert_canvas: graphics::Canvas::new(ctx, WINDOW_SIZE_X as i32, WINDOW_SIZE_Y as i32)?,
            pause_alert_state: ContextState::Updated,

            game_over_alert: GameOverAlert::try_new(
                Alert::try_new("Game over", "Press 'R' to restart")?,
                0,
                "Score"
            )?,
            game_over_alert_canvas: graphics::Canvas::new(ctx, WINDOW_SIZE_X as i32, WINDOW_SIZE_Y as i32)?,
            game_over_alert_state: ContextState::Updated,
        })
    }

    pub fn pause(&mut self) {
        self.is_paused = true;
        self.is_locked = true;
    }

    pub fn resume(&mut self) {
        self.is_paused = false;
        self.is_locked = false;
    }

    pub fn game_over(&mut self) {
        self.is_game_over = true;
        self.is_locked = true;
    }

    pub fn restart(&mut self) {
        self.is_game_over = false;
        self.is_paused = false;
        self.is_locked = false;

        self.snake = Snake::new();
        self.snake_state = ContextState::Updated;

        self.game_over_alert.score = 0;
        self.game_over_alert_state = ContextState::Updated;
    }
}

impl TetraState for GameState {
    fn update(&mut self, _ctx: &mut Context) -> tetra::Result {
        if self.is_locked {
            return Ok(());
        }

        if let Some(direction) = self.snake_direction_queue {
            self.snake.direction = direction;
            self.snake_direction_queue = None;
        }

        let mut snake_clone = self.snake.clone();
        snake_clone.move_forward()?;
        if snake_clone.head_collides() {
            self.game_over();
        } else {
            self.snake = snake_clone;
            self.snake_state = ContextState::Updated;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgba(0, 0, 0, 1.0).into());

        if self.background_state == ContextState::Updated {
            graphics::set_canvas(ctx, &self.background_canvas);
            graphics::clear(ctx, Color::transparent().into());
            self.background.draw(ctx)?;
            graphics::reset_canvas(ctx);

            self.background_state = ContextState::Drawn;
        }
        graphics::draw(ctx, &self.background_canvas, Vec2::new(0.0, 0.0));

        if self.snake_state == ContextState::Updated {
            graphics::set_canvas(ctx, &self.snake_canvas);
            graphics::clear(ctx, Color::transparent().into());
            self.snake.draw(ctx)?;
            graphics::reset_canvas(ctx);

            self.snake_state = ContextState::Drawn;
        }
        graphics::draw(
            ctx,
            &self.snake_canvas,
            Vec2::new(config::PLAYGROUND_WALL_WIDTH as f32, config::PLAYGROUND_WALL_WIDTH as f32)
        );

        if self.is_game_over {
            if self.game_over_alert_state == ContextState::Updated {
                graphics::set_canvas(ctx, &self.game_over_alert_canvas);
                graphics::clear(ctx, Color::transparent().into());
                self.game_over_alert.draw(ctx)?;
                graphics::reset_canvas(ctx);

                self.game_over_alert_state = ContextState::Drawn;
            }
            graphics::draw(ctx, &self.game_over_alert_canvas, Vec2::new(0.0, 0.0));
        }

        if self.is_paused {
            if self.pause_alert_state == ContextState::Updated {
                graphics::set_canvas(ctx, &self.pause_alert_canvas);
                graphics::clear(ctx, Color::transparent().into());
                self.pause_alert.draw(ctx)?;
                graphics::reset_canvas(ctx);

                self.pause_alert_state = ContextState::Drawn;
            }
            graphics::draw(ctx, &self.pause_alert_canvas, Vec2::new(0.0, 0.0));
        }

        Ok(())
    }

    fn event(&mut self, _ctx: &mut Context, event: Event) -> tetra::Result {
        if ! self.is_locked {
            match event {
                Event::KeyPressed { key } => {
                    match key {
                        Key::W | Key::Up if self.snake.direction != Direction::Down => {
                            self.snake_direction_queue = Some(Direction::Up);
                        }
                        Key::S | Key::Down if self.snake.direction != Direction::Up => {
                            self.snake_direction_queue = Some(Direction::Down);
                        }
                        Key::A | Key::Left if self.snake.direction != Direction::Right => {
                            self.snake_direction_queue = Some(Direction::Left);
                        }
                        Key::D | Key::Right if self.snake.direction != Direction::Left => {
                            self.snake_direction_queue = Some(Direction::Right);
                        }
                        Key::Escape | Key::P => { self.pause(); }
                        _ => {}
                    }
                }
                Event::FocusLost => { self.pause(); }
                _ => {}
            }

            return Ok(());
        }

        if self.is_paused {
            match event {
                Event::KeyPressed { key } => {
                    match key {
                        Key::Escape | Key::P | Key::Space | Key::Enter | Key::NumPadEnter => {
                            self.resume();
                        }
                        _ => {}
                    }
                }
                Event::MouseButtonPressed { button } => {
                    match button {
                        MouseButton::Left => { self.resume(); }
                        _ => {}
                    }
                }
                _ => {}
            }

            return Ok(());
        }

        if self.is_game_over {
            match event {
                Event::KeyPressed { key } => {
                    match key {
                        Key::R | Key::Space | Key::Enter | Key::NumPadEnter => {
                            self.restart();
                        }
                        _ => {}
                    }
                }
                Event::MouseButtonPressed { button } => {
                    match button {
                        MouseButton::Left => { self.restart(); }
                        _ => {}
                    }
                }
                _ => {}
            }

            return Ok(());
        }

        Ok(())
    }
}

const PLAYGROUND_SIZE_X: u16 = config::TILE_SIZE * config::TILE_COUNT_X;
const PLAYGROUND_SIZE_Y: u16 = config::TILE_SIZE * config::TILE_COUNT_Y;
const WINDOW_SIZE_X: u16 = PLAYGROUND_SIZE_X + config::PLAYGROUND_WALL_WIDTH * 2;
const WINDOW_SIZE_Y: u16 = PLAYGROUND_SIZE_Y + config::PLAYGROUND_WALL_WIDTH * 2;

fn main() -> tetra::Result {
    ContextBuilder::new(
        "Snake",
        WINDOW_SIZE_X as i32,
        WINDOW_SIZE_Y as i32
    )
        .timestep(Timestep::Fixed(3.0))
        .show_mouse(true)
        .build()?
        .run(GameState::factory)
}
