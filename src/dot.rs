use rand::{rngs::ThreadRng, Rng};
use raylib::prelude::*;

use crate::{HEIGHT, SQUARE_SIZE, WIDTH};

#[derive(Clone, Copy)]
pub struct Dot {
    x: i32,
    y: i32,
}

impl Dot {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(rng: &mut ThreadRng) -> Self {
        Self::new(rng.gen_range(0..SQUARE_SIZE), rng.gen_range(0..SQUARE_SIZE))
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        let color = if self.is_inside() {
            Color::GREEN
        } else {
            Color::RED
        };

        d.draw_pixel(
            WIDTH / 2 - SQUARE_SIZE / 2 + self.x,
            HEIGHT / 2 - SQUARE_SIZE / 2 + self.y,
            color,
        );
    }

    pub fn is_inside(&self) -> bool {
        let dist_x = SQUARE_SIZE / 2 - self.x;
        let dist_y = SQUARE_SIZE / 2 - self.y;
        let dist = ((dist_x.pow(2) + dist_y.pow(2)) as f32).sqrt();
        return dist <= (SQUARE_SIZE / 2) as f32;
    }
}
