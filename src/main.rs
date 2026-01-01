use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::prelude::*;
use macroquad::rand::srand;
use macroquad::ui::{hash, root_ui, widgets};

struct Body {
    position: Vec2,
    velocity: Vec2,
}

struct State {
    bodies: Vec<Body>,
    frame_rates: Vec<f32>,
}

impl State {
    fn new(num_bodies: u32) -> Self {
        let unix_epoch_diff = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap()).as_secs();
        srand(unix_epoch_diff);

        pretty_env_logger::init();
        let mut bodies = Vec::new();
        for _ in 0..num_bodies {
            bodies.push(Body {
                position: Vec2::new(
                    rand::gen_range(0.0, screen_width()),
                    rand::gen_range(0.0, screen_height()),
                ),
                velocity: Vec2::new(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)),
            });
        }

        Self {
            bodies,
            frame_rates: Vec::new(),
        }
    }
    fn frame(&mut self) {
        clear_background(BLACK);

        if self.frame_rates.len() == 5 {
            self.frame_rates = Vec::new();
        }
        self.frame_rates.push(1.0f32 / get_frame_time());

        self.draw_ui();

        for body in self.bodies.iter_mut() {
            // Euler integration
            body.position += body.velocity;
            draw_circle(body.position.x, body.position.y, 2.5, LIGHTGRAY);
        }
    }
    /// Draws the user interface for modifying values and seeing bodies.
    fn draw_ui(&mut self) {
        widgets::Window::new(hash!(), vec2(20.0, 20.0), vec2(150.0, 200.0))
            .label("Simulation Info")
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, format!("Body Count: {}", self.bodies.len()).as_str());
                // compute frame time average
                // debug!("Frame Times: {:#?}", self.frame_rates);
                let frame_rate_avg: f32 =
                    self.frame_rates.iter().sum::<f32>() / self.frame_rates.len() as f32;
                ui.label(
                    None,
                    format!("Framerate: {}", frame_rate_avg.round()).as_str(),
                );
            });
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
