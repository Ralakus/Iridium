use super::*;

use std::collections::HashMap;

pub trait IridiumState {

    fn awake (&mut self)             -> Result<(), error::IridiumError>;
    fn update(&mut self, delta: f32) -> Result<(), error::IridiumError>;
    fn end   (&mut self)             -> Result<(), error::IridiumError>;

    fn handle_event(&mut self, event: event::IridiumEvent) -> Result<(), error::IridiumError>;

    fn get_name(&self) -> String;

}


pub struct StateManager<'a> {
    state_map: HashMap<String, Box<IridiumState + 'a>>,
    current_state: String,
    state_selected: bool,
}

impl<'a> StateManager<'a> {
    pub fn new() -> Self {
        StateManager {
            state_map: HashMap::new(),
            current_state: String::new(),
            state_selected: false,
        }
    }

    pub fn register_state<T: IridiumState + 'a>(&mut self, state: T) -> Result<(), error::IridiumError> {
        if self.state_map.contains_key(&state.get_name()) {
            return Err(error::IridiumError::new(String::from("State of the same type already registered in state manager")));
        }
        else {
            self.state_map.insert(state.get_name(), Box::new(state));
        }

        Ok(())
    }

    pub fn set_state(&mut self, state_name: &String) -> Result<(), error::IridiumError> {
        if self.state_map.contains_key(state_name) {
            self.current_state = state_name.to_string();
            self.state_selected = true;
        }
        else {
            return Err(error::IridiumError::new(format!("State \"{0}\" is not registered in state manager or does not exist!", state_name)));
        }
        Ok(())
    }

    pub fn awake_state(&mut self) -> Result<(), error::IridiumError> {
        if self.state_selected {
            if let Some(state) = self.state_map.get_mut(&self.current_state) {
                match state.as_mut().awake() {
                    Ok(_)  => (),
                    Err(e) => return Err(e),
                }
            }
        }
        else {
            return Err(error::IridiumError::new(String::from("State manager does not have a state selected!")));
        }
        Ok(())
    }

    pub fn update_state(&mut self, delta: f32) -> Result<(), error::IridiumError> {
        if self.state_selected {
            if let Some(state) = self.state_map.get_mut(&self.current_state) {
                match state.as_mut().update(delta) {
                    Ok(_)  => (),
                    Err(e) => return Err(e),
                }
            }
        }
        else {
            return Err(error::IridiumError::new(String::from("State manager does not have a state selected!")));
        }
        Ok(())
    }

    pub fn end_state(&mut self) -> Result<(), error::IridiumError> {
        if self.state_selected {
            if let Some(state) = self.state_map.get_mut(&self.current_state) {
                match state.as_mut().end() {
                    Ok(_)  => (),
                    Err(e) => return Err(e),
                }
            }
        }
        else {
            return Err(error::IridiumError::new(String::from("State manager does not have a state selected!")));
        }
        Ok(())
    }

    pub fn send_event(&mut self, event: event::IridiumEvent) -> Result<(), error::IridiumError> {
        if self.state_selected {
            if let Some(state) = self.state_map.get_mut(&self.current_state) {
                match state.as_mut().handle_event(event) {
                    Ok(_)  => (),
                    Err(e) => return Err(e),
                }
            }
        }
        else {
            return Err(error::IridiumError::new(String::from("State manager does not have a state selected!")));
        }
        Ok(())
    }

    
}