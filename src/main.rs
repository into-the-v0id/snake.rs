// Copyright (C) Oliver Amann
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 3 as
// published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use tetra::{graphics, Context, ContextBuilder, State as TetraState, Event};
use tetra::time::Timestep;
use crate::screen::Screen;
use crate::color::Color;

mod config;
mod screen;
mod color;
mod alert;
mod lazy_drawable;
mod drawable_collection;

pub enum ScreenRefMut<'a> {
    Start(&'a mut screen::StartScreen),
    Game(&'a mut screen::GameScreen),
}

impl ScreenRefMut<'_> {
    pub fn name(&self) -> ScreenName {
        match self {
            ScreenRefMut::Start(_) => ScreenName::Start,
            ScreenRefMut::Game(_) => ScreenName::Game,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ScreenName {
    Start,
    Game,
}

#[derive(Copy, Clone)]
pub struct CurrentScreen {
    name: ScreenName,
}

impl CurrentScreen {
    pub fn use_screen(&mut self, screen: ScreenName) {
        self.name = screen;
    }

    pub fn name(&self) -> &ScreenName {
        &self.name
    }
}

struct Screens {
    start: screen::StartScreen,
    game: screen::GameScreen,
}

struct State {
    current_screen: CurrentScreen,
    pub screens: Screens,
}

impl State {
    pub fn factory(ctx: &mut Context) -> tetra::Result<State> {
        Ok(State {
            current_screen: CurrentScreen {
                name: ScreenName::Start,
            },
            screens: Screens {
                start: screen::StartScreen::try_new(ctx)?,
                game: screen::GameScreen::try_new(ctx)?,
            }
        })
    }

    pub fn current_screen_mut(&mut self) -> &mut dyn Screen {
        match self.current_screen.name {
            ScreenName::Start => (&mut self.screens.start) as &mut dyn Screen,
            ScreenName::Game => (&mut self.screens.game) as &mut dyn Screen,
        }
    }
}

impl TetraState for State {
    fn update(&mut self, _ctx: &mut Context) -> tetra::Result {
        let mut current_screen = self.current_screen;
        self.current_screen_mut()
            .update(&mut current_screen);
        self.current_screen = current_screen;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgba(0, 0, 0, 1.0).into());

        self.current_screen_mut()
            .draw(ctx)?;

        Ok(())
    }

    fn event(&mut self, _ctx: &mut Context, event: Event) -> tetra::Result {
        let mut current_screen = self.current_screen;
        self.current_screen_mut()
            .event(&mut current_screen, event);
        self.current_screen = current_screen;

        Ok(())
    }
}

const PLAYGROUND_WIDTH: u16 = config::TILE_SIZE * config::TILE_COUNT_X;
const PLAYGROUND_HEIGHT: u16 = config::TILE_SIZE * config::TILE_COUNT_Y;
const WINDOW_WIDTH: u16 = PLAYGROUND_WIDTH + config::PLAYGROUND_WALL_WIDTH * 2;
const WINDOW_HEIGHT: u16 = PLAYGROUND_HEIGHT + config::PLAYGROUND_WALL_WIDTH * 2;

fn main() -> tetra::Result {
    ContextBuilder::new(
        "Snake",
        WINDOW_WIDTH as i32,
        WINDOW_HEIGHT as i32
    )
        .timestep(Timestep::Fixed(3.0))
        .show_mouse(true)
        .build()?
        .run(State::factory)
}
