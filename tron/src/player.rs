use tron::*;
use crate::keybinds::Keybinds;
use crate::vec2::Vec2;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, Color, Mesh};
use ggez::{Context, GameResult};
use ggez::timer;
use std::collections::HashSet;

pub const DRIVES_PER_SECOND: u32 = 10;

pub struct Player {
    name: String,
    prev_positions: HashSet<Vec2>,
    position: Vec2,
    rect: Mesh,
    trail_rect: Mesh,
    dir: Direction,
    dead: bool
}

impl Player {
    pub fn new(ctx: &mut Context, name: String, color: Color) -> GameResult<Player> {
        let mut trail_color = color.clone();
        trail_color.a = 0.7;

        let rect = Player::create_rect(ctx, color)?;
        let trail_rect = Player::create_rect(ctx, trail_color)?;

        let s = Player {
            name: name,
            prev_positions: HashSet::new(),
            position: Vec2::new(SCREEN_WIDTH/2.0, SCREEN_HEIGHT/2.0),
            rect: rect,
            trail_rect: trail_rect,
            dir: Direction::Right,
            dead: false
        };
        Ok(s)
    }

    fn create_rect(ctx: &mut Context, color: Color) -> GameResult<Mesh> {
        let shape = graphics::Rect::new(0.0, 0.0, GRID_SIZE, GRID_SIZE);
        Ok(Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), shape, color)?)
    }

    fn drive(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.dead {
            return Ok(());
        }

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

        self.check_collision(ctx)?;
        Ok(())
    }

    fn check_collision(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.prev_positions.contains(&self.position) {
            self.die(ctx)?;
        }
        Ok(())
    }

    fn die(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dead = true;
        self.rect = Player::create_rect(ctx, Color::new(1.0, 0.0, 0.0, 1.0))?;
        Ok(())
    }
    
    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, DRIVES_PER_SECOND) {
            self.drive(ctx)?;
            self.prev_positions.insert(self.position.clone());
        }
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.rect, (self.position,))?;

        for pos in self.prev_positions.iter() {
            graphics::draw(ctx, &self.trail_rect, (*pos,))?;
        }

        Ok(())
    }

    pub fn key_down_event(&mut self, _ctx: &mut Context, key: KeyCode, _mods: KeyMods, _b: bool) {
        let keybinds = Keybinds::player(&self.name);

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