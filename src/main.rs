use tetra::math::Vec2;
use tetra::input::{Key, MouseButton};
use tetra::{graphics, Context, ContextBuilder, State as TetraState, Event};
use background::Background;
use snake::Snake;
use color::Color;
use tetra::time::Timestep;
use direction::Direction;
use alert::Alert;
use game_over_alert::GameOverAlert;
use stateful_drawable::StatefulDrawable;
use tile::Tile;
use drawable_group::DrawableGroup;
use rand;
use rand::Rng;

mod config;
mod color;
mod direction;
mod stateful_drawable;
mod background;
mod snake;
mod tile;
mod drawable_group;
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
    pub apples_wrapper: StatefulDrawable<DrawableGroup<Tile>>,
    pub pause_alert_wrapper: StatefulDrawable<Alert>,
    pub game_over_alert_wrapper: StatefulDrawable<GameOverAlert>,
}

impl GameState {
    pub fn factory(ctx: &mut Context) -> tetra::Result<GameState> {
        let mut state = GameState {
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
            apples_wrapper: StatefulDrawable::new(
                DrawableGroup::new(),
                graphics::Canvas::new(ctx, PLAYGROUND_SIZE_X as i32, PLAYGROUND_SIZE_Y as i32)?,
                Vec2::new(config::PLAYGROUND_WALL_WIDTH as f32, config::PLAYGROUND_WALL_WIDTH as f32)
            ),
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
        };

        state.spawn_apple();

        Ok(state)
    }

    pub fn spawn_apple(&mut self) -> &Tile {
        let apple = Tile {
            position: self.choose_apple_position(),
            color: Color::from(config::APPLE_COLOR),
        };

        self.apples_wrapper.inner.items.push(apple);

        self.apples_wrapper.inner.items.last()
            .expect("Cannot get last apple")
    }

    pub fn choose_apple_position(&self) -> Vec2<i32> {
        let mut rand_range = rand::thread_rng();
        let pos = Vec2::new(
            rand_range.gen_range(0, config::TILE_COUNT_X) as i32,
            rand_range.gen_range(0, config::TILE_COUNT_Y) as i32
        );

        let next_head_pos = self.snake_wrapper.inner.get_next_head_position();

        let is_blacklisted = self.snake_wrapper.inner.head.position == pos
            || next_head_pos == pos
            || self.snake_wrapper.inner.tail.iter().any(|tile| tile.position == pos)
            || self.apples_wrapper.inner.items.iter().any(|apple| apple.position == pos);

        if is_blacklisted {
            return self.choose_apple_position();
        }

        pos
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

        self.game_over_alert_wrapper.inner.score = self.snake_wrapper.inner.tail.len() as u16;
        self.game_over_alert_wrapper.state = ContextState::Updated;
    }

    pub fn restart(&mut self) {
        self.is_game_over = false;
        self.is_paused = false;
        self.is_locked = false;

        self.snake_wrapper.inner = Snake::new();
        self.snake_wrapper.state = ContextState::Updated;

        self.apples_wrapper.inner.items.clear();
        self.spawn_apple();
        self.apples_wrapper.state = ContextState::Updated;

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
                        && dir != self.snake_wrapper.inner.direction
                });

            if let Some((index, &dir)) = dir_match {
                self.snake_wrapper.inner.direction = dir;
                self.snake_direction_queue = Vec::from(&self.snake_direction_queue[(index + 1)..]);
            } else {
                self.snake_direction_queue.clear();
            }
        }

        let next_head_pos = self.snake_wrapper.inner.get_next_head_position();

        let collided_apple_index = self.apples_wrapper.inner.items.iter()
            .enumerate()
            .find(|(_index, apple)| apple.position == next_head_pos)
            .and_then(|(index, _apple)| Some(index));

        if let Some(index) = collided_apple_index {
            let new_position = self.choose_apple_position();

            let apple = self.apples_wrapper.inner.items.get_mut(index)
                .expect("Could not find apple");
            apple.position = new_position;
            self.apples_wrapper.state = ContextState::Updated;

            self.snake_wrapper.inner.grow_tail();
            self.snake_wrapper.state = ContextState::Updated;
        }

        let mut moved_snake = self.snake_wrapper.inner.clone();
        moved_snake.move_forward();

        if moved_snake.head_collides() {
            self.game_over();
            return Ok(());
        }

        self.snake_wrapper.inner = moved_snake;
        self.snake_wrapper.state = ContextState::Updated;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgba(0, 0, 0, 1.0).into());

        self.background_wrapper.draw(ctx)?;
        self.apples_wrapper.draw(ctx)?;
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
