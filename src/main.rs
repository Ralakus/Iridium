extern crate rhai;

mod rhai_ir_stl;
use rhai_ir_stl as ir;

mod error;

fn main() {

    let mut rhai_engine = rhai::Engine::new();

    ir::register_stl(&mut rhai_engine);

    if let Err(e) = rhai_engine.consume_file("Test.rhai"){
        println!("Error: {0}", e);
    }

    if let Ok(result) = rhai_engine.eval::<i64>("add(5, 3)") {
        println!("Result: {0}", result);
    }

}