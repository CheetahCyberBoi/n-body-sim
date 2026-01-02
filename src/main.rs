use std::ops::Range;
use std::time::{SystemTime, UNIX_EPOCH};

use macroquad::prelude::camera::mouse::Camera;
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
    camera: Camera,
    scale_factor: f32,
}

impl State {
    fn new(num_bodies: u32) -> Self {
        // seed the random number generator.
        // this is needed because macroquad doesn't seed it from the systemtime itself
        // lazy macroquad :(
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
                velocity: Vec2::new(rand::gen_range(-10.0, 10.0), rand::gen_range(-10.0, 10.0)),
            });
        }

        let camera = Camera::new(vec2(0.0, 0.0), 1.0);

        Self {
            bodies,
            frame_rates: Vec::new(),
            camera,
            scale_factor: 2.0,
        }
    }
    // Called every frame
    // TODO: Maybe split this into a physics update and a rendering update in the future?
    fn frame(&mut self) {
        clear_background(BLACK);

        // Update camera position and scroll
        // Scale around center of camera, NOT ORIGIN!!
        self.camera
            .scale_wheel(self.camera.offset, mouse_wheel().1, self.scale_factor);
        self.camera.update(
            mouse_position_local(),
            is_mouse_button_down(MouseButton::Right),
        );

        // Actually apply the camera
        let camera_2d: Camera2D = (&self.camera).into();
        set_camera(&camera_2d);

        if self.frame_rates.len() == 5 {
            self.frame_rates = Vec::new();
        }
        self.frame_rates.push(1.0f32 / get_frame_time());

        self.draw_ui();

        self.update_and_draw();

        // All physics and drawing happens here.
    }
    /// Update the positions of the bodies each frame.
    fn update_and_draw(&mut self) {
        for body in self.bodies.iter_mut() {
            // Euler integration.
            body.position += body.velocity * get_frame_time();
            // Draw the body.
            draw_circle(body.position.x, body.position.y, 2.5, LIGHTGRAY);
        }
    }
    /// Draws the user interface for modifying values and seeing bodies.
    fn draw_ui(&mut self) {
        // TODO: Make the window size dynamic based on the screen size
        widgets::Window::new(hash!(), vec2(20.0, 20.0), vec2(200.0, 200.0))
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
                ui.slider(
                    hash!(),
                    "Scale Factor",
                    Range {
                        start: 1.0,
                        end: 2.0,
                    },
                    &mut self.scale_factor,
                );
                ui.label(
                    None,
                    format!("Zoom: {}", 1.0f32 / self.camera.scale).as_str(),
                );
            });
    }
}

#[macroquad::main("N-Body Simulation")]
async fn main() {
    let mut state = State::new(1000);
    loop {
        state.frame();
        next_frame().await
    }
}
