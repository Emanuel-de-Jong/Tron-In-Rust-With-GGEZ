use ggez::mint::Point2;
use std::hash::{Hash, Hasher};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x: x, y: y }
    }
}

impl Eq for Vec2 {}

impl Hash for Vec2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x as u32).hash(state);
        (self.y as u32).hash(state);
    }
}

impl From<Vec2> for Point2<f32> {
    fn from(vec2: Vec2) -> Point2<f32> {
        Point2 { x: vec2.x, y: vec2.y }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}