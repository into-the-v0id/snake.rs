use crate::color::Color;

pub const TILE_SIZE: u16 = 50;
pub const TILE_COUNT_X: u16 = 15;
pub const TILE_COUNT_Y: u16 = 9;

pub const SNAKE_HEAD_COLOR: Color = Color::rgb(5, 185, 190);
pub const SNAKE_TAIL_COLOR: Color = Color::rgb(25, 200, 50);
pub const APPLE_COLOR: Color = Color::rgb(250, 40, 25);

pub const PLAYGROUND_GROUND_COLOR: Color = Color::rgb(198, 198, 198);
pub const PLAYGROUND_WALL_WIDTH: u16 = 5;
pub const PLAYGROUND_WALL_COLOR: Color = Color::rgb(56, 56, 56);

pub const ALERT_BACKGROUND_COLOR: Color = Color::rgba(0, 0, 0, 0.75);
pub const ALERT_FONT_COLOR: Color = Color::rgb(255, 255, 255);
