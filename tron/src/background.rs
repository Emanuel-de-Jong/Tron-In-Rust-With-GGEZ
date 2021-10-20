use tron::*;
use crate::vec2::Vec2;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};

pub const BG_COLOR: Color = Color::new(0.1, 0.2, 0.3, 1.0);
const LINE_COLOR: Color = Color::new(0.5, 0.5, 0.5, 1.0);

pub struct Background {}

impl Background {
    pub fn new(_ctx: &mut Context) -> GameResult<Background> {
        Ok(Background {})
    }
    
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let v_line = graphics::MeshBuilder::new()
            .line(
                &[
                    Vec2::new(0.0, 0.0),
                    Vec2::new(SCREEN_WIDTH, 0.0)
                ],
                1.0,
                LINE_COLOR,
            )?
            .build(ctx)?;
        
        let h_line = graphics::MeshBuilder::new()
            .line(
                &[
                    Vec2::new(0.0, 0.0),
                    Vec2::new(0.0, SCREEN_HEIGHT)
                ],
                1.0,
                LINE_COLOR,
            )?
            .build(ctx)?;
        
        for i in 1..H_GRIDS as i32 {
            graphics::draw(ctx, &v_line, (Vec2::new(0.0, i as f32 * GRID_SIZE),))?;
        }

        for i in 1..V_GRIDS as i32 {
            graphics::draw(ctx, &h_line, (Vec2::new(i as f32 * GRID_SIZE, 0.0),))?;
        }

        Ok(())
    }
}