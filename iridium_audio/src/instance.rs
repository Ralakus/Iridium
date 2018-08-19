use super::rodio as rodio;
use super::core as core;
use std::rc::Rc;

pub struct AudioInstance {
    device: rodio::Device,
    sinks: Vec<Rc<rodio::Sink>>,
}

impl AudioInstance {
    pub fn new() -> Self {

        AudioInstance {
            device: rodio::default_output_device().unwrap(),
            sinks: Vec::new(),
        }

    }

    pub fn create_sink(&mut self) -> Result<(&Rc<rodio::Sink>, usize), core::IridiumError> {
        self.sinks.push(Rc::new(rodio::Sink::new(&self.device)));
        match self.sinks.len() {
            0 => { return Err(core::IridiumError::new(String::from("Error creating sink"))); } ,
            n => { return Ok((&self.sinks[n-1], n-1)); },
        }
    }

    pub fn remove_sink(&mut self, index: usize) {
        self.sinks.remove(index);
    }
}