use std::hash::{Hash, Hasher};
use ggez::mint::Point2;

#[derive(PartialEq, Clone, Copy)]
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
    fn from(vec: Vec2) -> Point2<f32> {
        Point2 { x: vec.x, y: vec.y }
    }
}