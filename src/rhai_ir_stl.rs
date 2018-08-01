extern crate rhai;

use rhai::RegisterFn;
use std::fmt::Display;

fn rhai_print<T: Display>(text: T) -> () {
    println!("{0}", text);
}

pub fn register_stl(engine: &mut rhai::Engine) -> () {

    engine.register_fn("print", rhai_print as fn(x: i32) -> ());
    engine.register_fn("print", rhai_print as fn(x: i64) -> ());
    engine.register_fn("print", rhai_print as fn(x: u32) -> ());
    engine.register_fn("print", rhai_print as fn(x: u64) -> ());
    engine.register_fn("print", rhai_print as fn(x: f32) -> ());
    engine.register_fn("print", rhai_print as fn(x: f64) -> ());
    engine.register_fn("print", rhai_print as fn(x: bool) -> ());
    engine.register_fn("print", rhai_print as fn(x: String) -> ());

}