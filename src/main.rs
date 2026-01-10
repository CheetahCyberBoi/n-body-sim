use macroquad::prelude::*;
use n_body_sim::State;

#[macroquad::main("N-Body Simulation")]
async fn main() {
    let mut state = State::new(100);
    loop {
        state.frame();
        next_frame().await
    }
}
