use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Boundary {
    pub a: Vec2,
    pub b: Vec2,
}

impl Boundary {
    pub fn new(point_a: Vec2, point_b: Vec2) -> Boundary {
        Boundary {
            a: point_a,
            b: point_b,
        }
    }
    pub fn render(&self) {
        draw_line(self.a.x, self.a.y, self.b.x, self.b.y, 1., WHITE);
    }
}