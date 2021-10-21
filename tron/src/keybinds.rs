use tron::*;
use ggez::event::KeyCode;
use std::collections::HashMap;


pub struct Keybinds {
    pub general: HashMap<Action, KeyCode>,
    players: Vec<HashMap<Direction, KeyCode>>
}

impl Keybinds {
    pub fn default() -> Keybinds {
        let mut general = HashMap::new();
        general.insert(Action::Restart, KeyCode::R);

        let mut player1 = HashMap::new();
        player1.insert(Direction::Left, KeyCode::A);
        player1.insert(Direction::Right, KeyCode::D);
        player1.insert(Direction::Up, KeyCode::W);
        player1.insert(Direction::Down, KeyCode::S);

        let mut player2 = HashMap::new();
        player2.insert(Direction::Left, KeyCode::F);
        player2.insert(Direction::Right, KeyCode::H);
        player2.insert(Direction::Up, KeyCode::T);
        player2.insert(Direction::Down, KeyCode::G);

        let mut player3 = HashMap::new();
        player3.insert(Direction::Left, KeyCode::J);
        player3.insert(Direction::Right, KeyCode::L);
        player3.insert(Direction::Up, KeyCode::I);
        player3.insert(Direction::Down, KeyCode::K);

        let mut player4 = HashMap::new();
        player4.insert(Direction::Left, KeyCode::Left);
        player4.insert(Direction::Right, KeyCode::Right);
        player4.insert(Direction::Up, KeyCode::Up);
        player4.insert(Direction::Down, KeyCode::Down);

        Keybinds {
            general: general,
            players: vec![player1, player2, player3, player4]
        }
    }

    pub fn player(&self, number: u8) -> &HashMap<Direction, KeyCode> {
        &self.players[(number-1) as usize]
    }
}