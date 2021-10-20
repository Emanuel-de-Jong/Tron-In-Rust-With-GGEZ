use std::collections::HashMap;
use ggez::event::KeyCode;
use tron::*;

pub struct Keybinds {
}

impl Keybinds {
    pub fn general() -> HashMap<Action, KeyCode> {
        let mut general = HashMap::new();
        general.insert(Action::Restart, KeyCode::Escape);
        general
    }

    pub fn player(name: &str) -> HashMap<Direction, KeyCode> {
        if name == "player1" {
            let mut player = HashMap::new();
            player.insert(Direction::Left, KeyCode::A);
            player.insert(Direction::Right, KeyCode::D);
            player.insert(Direction::Up, KeyCode::W);
            player.insert(Direction::Down, KeyCode::S);
            return player;
        }
        else if name == "player2" {
            let mut player = HashMap::new();
            player.insert(Direction::Left, KeyCode::G);
            player.insert(Direction::Right, KeyCode::J);
            player.insert(Direction::Up, KeyCode::Y);
            player.insert(Direction::Down, KeyCode::H);
            return player;
        }

        HashMap::new()
    }
}