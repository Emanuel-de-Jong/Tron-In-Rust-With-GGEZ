mod background;
mod player;
mod keybinds;
mod vec2;

use keybinds::Keybinds;
use tron::*;
use background::Background;
use player::Player;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, KeyCode, KeyMods};
use std::{env, path};

struct MainState {
    background: Background,
    player: Player
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState {
            background: Background::new(ctx)?,
            player: Player::new(ctx, "player1".into(), Color::new(1.0, 1.0, 1.0, 1.0))?
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.player.update(ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, background::BG_COLOR);
        
        self.player.draw(ctx)?;
        self.background.draw(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, b: bool) {
        let keybinds = Keybinds::general();
        if key == keybinds[&Action::Restart] {

        }

        self.player.key_down_event(ctx, key, mods, b);
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