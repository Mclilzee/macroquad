#![allow(unused)]

use macroquad::{
    prelude::*,
    rand::{ChooseRandom, RandomRange, gen_range, rand},
};

const PLAYER_WIDTH: f32 = 20.;
const PLAYER_HEIGHT: f32 = 50.;
const HALF_PLAYER_WIDTH: f32 = PLAYER_WIDTH / 2.;
const HALF_PLAYER_HEIGHT: f32 = PLAYER_HEIGHT / 2.;

struct Square {
    size: f32,
    x: f32,
    y: f32,
    speed: f32,
    color: Color,
    alive: bool,
}

#[macroquad::main("Simulation")]
async fn main() {
    show_mouse(false);
    set_fullscreen(true);

    // // let player_speed = 400.;
    // let mut points = vec![];
    // let mut squares: Vec<Square> = vec![];
    // // let mut player = Vec2::new(10., 10.);
    // let detect_radius = 500.0;
    // let point_detect_radius = 100.0;
    // let mut square_spawn_timer = 0.;
    // let rand = (0.0, 10.0);
    let mut anchor = None;
    let circle_size = 6.;
    loop {
        clear_background(GRAY);
        let (mx, my) = mouse_position();
        draw_circle(mx, my, circle_size, BLUE);
        if is_mouse_button_pressed(MouseButton::Left) {
            anchor = Some((mx, my));
        } else if is_mouse_button_pressed(MouseButton::Right) {
            anchor = None;
        }

        if let Some((px, py)) = anchor {
            draw_circle(px, py, circle_size, GREEN);
            let size = 200.;
            draw_rectangle_lines(px - size, py - size, size * 2., size * 2., 6., RED);
            draw_line(mx, my, px, py, 2., YELLOW);
            let mouse = Vec2::new(mx, my);
            let anchor = Vec2::new(px, py);
            let distance = mouse.distance(anchor);
            let font_size = 20.;
            draw_text(
                &format!("{:.2}", distance),
                mx - font_size / 2.,
                my + 20.,
                font_size,
                GREEN,
            );
        }

        //     let height = screen_height();
        //     squares.retain(|s| s.y < height && s.alive);
        //     square_spawn_timer += get_frame_time();
        //     if square_spawn_timer > 0.01 {
        //         square_spawn_timer = 0.;
        //         let size = RandomRange::gen_range(6., 30.);
        //         let speed = RandomRange::gen_range(100., 1000.);
        //         let x = RandomRange::gen_range(-size, screen_width());
        //         squares.push(Square {
        //             size,
        //             speed,
        //             x,
        //             y: 0.0,
        //             color: rand_color(),
        //             alive: true,
        //         });
        //     }
        //
        //     if is_key_pressed(KeyCode::R) {
        //         points = vec![];
        //     }
        //
        //     if is_key_pressed(KeyCode::Space) {
        //         break;
        //     }
        //
        //     if is_mouse_button_pressed(MouseButton::Right) && !points.is_empty() {
        //         points.remove(points.len() - 1);
        //     }
        //
        //     let r = 10.0;
        //     let d = r * 2.;
        //     let font_size = 30.0;
        //     draw_circle(mx, my, r, RED);
        //     draw_text(
        //         &format!("{mx}, {my}"),
        //         mx - d - font_size,
        //         my + r + font_size,
        //         font_size,
        //         WHITE,
        //     );
        //
        //     let mut draw_positions = Vec::new();
        //     for s in &mut squares {
        //         s.y += s.speed * get_frame_time();
        //         draw_rectangle(s.x, s.y, s.size, s.size, s.color);
        //         let sx = s.x + s.size / 2.;
        //         let sy = s.y + s.size / 2.;
        //         if (sx - mx).abs() <= detect_radius && (sy - my).abs() <= detect_radius {
        //             draw_positions.push((sx, sy));
        //         }
        //     }
        //
        //     draw_positions.sort_by(|a, b| {
        //         ((a.0 - mx).abs() + (a.1 - my).abs())
        //             .partial_cmp(&((b.0 - mx).abs() + (b.1 - my).abs()))
        //             .unwrap()
        //     });
        //
        //     for (px, py) in draw_positions.into_iter().take(20) {
        //         draw_line(px, py, mx, my, r / 4., GREEN);
        //     }
        //
        //     for (x, y) in &points {
        //         let mut count = 0;
        //         for s in &mut squares {
        //             if (x - s.x).abs() <= point_detect_radius && (y - s.y).abs() <= point_detect_radius
        //             {
        //                 count += 1;
        //                 draw_line(*x, *y, s.x, s.y, r / 4., GREEN);
        //             }
        //
        //             if count > 2 {
        //                 break;
        //             }
        //         }
        //     }
        //
        //     draw_fps();
        //     draw_text(
        //         &format!("SQUARES: {}", squares.len()),
        //         200.,
        //         20.,
        //         font_size,
        //         GREEN,
        //     );
        //     draw_text(
        //         &format!("W: {}, H: {}", screen_width(), height),
        //         600.,
        //         20.,
        //         font_size,
        //         GREEN,
        //     );
        next_frame().await
    }
}

fn rand_color() -> Color {
    Color::from_rgba(rand_u8(), rand_u8(), rand_u8(), 255)
}

fn rand_u8() -> u8 {
    gen_range(0, 255) as u8
}
