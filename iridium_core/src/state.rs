use super::*;
use error::IridiumError;

use std::collections::HashMap;

pub trait IridiumState {

    fn awake (&mut self, shared_data: &mut Option<Box<std::any::Any>>)   -> Result<(), IridiumError>;
    fn update(&mut self, delta: f32)                                     -> Result<(), IridiumError>;
    fn end   (&mut self)                                                 -> Result<(), IridiumError>;

    fn handle_event(&mut self, event: event::IridiumEvent)               -> Result<(), IridiumError>;

    fn get_name(&self) -> String;

}


pub struct StateManager<'a> {
    state_map: HashMap<String, Box<IridiumState + 'a>>,
    current_state: String,
    pending_state: String,
    state_selected: bool,
    pending_state_change: bool,
    shared_data: Option<Box<std::any::Any>>,
}

impl<'a> StateManager<'a> {
    pub fn new() -> Self {
        StateManager {
            state_map: HashMap::new(),
            current_state: String::new(),
            pending_state: String::new(),
            state_selected: false,
            pending_state_change: false,
            shared_data: None
        }
    }

    pub fn register_state<T: IridiumState + 'a>(&mut self, state: T) -> Result<(), IridiumError> {
        if self.state_map.contains_key(&state.get_name()) {
            return Err(IridiumError::new(String::from("State of the same type already registered in state manager")));
        }
        else {
            self.state_map.insert(state.get_name(), Box::new(state));
        }

        Ok(())
    }

    pub fn set_next_state(&mut self, state_name: &String) -> Result<(), IridiumError> {
        if self.state_map.contains_key(state_name) {
            self.pending_state = state_name.to_string();
            self.pending_state_change = true;
        }
        else {
            return Err(IridiumError::new(format!("State \"{0}\" is not registered in state manager or does not exist!", state_name)));
        }
        Ok(())
    }

    pub fn awake_state(&mut self) -> Result<(),  IridiumError> {
        if self.state_selected {
            if let Some(state) = self.state_map.get_mut(&self.current_state) {
                match state.as_mut().awake(&mut self.shared_data) {
                    Ok(_)  => (),
                    Err(e) => return Err(e),
                }
            }
        }
        else {
            return Err(IridiumError::new(String::from("State manager does not have a state selected!")));
        }
        Ok(())
    }

    pub fn update_state(&mut self, delta: f32) -> Result<(), IridiumError> {
        if self.state_selected {
            if let Some(state) = self.state_map.get_mut(&self.current_state) {
                match state.as_mut().update(delta) {
                    Ok(_)  => (),
                    Err(e) => return Err(e),
                }
            }
        }
        else {
            return Err(IridiumError::new(String::from("State manager does not have a state selected!")));
        }
        Ok(())
    }

    pub fn end_state(&mut self) -> Result<(),  IridiumError> {
        if self.state_selected {
            if let Some(state) = self.state_map.get_mut(&self.current_state) {
                match state.as_mut().end() {
                    Ok(_)  => (),
                    Err(e) => return Err(e),
                }
            }
        }
        else {
            return Err(IridiumError::new(String::from("State manager does not have a state selected!")));
        }
        Ok(())
    }

    pub fn send_event(&mut self, event: event::IridiumEvent) -> Result<(), IridiumError> {
        if self.state_selected {
            if let Some(state) = self.state_map.get_mut(&self.current_state) {
                match state.as_mut().handle_event(event) {
                    Ok(_)  => (),
                    Err(e) => return Err(e),
                }
            }
        }
        else {
            return Err(IridiumError::new(String::from("State manager does not have a state selected!")));
        }
        Ok(())
    }

    pub fn switch_to_next_state(&mut self) -> Result<(), IridiumError> {
        if self.pending_state_change {
            if self.state_selected {
                if let Err(e) = self.end_state() {
                    return Err(e);
                }
            }
            self.current_state = self.pending_state.clone();
            self.pending_state_change = false;
            self.state_selected = true;
            if let Err(e) = self.awake_state() {
                return Err(e);
            }
        }
        else {
            return Err(IridiumError::new(String::from("There is no state pending!")));
        }

        Ok(())
    }

    pub fn set_shared_data(&mut self, data: Box<std::any::Any>) {
        self.shared_data = Some(data);
    }

    
}