use tetra::graphics;
use tetra::math::Vec2;
use tetra::input::Key;
use tetra::input::MouseButton;
use tetra::{Context, ContextBuilder, State as TetraState, Event};
use background::Background;
use snake::Snake;
use color::Color;
use tetra::time::Timestep;
use crate::direction::Direction;
use crate::alert::Alert;
use crate::game_over_alert::GameOverAlert;
use crate::stateful_drawable::StatefulDrawable;

mod config;
mod color;
mod direction;
mod stateful_drawable;
mod background;
mod snake;
mod tile;
mod alert;
mod game_over_alert;

pub trait Drawable {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result;
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ContextState {
    Updated,
    Drawn,
}

struct GameState {
    pub is_locked: bool,
    pub is_paused: bool,
    pub is_game_over: bool,

    pub background_wrapper: StatefulDrawable<Background>,
    pub snake_wrapper: StatefulDrawable<Snake>,
    snake_direction_queue: Vec<Direction>,
    pub pause_alert_wrapper: StatefulDrawable<Alert>,
    pub game_over_alert_wrapper: StatefulDrawable<GameOverAlert>,
}

impl GameState {
    pub fn factory(ctx: &mut Context) -> tetra::Result<GameState> {
        tetra::Result::Ok(GameState {
            is_locked: false,
            is_paused: false,
            is_game_over: false,

            background_wrapper: StatefulDrawable::new(
                Background,
                graphics::Canvas::new(ctx, WINDOW_SIZE_X as i32, WINDOW_SIZE_Y as i32)?,
                None
            ),
            snake_wrapper: StatefulDrawable::new(
                Snake::new(),
                graphics::Canvas::new(ctx, PLAYGROUND_SIZE_X as i32, PLAYGROUND_SIZE_Y as i32)?,
                Vec2::new(config::PLAYGROUND_WALL_WIDTH as f32, config::PLAYGROUND_WALL_WIDTH as f32)
            ),
            snake_direction_queue: Vec::new(),
            pause_alert_wrapper: StatefulDrawable::new(
                Alert::try_new("Paused", "Press 'ESC' to resume")?,
                graphics::Canvas::new(ctx, WINDOW_SIZE_X as i32, WINDOW_SIZE_Y as i32)?,
                None
            ),
            game_over_alert_wrapper: StatefulDrawable::new(
                GameOverAlert::try_new(
                    Alert::try_new("Game over", "Press 'R' to restart")?,
                    0,
                    "Score"
                )?,
                graphics::Canvas::new(ctx, WINDOW_SIZE_X as i32, WINDOW_SIZE_Y as i32)?,
                None
            ),
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

        self.snake_wrapper.inner = Snake::new();
        self.snake_wrapper.state = ContextState::Updated;

        self.game_over_alert_wrapper.inner.score = 0;
        self.game_over_alert_wrapper.state = ContextState::Updated;
    }
}

impl TetraState for GameState {
    fn update(&mut self, _ctx: &mut Context) -> tetra::Result {
        if self.is_locked {
            return Ok(());
        }

        if self.snake_direction_queue.len() > 0 {
            let dir_match = self.snake_direction_queue.iter().enumerate()
                .rfind(|(_index, &dir)| {
                    dir != Direction::opposite(&self.snake_wrapper.inner.direction)
                });

            if let Some((index, &dir)) = dir_match {
                self.snake_wrapper.inner.direction = dir;
                self.snake_direction_queue = Vec::from(&self.snake_direction_queue[(index + 1)..]);
            } else {
                self.snake_direction_queue.clear();
            }
        }

        let mut snake_clone = self.snake_wrapper.inner.clone();
        snake_clone.move_forward();
        if snake_clone.head_collides() {
            self.game_over();
        } else {
            self.snake_wrapper.inner = snake_clone;
            self.snake_wrapper.state = ContextState::Updated;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgba(0, 0, 0, 1.0).into());

        self.background_wrapper.draw(ctx)?;
        self.snake_wrapper.draw(ctx)?;
        if self.is_game_over { self.game_over_alert_wrapper.draw(ctx)?; }
        if self.is_paused { self.pause_alert_wrapper.draw(ctx)?; }

        Ok(())
    }

    fn event(&mut self, _ctx: &mut Context, event: Event) -> tetra::Result {
        if ! self.is_locked {
            match event {
                Event::KeyPressed { key } => {
                    match key {
                        Key::W | Key::Up => {
                            self.snake_direction_queue.push(Direction::Up);
                        }
                        Key::S | Key::Down => {
                            self.snake_direction_queue.push(Direction::Down);
                        }
                        Key::A | Key::Left => {
                            self.snake_direction_queue.push(Direction::Left);
                        }
                        Key::D | Key::Right => {
                            self.snake_direction_queue.push(Direction::Right);
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
