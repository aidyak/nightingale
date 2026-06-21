use lazy_static::lazy_static;
use macroquad::prelude::*;
use magnus::{function, Error, Ruby, Symbol};
use std::sync::Mutex;

struct EngineState {
    x: f32,
    y: f32,
    facing_direction: FacingDirection,
    pressed_left: bool,
    pressed_right: bool,
    pressed_up: bool,
    pressed_down: bool,
}

#[derive(Clone, Copy)]
enum FacingDirection {
    Left,
    Right,
    Up,
    Down,
}

lazy_static! {
    static ref STATE: Mutex<EngineState> = Mutex::new(EngineState {
        x: 100.0,
        y: 100.0,
        facing_direction: FacingDirection::Right,
        pressed_left: false,
        pressed_right: false,
        pressed_up: false,
        pressed_down: false
    });
}

fn update_position(x: f32, y: f32) {
    let mut state = STATE.lock().unwrap();
    state.x = x as f32;
    state.y = y as f32;

    if state.x > 800.0 {
        state.x = 0.0;
    }
    if state.y > 600.0 {
        state.y = 0.0;
    }
}

fn is_key_down_ruby(key_symbol: Symbol) -> bool {
    let state = STATE.lock().unwrap();
    let key_str = key_symbol.to_string();
    match key_str.as_str() {
        "up" => state.pressed_up,
        "down" => state.pressed_down,
        "left" => state.pressed_left,
        "right" => state.pressed_right,
        _ => false,
    }
}

fn draw_player(x: f32, y: f32, facing_direction: FacingDirection) {
    let body = Color::from_rgba(220, 74, 58, 255);
    let wing = Color::from_rgba(84, 132, 183, 255);
    let belly = Color::from_rgba(255, 214, 170, 255);
    let beak = Color::from_rgba(245, 180, 66, 255);

    match facing_direction {
        FacingDirection::Up | FacingDirection::Down => {
            let direction = if matches!(facing_direction, FacingDirection::Up) {
                -1.0
            } else {
                1.0
            };
            let body_center = vec2(x + 25.0, y + 29.0);
            let head_center = vec2(body_center.x, body_center.y + direction * 16.0);
            let beak_tip = vec2(head_center.x, head_center.y + direction * 18.0);

            draw_circle(body_center.x, body_center.y, 21.0, body);
            draw_circle(body_center.x, body_center.y + direction * 5.0, 12.0, belly);
            draw_circle(head_center.x, head_center.y, 13.0, body);
            draw_triangle(
                vec2(head_center.x - 5.0, head_center.y + direction * 10.0),
                vec2(head_center.x + 6.0, head_center.y + direction * 10.0),
                beak_tip,
                beak,
            );
            draw_circle(
                head_center.x + 4.0,
                head_center.y - direction * 5.0,
                2.5,
                BLACK,
            );

            draw_triangle(
                vec2(body_center.x - 9.0, body_center.y - direction * 2.0),
                vec2(body_center.x + 13.0, body_center.y - direction * 4.0),
                vec2(body_center.x + 2.0, body_center.y - direction * 27.0),
                wing,
            );
            draw_triangle(
                vec2(body_center.x - 11.0, body_center.y - direction * 22.0),
                vec2(body_center.x - 20.0, body_center.y - direction * 38.0),
                vec2(body_center.x + 2.0, body_center.y - direction * 32.0),
                body,
            );
        }
        FacingDirection::Left | FacingDirection::Right => {
            let direction = if matches!(facing_direction, FacingDirection::Right) {
                1.0
            } else {
                -1.0
            };
            let body_center = vec2(x + 25.0, y + 29.0);
            let head_center = vec2(x + 25.0 + direction * 14.0, y + 16.0);
            let beak_tip = vec2(head_center.x + direction * 18.0, head_center.y + 1.0);

            draw_circle(body_center.x, body_center.y, 21.0, body);
            draw_circle(
                body_center.x + direction * 5.0,
                body_center.y + 6.0,
                12.0,
                belly,
            );
            draw_circle(head_center.x, head_center.y, 13.0, body);
            draw_triangle(
                vec2(head_center.x + direction * 10.0, head_center.y - 5.0),
                vec2(head_center.x + direction * 10.0, head_center.y + 6.0),
                beak_tip,
                beak,
            );
            draw_circle(
                head_center.x + direction * 5.0,
                head_center.y - 4.0,
                2.5,
                BLACK,
            );

            draw_triangle(
                vec2(body_center.x - direction * 2.0, body_center.y - 9.0),
                vec2(body_center.x - direction * 4.0, body_center.y + 13.0),
                vec2(body_center.x - direction * 27.0, body_center.y + 2.0),
                wing,
            );
            draw_triangle(
                vec2(x + 6.0 - direction * 2.0, y + 31.0),
                vec2(x - direction * 16.0, y + 20.0),
                vec2(x - direction * 10.0, y + 42.0),
                body,
            );
        }
    }
}

fn state_engine() {
    macroquad::Window::new("Nightingale Play Game Engine", async {
        loop {
            let mut state = STATE.lock().unwrap();
            state.pressed_left = is_key_down(KeyCode::Left) || is_key_down(KeyCode::A);
            state.pressed_right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D);
            state.pressed_up = is_key_down(KeyCode::Up) || is_key_down(KeyCode::W);
            state.pressed_down = is_key_down(KeyCode::Down) || is_key_down(KeyCode::S);

            let speed = 4.0;
            if state.pressed_left {
                state.x -= speed;
                state.facing_direction = FacingDirection::Left;
            }
            if state.pressed_right {
                state.x += speed;
                state.facing_direction = FacingDirection::Right;
            }
            if state.pressed_up {
                state.y -= speed;
                state.facing_direction = FacingDirection::Up;
            }
            if state.pressed_down {
                state.y += speed;
                state.facing_direction = FacingDirection::Down;
            }

            state.x = state.x.clamp(0.0, 750.0);
            state.y = state.y.clamp(0.0, 550.0);

            clear_background(BLACK);
            draw_player(state.x, state.y, state.facing_direction);
            drop(state);

            next_frame().await;
        }
    })
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    ruby.define_global_function("start_game_engine", function!(state_engine, 0));
    ruby.define_global_function("update_box_position", function!(update_position, 2));
    ruby.define_global_function("key_down?", function!(is_key_down_ruby, 1));
    Ok(())
}
