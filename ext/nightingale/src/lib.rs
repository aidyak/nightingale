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

pub fn hello(subject: String) -> String {
    format!("Hello {subject}, from Rust!")
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
