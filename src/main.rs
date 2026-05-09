#![allow(unused)]

use macroquad::{
    prelude::*,
    rand::{ChooseRandom, RandomRange, gen_range, rand},
};

const THICKNESS: f32 = 6.;
const SIZE: f32 = 200.;
const FONT_SIZE: f32 = 20.;
const FONT_PADDING: f32 = 20.;
const CIRCLE_SIZE: f32 = 6.;
const COLUMNS: u16 = 20;
const ROWS: u16 = 20;

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

    let mut scale = 1.;
    let mut anchor = None;
    loop {
        clear_background(GRAY);
        draw_lines(scale);
        let (_, scroll) = mouse_wheel();
        if (scroll != 0.) {
            scale += (scroll * 0.1);
            scale = scale.clamp(0.1, 20.);
        }
        let font_size = FONT_SIZE * scale;
        let font_padding = FONT_PADDING * scale;
        let thickness = THICKNESS * scale;
        let size = SIZE * scale;
        let circle_size = CIRCLE_SIZE * scale;
        let (mx, my) = mouse_position();
        draw_circle(mx, my, circle_size, BLUE);
        if is_mouse_button_pressed(MouseButton::Left) {
            anchor = Some((mx, my));
        } else if is_mouse_button_pressed(MouseButton::Right) {
            anchor = None;
        }

        if let Some((px, py)) = anchor {
            draw_circle(px, py, circle_size, GREEN);
            draw_rectangle_lines(px - size, py - size, size * 2., size * 2., thickness, RED);
            draw_circle_lines(px, py, size - thickness, thickness, PINK);
            draw_line(mx, my, px, py, thickness, YELLOW);
            let mouse = Vec2::new(mx, my);
            let anchor = Vec2::new(px, py);
            let distance = mouse.distance(anchor);
            draw_text(
                &format!("{:.2}", distance),
                mx - font_size / 2.,
                my + font_padding,
                font_size,
                GREEN,
            );
        }

        next_frame().await
    }
}

fn draw_lines(scale: f32) {
    let width = screen_width();
    let height = screen_height();
    let stride = 80. * scale;
    assert_ne!(stride, 0.);
    let columns = (width / stride) as u32 + 1;
    let rows = (height / stride) as u32 + 1;

    for i in 0..=columns {
        let x = i as f32 * stride as f32;
        draw_line(x, 0., x, height, 2., BLACK);
    }

    for i in 0..=rows {
        let y = i as f32 * stride as f32;
        draw_line(0., y, width, y, 2., BLACK);
    }
}
