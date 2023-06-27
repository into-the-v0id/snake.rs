use tetra::{Context, Event};

mod game;
mod start;

use crate::CurrentScreen;
pub use game::GameScreen;
pub use start::StartScreen;

pub trait Drawable {
    fn draw(&mut self, _ctx: &mut Context) -> tetra::Result {
        Ok(())
    }
}

pub trait Updatable {
    fn update(&mut self, _screen: &mut CurrentScreen) {}
}

pub trait EventHandler {
    fn event(&mut self, _screen: &mut CurrentScreen, _event: Event) {}
}

pub trait Screen: Drawable + Updatable + EventHandler {}
