use lazy_static::lazy_static;
use macroquad::prelude::*;
use magnus::{define_global_function, function, prelude::*, Error, Ruby};
use std::sync::Mutex;

struct EngineState {
    x: f32,
    y: f32,
}

lazy_static! {
    static ref STATE: Mutex<EngineState> = Mutex::new(EngineState { x: 100.0, y: 100.0 });
}

fn update_position(dx: f32, dy: f32) {
    let mut state = STATE.lock().unwrap();
    state.x += dx as f32;
    state.y += dy as f32;

    if state.x > 800.0 {
        state.x = 0.0;
    }
    if state.y > 600.0 {
        state.y = 0.0;
    }
}

fn state_engine() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        request_new_screen_size(800, 600);
        loop {
            clear_background(WHITE);
            let state = STATE.lock().unwrap();
            draw_circle(state.x, state.y, 50.0, 50.0, RED);
            drop(state);
            next_frame().await;
        }
    });
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Nightingale")?;
    module.define_singleton_method("hello", function!(hello, 1))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::hello;
    use rb_sys_test_helpers::ruby_test;

    #[ruby_test]
    fn test_hello() {
        assert_eq!("Hello world, from Rust!", hello("world".to_string()));
    }
}
