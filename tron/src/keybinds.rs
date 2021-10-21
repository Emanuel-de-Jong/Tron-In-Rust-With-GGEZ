use std::collections::HashMap;
use ggez::event::KeyCode;
use tron::*;

pub struct Keybinds {
    pub general: HashMap<Action, KeyCode>,
    pub player1: HashMap<Direction, KeyCode>,
    pub player2: HashMap<Direction, KeyCode>
}

impl Keybinds {
    pub fn default() -> Keybinds {
        let mut general = HashMap::new();
        general.insert(Action::Restart, KeyCode::Escape);

        let mut player1 = HashMap::new();
        player1.insert(Direction::Left, KeyCode::A);
        player1.insert(Direction::Right, KeyCode::D);
        player1.insert(Direction::Up, KeyCode::W);
        player1.insert(Direction::Down, KeyCode::S);

        let mut player2 = HashMap::new();
        player2.insert(Direction::Left, KeyCode::G);
        player2.insert(Direction::Right, KeyCode::J);
        player2.insert(Direction::Up, KeyCode::Y);
        player2.insert(Direction::Down, KeyCode::H);

        Keybinds {
            general: general,
            player1: player1,
            player2: player2
        }
    }

    pub fn player(&self, name: &str) -> &HashMap<Direction, KeyCode> {
        if name == "player1" {
            return &self.player1;
        }
        else if name == "player2" {
            return &self.player2;
        }
        &self.player1
    }
}