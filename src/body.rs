use macroquad::math::Vec2;

use crate::GRAVITY_CONSTANT;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Body {
    pub id: u64,
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f32,
}

impl Body {
    pub fn new(position: Vec2, velocity: Vec2, mass: f32, id: u64) -> Self {
        Self {
            id,
            position,
            velocity,
            mass,
        }
    }

    // Weird equal thing necessary to get around Rust's strange type checking.
    pub fn equal(&self, other_body: &Body) -> bool {
        (self.position == other_body.position)
            && (self.velocity == other_body.velocity)
            && (self.mass == other_body.mass)
    }

    // Calculates the gravitational attraction force between two bodies.
    // Assumes each pixel is a meter and each mass is a kilogram.
    // Returns the resulting vector to add to the velocity of this body.
    pub fn calculate_force(&mut self, other_body: &Body) -> Vec2 {
        // Exit early if we're calculating force against ourself (div by 0)
        if self.equal(other_body) {
            return Vec2::new(0.0, 0.0);
        }

        let distance = other_body.position - self.position;

        let distance_squared = distance.length_squared();

        // Clamp the distances to stay within some limit to prevent bodies from flying into the
        // abyss.
        let distance_squared = distance_squared.max(crate::MINIMUM_DISTANCE);

        // Don't know how this works, see
        // https://github.com/octo-kumo/space-rs/blob/master/src/body.rs .
        let force = (GRAVITY_CONSTANT * self.mass * other_body.mass / distance_squared) * distance
            / distance_squared.sqrt();

        force
    }

    // Applies a force given from the `calculate_force` function
    pub fn apply_force(&mut self, force: Vec2, delta_t: f32) {
        self.velocity += force / self.mass * delta_t;
    }
}
