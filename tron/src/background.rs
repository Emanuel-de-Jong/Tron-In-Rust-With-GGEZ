use tron::*;
use crate::vec2::Vec2;
use crate::cacher::Cacher;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};


pub const BG_COLOR: Color = Color::new(0.1, 0.2, 0.3, 1.0);
pub const LINE_COLOR: Color = Color::new(0.5, 0.5, 0.5, 1.0);


pub struct Background {}

impl Background {
    pub fn new(_ctx: &mut Context, _cacher: &Cacher) -> GameResult<Background> {
        Ok(Background {})
    }
}

impl Background {
    pub fn draw(&mut self, ctx: &mut Context, cacher: &Cacher) -> GameResult {
        for i in 1..H_GRIDS as i32 {
            graphics::draw(ctx, &cacher.bg_v_line, (Vec2::new(0.0, i as f32 * GRID_SIZE),))?;
        }

        for i in 1..V_GRIDS as i32 {
            graphics::draw(ctx, &cacher.bg_h_line, (Vec2::new(i as f32 * GRID_SIZE, 0.0),))?;
        }

        Ok(())
    }
}