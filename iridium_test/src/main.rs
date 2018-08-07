extern crate iridium;

struct TestShared {
    val: i32
}

struct TestState {
    val1: i32
}

impl iridium::core::IridiumState for TestState {
    fn awake(&mut self,shared_data: &mut Option<Box<std::any::Any>>) -> Result<(), iridium::core::IridiumError> {

        match shared_data.as_mut() {
            Some(d) => { 
                match d.downcast_ref::<TestShared>() {
                    Some(data) => {
                        self.val1 = data.val;
                    },
                    None => (),
                }
            },
            None => (),
        }

        println!("State awoken!");
        Ok(())
    }
    fn update(&mut self, delta: f32)                                 -> Result<(), iridium::core::IridiumError>{
        println!("State updated with a delta of {0}", delta);
        if self.val1 < 0 {
            return Err(iridium::core::IridiumError::new(String::from("It works!")));
        }
        Ok(())
    }
    fn end(&mut self)                                                -> Result<(), iridium::core::IridiumError>{
        println!("State ended!");
        Ok(())
    }

    fn handle_event(&mut self, event: iridium::core::IridiumEvent)   -> Result<(), iridium::core::IridiumError>{
        println!("Event {0:?} recieved", event);
        Ok(())
    }

    fn get_name(&self) -> String {
        String::from("TestState")
    }
}

fn main() {
    let mut state_manager = iridium::core::StateManager::new();
    let test_state  = TestState  {val1: 0};
    let test_shared = TestShared {val: -0};

    state_manager.set_shared_data(Box::new(test_shared));

    state_manager.register_state(test_state).unwrap();
    state_manager.set_next_state(&String::from("TestState")).unwrap();
    state_manager.switch_to_next_state().unwrap();

    state_manager.update_state(0.0032f32).unwrap();
    state_manager.send_event(iridium::core::IridiumEvent::Close).unwrap();
    state_manager.end_state().unwrap();

}