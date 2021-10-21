use tron::*;
use crate::vec2::Vec2;
use crate::background;
use crate::keybinds::Keybinds;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Font, Mesh, Rect};


pub struct Cacher {
    pub font: Font,
    pub keybinds: Keybinds,
    pub bg_h_line: Mesh,
    pub bg_v_line: Mesh,
    pub player_shape: Rect
}

impl Cacher {
    pub fn new(ctx: &mut Context) -> GameResult<Cacher> {
        let bg_h_line = graphics::MeshBuilder::new()
            .line(
                &[
                    Vec2::new(0.0, 0.0),
                    Vec2::new(0.0, SCREEN_HEIGHT)
                ],
                1.0,
                background::LINE_COLOR,
            )?
            .build(ctx)?;
        
        let bg_v_line = graphics::MeshBuilder::new()
            .line(
                &[
                    Vec2::new(0.0, 0.0),
                    Vec2::new(SCREEN_WIDTH, 0.0)
                ],
                1.0,
                background::LINE_COLOR,
            )?
            .build(ctx)?;
        
        Ok(Cacher {
            font: Font::new(ctx, FONT_PATH)?,
            keybinds: Keybinds::default(),
            bg_h_line,
            bg_v_line,
            player_shape: Rect::new(0.0, 0.0, GRID_SIZE, GRID_SIZE)
        })
    }
}