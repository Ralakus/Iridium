pub extern crate iridium_core as core;
pub extern crate iridium_rhai as script;

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        let mut engine = script::rhai::Engine::new();

        script::script_stl::register_stl(&mut engine);

        if let Ok(result) = engine.eval::<i64>("40 + 5") {
            println!("Result: {0}", result);
        }

        let ir_error = core::error::IridiumError::new(String::from("Something"));

        println!("Iridium error: {0}", ir_error);

    }
}