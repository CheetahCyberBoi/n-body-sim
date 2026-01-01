use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::prelude::*;
use macroquad::rand::srand;

struct Body {
    position: Vec2,
}

struct State {
    bodies: Vec<Body>,
}

impl State {
    fn new(num_bodies: u32) -> Self {
        let unix_epoch_diff = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap()).as_secs();
        srand(unix_epoch_diff);

        pretty_env_logger::init();
        let mut bodies = Vec::new();
        for _ in 0..num_bodies - 1 {
            bodies.push(Body {
                position: Vec2::new(
                    rand::gen_range(0.0, screen_width()),
                    rand::gen_range(0.0, screen_height()),
                ),
            });
        }

        Self { bodies }
    }
    fn frame(&mut self) {
        clear_background(BLACK);

        for body in self.bodies.iter() {
            draw_circle(body.position.x, body.position.y, 2.5, LIGHTGRAY);
        }
    }
}

#[macroquad::main("N-Body Simulation")]
async fn main() {
    let mut state = State::new(100);
    loop {
        state.frame();
        next_frame().await
    }
}
