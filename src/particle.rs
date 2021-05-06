use macroquad::prelude::*;
use super::boundary::Boundary;
use super::ray::Ray;

pub struct Particle {
    pub position: Vec2,
    pub rays: Vec<Ray>,
}
impl Particle {
    pub fn new(pos: Vec2) -> Particle {
        let mut ray_list: Vec<Ray> = vec![];

        for i in (0..360).step_by(1) {
            ray_list.push(Ray::new(pos, vec2_from_angle(i as f32)));
        }

        Particle {
            position: pos,
            rays: ray_list,
        }
    }
    pub fn render(&self) {
        draw_circle(self.position.x, self.position.y, 5., WHITE);
        for i in 0..self.rays.len() {
            self.rays[i].render();
        }
    }
    pub fn look(&self, bound: &mut Vec<Boundary>) {
        for i in 0..self.rays.len() {
            let mut closest: Vec2 = vec2(0., 0.);
            let mut record = f32::MAX;
            for j in 0..bound.len() {
                let pt: Vec2 = self.rays[i].cast(bound[j]);

                if pt != vec2(0., 0.) {
                    let d: f32 = distance(self.position, pt);
                    if d < record {
                        record = d;
                        closest = pt;
                    }
                }
            }
            if closest != vec2(0., 0.) {
                draw_line(
                    self.position.x,
                    self.position.y,
                    closest.x,
                    closest.y,
                    0.5,
                    Color{
                        r:255.,
                        g:255.,
                        b:255.,
                        a:75.
                    },
                );
            }
        }
    }
    pub fn update(&mut self, m_pos: (f32, f32)) {
        for ray in self.rays.iter_mut() {
            ray.position = self.position;
        }
        self.position.x = m_pos.0;
        self.position.y = m_pos.1;
    }
}
fn vec2_from_angle(angle: f32) -> Vec2 {
    let rad: f32 = (22. / 7.) * angle / 180.0;
    let mut vec: Vec2 = vec2(0., 0.);
    vec.x = f32::cos(rad);
    vec.y = f32::sin(rad);
    vec
}
fn distance(vector1: Vec2, vector2: Vec2) -> f32 {
    f32::sqrt(
        (vector1.x - vector2.x) * (vector1.x - vector2.x)
            + (vector1.y - vector2.y) * (vector1.y - vector2.y),
    )
}
