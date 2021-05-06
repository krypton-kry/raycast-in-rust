use macroquad::prelude::*;

mod boundary;
mod particle;
mod ray;

const WINDOW_HEIGHT: i32 = 480;
const WINDOW_WIDTH: i32 = 640;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("raycast"),
        window_height: WINDOW_HEIGHT,
        window_width: WINDOW_WIDTH,
        window_resizable: false,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut walls: Vec<boundary::Boundary> = vec![];
    for _i in 0..7 {
        walls.push(boundary::Boundary::new(
            vec2(
                rand::gen_range(0., WINDOW_WIDTH as f32),
                rand::gen_range(0., WINDOW_HEIGHT as f32),
            ),
            vec2(
                rand::gen_range(0., WINDOW_WIDTH as f32),
                rand::gen_range(0., WINDOW_HEIGHT as f32),
            ),
        ));
    }
    //borders
    walls.push(boundary::Boundary::new(vec2(0., 0.), vec2(WINDOW_WIDTH as f32, 0.)));
    walls.push(boundary::Boundary::new(
        vec2(WINDOW_WIDTH as f32, 0.),
        vec2(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
    ));
    walls.push(boundary::Boundary::new(
        vec2(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
        vec2(0., WINDOW_HEIGHT as f32),
    ));
    walls.push(boundary::Boundary::new(vec2(0., WINDOW_HEIGHT as f32), vec2(0., 0.)));

    let mut par = particle::Particle::new(vec2(WINDOW_WIDTH as f32 / 2., WINDOW_HEIGHT as f32 / 2.));

    loop {
        clear_background(BLACK);
        for i in 0..walls.len() {
            walls[i].render();
        }
        par.render();
        par.look(&mut walls);
        par.update(mouse_position());

        next_frame().await;
    }
}
