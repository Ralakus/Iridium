pub extern crate iridium_core     as core;
pub extern crate iridium_rhai     as script;
pub extern crate iridium_graphics as graphics;
pub extern crate iridium_audio    as audio;
pub extern crate iridium_physics  as physics;

#[cfg(test)]
mod test {

    use super::*;

    struct TestState {
        val1: i32
    }

    impl core::IridiumState for TestState {
        fn awake(&mut self)              -> Result<(), core::IridiumError> {
            println!("State awoken!");
            Ok(())
        }
        fn update(&mut self, delta: f32) -> Result<(), core::IridiumError>{
            println!("State updated with a delta of {0}", delta);
            if self.val1 < 0 {
                return Err(core::IridiumError::new(String::from("It works!")));
            }
            Ok(())
        }
        fn end(&mut self)                -> Result<(), core::IridiumError>{
            println!("State ended!");
            Ok(())
        }

        fn handle_event(&mut self, event: core::IridiumEvent) -> Result<(), core::IridiumError>{
            println!("Event {0:?} recieved", event);
            Ok(())
        }

        fn get_name(&self) -> String {
            String::from("TestState")
        }
    }

    #[test]
    fn rhai_test() {
        let mut engine = script::rhai::Engine::new();

        script::register_stl(&mut engine);

        if let Ok(result) = engine.eval::<i64>("40 + 5") {
            println!("Result: {0}", result);
        }

    }
    
    #[test]
    fn state_test () {
        let mut state_manager = core::StateManager::new();
        let test_state = TestState {val1: 0};

        state_manager.register_state(test_state).expect("Error");
        state_manager.set_next_state(&String::from("TestState")).expect("Error");
        state_manager.switch_to_next_state().expect("Error");

        state_manager.update_state(0.0032f32).expect("Error");
        state_manager.send_event(core::IridiumEvent::Close).expect("Error");
        state_manager.end_state().expect("Error");

    }
}