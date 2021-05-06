use macroquad::prelude::*;
use super::boundary::Boundary;

pub struct Ray {
    pub position: Vec2,
    pub direction: Vec2,
}

impl Ray {
    pub fn new(pos: Vec2, dir: Vec2) -> Ray {
        Ray {
            position: pos,
            direction: dir,
        }
    }
    pub fn render(&self) {
        draw_line(
            self.position.x,
            self.position.y,
            self.position.x + self.direction.x,
            self.position.y + self.direction.y,
            1.,
            WHITE,
        );
    }
    pub fn cast(&self, bound: Boundary) -> Vec2 {
        let x1 = bound.a.x;
        let y1 = bound.a.y;
        let x2 = bound.b.x;
        let y2 = bound.b.y;

        let x3 = self.position.x;
        let y3 = self.position.y;
        let x4 = self.position.x + self.direction.x;
        let y4 = self.position.y + self.direction.y;

        let den: f32 = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if den == 0. {
            return vec2(0., 0.);
        }

        let t: f32 = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / den;
        let u: f32 = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / den;

        if t > 0. && t < 1. && u > 0. {
            let mut pt: Vec2 = vec2(0., 0.);
            pt.x = x1 + t * (x2 - x1);
            pt.y = y1 + t * (y2 - y1);
            return pt;
        } else {
            return vec2(0., 0.);
        }
    }
}
