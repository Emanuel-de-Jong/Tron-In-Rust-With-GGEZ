use std::hash::Hash;

pub const GRID_SIZE: f32 = 20.0;
pub const V_GRIDS: f32 = 36.0;
pub const H_GRIDS: f32 = 30.0;
pub const SCREEN_WIDTH: f32 = GRID_SIZE * V_GRIDS;
pub const SCREEN_HEIGHT: f32 = GRID_SIZE * H_GRIDS;
pub const FONT_PATH: &str = "/LiberationMono-Regular.ttf";

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

#[derive(PartialEq, Eq, Hash)]
pub enum Action {
    Restart
}