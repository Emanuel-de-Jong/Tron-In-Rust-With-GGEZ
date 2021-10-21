use tron::*;
use crate::vec2::Vec2;
use crate::cacher::Cacher;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, Color, Mesh, Text, TextFragment};
use ggez::{Context, GameResult};
use std::collections::{HashMap, HashSet};

pub const DRIVES_PER_SECOND: u32 = 10;
pub const PLAYER_COUNT: u8 = 2;
const DEAD_TEXT_COLOR: Color = Color::new(1.0, 0.0, 0.0, 1.0);

#[derive(Clone)]
pub struct Player {
    pub number: u8,
    pub prev_positions: HashSet<Vec2>,
    position: Vec2,
    rect: Mesh,
    trail_rect: Mesh,
    dir: Direction,
    pub dead: bool,
    pub paused: bool,
    text: Text,
    text_offset: Vec2
}

impl Player {
    pub fn new(ctx: &mut Context, cacher: &Cacher, number: u8, position: Vec2, color: Color, starting_dir: Direction) -> GameResult<Player> {
        let mut trail_color = color.clone();
        trail_color.a = 0.5;

        let rect = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), cacher.player_shape, color)?;
        let trail_rect = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), cacher.player_shape, trail_color)?;

        let text = Text::new((number.to_string(), cacher.font, 20.0));
        let text_offset = Vec2::new(
            (GRID_SIZE - text.width(ctx)) / 2.0,
            (GRID_SIZE - text.height(ctx)) / 2.0
        );

        let s = Player {
            number: number,
            prev_positions: HashSet::new(),
            position: position,
            rect: rect,
            trail_rect: trail_rect,
            dir: starting_dir,
            dead: false,
            paused: false,
            text: text,
            text_offset: text_offset
        };
        Ok(s)
    }

    fn drive(&mut self, ctx: &mut Context, cacher: &Cacher, all_prev_positions: &Vec<HashSet<Vec2>>) -> GameResult<()> {
        match self.dir {
            Direction::Left => {
                if self.position.x - GRID_SIZE >= 0.0 {
                    self.position.x -= GRID_SIZE;
                } else {
                    self.position.x = SCREEN_WIDTH - GRID_SIZE;
                }
            }
            Direction::Right => {
                self.position.x = (self.position.x + GRID_SIZE) % SCREEN_WIDTH;
            }
            Direction::Up => {
                if self.position.y - GRID_SIZE >= 0.0 {
                    self.position.y -= GRID_SIZE;
                } else {
                    self.position.y = SCREEN_HEIGHT - GRID_SIZE;
                }
            }
            Direction::Down => {
                self.position.y = (self.position.y + GRID_SIZE) % SCREEN_HEIGHT;
            }
        };

        self.check_collision(ctx, cacher, all_prev_positions)?;
        Ok(())
    }

    fn check_collision(&mut self, ctx: &mut Context, cacher: &Cacher, all_prev_positions: &Vec<HashSet<Vec2>>) -> GameResult<()> {
        for prev_positions in all_prev_positions.iter() {
            if prev_positions.contains(&self.position) {
                self.die(ctx, cacher)?;
            }
        }
        Ok(())
    }

    fn die(&mut self, ctx: &mut Context, cacher: &Cacher) -> GameResult<()> {
        self.dead = true;

        let fragment = TextFragment {
            text: self.number.to_string(),
            color: Some(DEAD_TEXT_COLOR),
            font: Some(cacher.font),
            scale: Some(22.0.into())
        };
        self.text = Text::new(fragment);
        Ok(())
    }
}

impl Player {
    pub fn update(&mut self, ctx: &mut Context, cacher: &Cacher, all_prev_positions: &Vec<HashSet<Vec2>>) -> GameResult {
        if self.paused || self.dead {
            return Ok(());
        }

        self.drive(ctx, cacher, all_prev_positions)?;
        self.prev_positions.insert(self.position.clone());
        Ok(())
    }

    pub fn draw_before(&mut self, ctx: &mut Context, cacher: &Cacher) -> GameResult {
        for pos in self.prev_positions.iter() {
            graphics::draw(ctx, &self.trail_rect, (*pos,))?;
        }
        Ok(())
    }

    pub fn draw_after(&mut self, ctx: &mut Context, cacher: &Cacher) -> GameResult {
        graphics::draw(ctx, &self.rect, (self.position,))?;
        graphics::draw(ctx, &self.text, (self.position + self.text_offset,))?;
        Ok(())
    }

    pub fn key_down_event(&mut self, _ctx: &mut Context, key: KeyCode, _mods: KeyMods, _b: bool, cacher: &Cacher) {
        if self.paused || self.dead {
            return;
        }

        let keybinds = cacher.keybinds.player(self.number);
        if key == keybinds[&Direction::Left] {
            if self.dir != Direction::Right {
                self.dir = Direction::Left;
            }
        }
        else if key == keybinds[&Direction::Right] {
            if self.dir != Direction::Left {
                self.dir = Direction::Right;
            }
        }
        else if key == keybinds[&Direction::Up] {
            if self.dir != Direction::Down {
                self.dir = Direction::Up;
            }
        }
        else if key == keybinds[&Direction::Down] {
            if self.dir != Direction::Up {
                self.dir = Direction::Down;
            }
        }
    }
}