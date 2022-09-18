mod dot;

use dot::Dot;
use raylib::prelude::*;

const WIDTH: i32 = 600;
const HEIGHT: i32 = 600;
const SQUARE_SIZE: i32 = 400;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Random Pi")
        .build();

    let mut rng = rand::thread_rng();

    // let mut dots: Vec<Dot> = Vec::new();
    let mut dots: i32 = 0;
    let mut inside_dots: i32 = 0;

    let mut first_draw = true;
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        if first_draw {
            d.clear_background(Color::BLACK);
        }

        for _ in 0..10000 {
            let new_dot = Dot::random(&mut rng);
            dots += 1;
            if new_dot.is_inside() {
                inside_dots += 1;
            }
            new_dot.draw(&mut d);

            // dots.push(new_dot);
        }

        // d.draw_circle_lines(
        //     WIDTH / 2,
        //     HEIGHT / 2,
        //     (SQUARE_SIZE / 2) as f32,
        //     Color::WHITE,
        // );
        // d.draw_rectangle_lines(
        //     WIDTH / 2 - SQUARE_SIZE / 2,
        //     HEIGHT / 2 - SQUARE_SIZE / 2,
        //     SQUARE_SIZE,
        //     SQUARE_SIZE,
        //     Color::WHITE,
        // );

        let pi = inside_dots as f32 / dots as f32 * 4.;

        let stats = format!("Dots: {}\nInside Dots: {}\nPI = {}", dots, inside_dots, pi);
        d.draw_rectangle(0, 0, WIDTH / 2, HEIGHT / 2 - SQUARE_SIZE / 2, Color::BLACK);
        d.draw_text(&stats, 20, 20, 20, Color::WHITE);

        // for dot in &dots {
        //     dot.draw(&mut d);
        // }
        first_draw = false;
    }
}
