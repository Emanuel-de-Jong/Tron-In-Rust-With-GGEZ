use tron::*;
use crate::vec2::Vec2;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, Color, Mesh, Font, Text};
use ggez::{Context, GameResult};
use std::collections::{HashMap, HashSet};

pub const DRIVES_PER_SECOND: u32 = 10;

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
    pub fn new(ctx: &mut Context, number: u8, position: Vec2, color: Color, starting_dir: Direction) -> GameResult<Player> {
        let mut trail_color = color.clone();
        trail_color.a = 0.7;

        let rect = Player::create_rect(ctx, color)?;
        let trail_rect = Player::create_rect(ctx, trail_color)?;

        let font = Font::new(ctx, FONT_PATH)?;
        let text = Text::new((number.to_string(), font, 20.0));
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
            text: Text::new((number.to_string(), font, 20.0)),
            text_offset: text_offset
        };
        Ok(s)
    }

    fn create_rect(ctx: &mut Context, color: Color) -> GameResult<Mesh> {
        let shape = graphics::Rect::new(0.0, 0.0, GRID_SIZE, GRID_SIZE);
        Ok(Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), shape, color)?)
    }

    fn drive(&mut self, ctx: &mut Context, all_prev_positions: &Vec<HashSet<Vec2>>) -> GameResult<()> {
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

        self.check_collision(ctx, all_prev_positions)?;
        Ok(())
    }

    fn check_collision(&mut self, ctx: &mut Context, all_prev_positions: &Vec<HashSet<Vec2>>) -> GameResult<()> {
        for prev_positions in all_prev_positions.iter() {
            if prev_positions.contains(&self.position) {
                self.die(ctx)?;
            }
        }
        Ok(())
    }

    fn die(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dead = true;
        self.rect = Player::create_rect(ctx, Color::new(1.0, 0.0, 0.0, 1.0))?;
        Ok(())
    }
}

impl Player {
    pub fn update(&mut self, ctx: &mut Context, all_prev_positions: &Vec<HashSet<Vec2>>) -> GameResult {
        if self.paused || self.dead {
            return Ok(());
        }

        self.drive(ctx, all_prev_positions)?;
        self.prev_positions.insert(self.position.clone());
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        for pos in self.prev_positions.iter() {
            graphics::draw(ctx, &self.trail_rect, (*pos,))?;
        }

        graphics::draw(ctx, &self.rect, (self.position,))?;
        graphics::draw(ctx, &self.text, (self.position + self.text_offset,))?;

        Ok(())
    }

    pub fn key_down_event(&mut self, _ctx: &mut Context, key: KeyCode, _mods: KeyMods, _b: bool, keybinds: &HashMap<Direction, KeyCode>) {
        if self.paused || self.dead {
            return;
        }

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