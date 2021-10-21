mod background;
mod player;
mod keybinds;
mod vec2;
mod cacher;

use vec2::Vec2;
use cacher::Cacher;
use tron::*;
use background::Background;
use player::Player;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, Text};
use ggez::timer;
use ggez::event::{self, KeyCode, KeyMods};
use std::{env, path};
use std::collections::HashSet;

pub struct MainState {
    pub cacher: Cacher,
    pub background: Background,
    pub players: Vec<Player>,
    pub win_text: Option<Text>,
    pub win_text_offset: Option<Vec2>
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let cacher = Cacher::new(ctx)?;

        let x = (V_GRIDS/10.0).floor()*GRID_SIZE;
        let y = (H_GRIDS/10.0).floor()*GRID_SIZE;

        let mut players: Vec<Player> = Vec::new();
        for i in 1..player::PLAYER_COUNT+1 {
            let mut position = Vec2::new(SCREEN_WIDTH/2.0, SCREEN_HEIGHT/2.0);
            let mut color = Color::new(0.5, 0.5, 0.5, 1.0);
            let mut starting_dir = Direction::Left;
            match i {
                1 => {
                    position.x = x;
                    color.b = 1.0;
                    starting_dir = Direction::Right;
                }
                2 => {
                    position.x = SCREEN_WIDTH-x-GRID_SIZE;
                    color.g = 1.0;
                    starting_dir = Direction::Left;
                }
                3 => {
                    position.y = y;
                    color.b = 1.0;
                    color.g = 1.0;
                    starting_dir = Direction::Down;
                }
                4 => {
                    position.y = SCREEN_HEIGHT-y-GRID_SIZE;
                    color.b = 1.0;
                    color.r = 1.0;
                    starting_dir = Direction::Up;
                }
                _ => ()
            }

            players.push(
                Player::new(ctx, &cacher, i, position, color, starting_dir)?
            );
        }
        
        Ok(MainState {
            background: Background::new(ctx, &cacher)?,
            players: players,
            win_text: None,
            win_text_offset: None,
            cacher: cacher
        })
    }

    fn check_win(&mut self, ctx: &mut Context) {
        if self.players.iter().map(|player| player.dead as u8).sum::<u8>() >= player::PLAYER_COUNT-1 {
            for player in self.players.iter_mut() {
                player.paused = true;

                if !player.dead {
                    let win_text = Text::new((format!("player {} won!", player.number), self.cacher.font, 60.0));
                    self.win_text_offset = Some(Vec2::new(
                        (SCREEN_WIDTH/2.0) - (win_text.width(ctx)/2.0),
                        (SCREEN_HEIGHT/2.0) - (win_text.height(ctx)/2.0)
                    ));
                    self.win_text = Some(win_text);
                }
            }
        }
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, player::DRIVES_PER_SECOND) {
            let all_prev_positions: Vec<HashSet<Vec2>> = self.players.iter().map(|player| player.prev_positions.clone()).collect();
            for player in self.players.iter_mut() {
                player.update(ctx, &self.cacher, &all_prev_positions)?;
            }

            self.check_win(ctx);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, background::BG_COLOR);
        
        for player in self.players.iter_mut() {
            player.draw_before(ctx, &self.cacher)?;
        }
        for player in self.players.iter_mut() {
            player.draw_after(ctx, &self.cacher)?;
        }
        
        self.background.draw(ctx, &self.cacher)?;

        match self.win_text.as_mut() {
            Some(text) =>
                graphics::draw(ctx, text, (self.win_text_offset.unwrap(),))?,
            None => ()
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, b: bool) {
        if key == self.cacher.keybinds.general[&Action::Restart] {
            *self = MainState::new(ctx).unwrap();
        }

        for player in self.players.iter_mut() {
            player.key_down_event(ctx, key, mods, b, &self.cacher);
        }
    }
}


pub fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("tron", "killbottt")
        .add_resource_path(resource_dir)
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT));

    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}