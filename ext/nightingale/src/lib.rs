use lazy_static::lazy_static;
use macroquad::prelude::*;
use magnus::{function, Error, Ruby, Symbol};
use std::sync::Mutex;

struct EngineState {
    x: f32,
    y: f32,
    pressed_left: bool,
    pressed_right: bool,
    pressed_up: bool,
    pressed_down: bool,
}

lazy_static! {
    static ref STATE: Mutex<EngineState> = Mutex::new(EngineState {
        x: 100.0,
        y: 100.0,
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

fn state_engine() {
    macroquad::Window::new("Nightingale Game Engine", async {
        loop {
            let mut state = STATE.lock().unwrap();
            state.pressed_left = is_key_down(KeyCode::Left) || is_key_down(KeyCode::A);
            state.pressed_right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D);
            state.pressed_up = is_key_down(KeyCode::Up) || is_key_down(KeyCode::W);
            state.pressed_down = is_key_down(KeyCode::Down) || is_key_down(KeyCode::S);

            let speed = 4.0;
            if state.pressed_left {
                state.x -= speed;
            }
            if state.pressed_right {
                state.x += speed;
            }
            if state.pressed_up {
                state.y -= speed;
            }
            if state.pressed_down {
                state.y += speed;
            }

            state.x = state.x.clamp(0.0, 750.0);
            state.y = state.y.clamp(0.0, 550.0);

            clear_background(BLACK);
            draw_rectangle(state.x, state.y, 50.0, 50.0, RED);
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
