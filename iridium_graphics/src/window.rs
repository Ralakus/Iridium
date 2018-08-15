#[allow(unused_imports)]
use super::winit as winit;

pub struct Window {
    winit_events_loop: Option<winit::EventsLoop>,
    winit_window: Option<winit::Window>,
    valid: bool,
}

impl Window {
    pub fn new() -> Self {
        let events_loop = winit::EventsLoop::new();
        let window = winit::Window::new(&events_loop).unwrap();
        Window{
            winit_events_loop: Some(events_loop),
            winit_window: Some(window),
            valid: true,
        }
    }

    pub fn set_title(&mut self, name: String) -> () {
        if let Some(ref window) = self.winit_window {
            window.set_title(name.as_str());
        }
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn update(&mut self) -> Result<(), super::core::IridiumError> {
        if self.valid {
            let mut temp_valid = self.valid;

            if let Some(ref mut events_loop) = self.winit_events_loop {
                events_loop.poll_events(|event| {
                    match event {

                        winit::Event::WindowEvent {
                            event: winit::WindowEvent::CloseRequested,
                            ..
                        } => temp_valid = false,

                        _ => (),

                    }
                });
            }

            self.valid = temp_valid;

            return Ok(());
        }
        else {
            return Err(super::core::IridiumError::new(String::from("Window was invalid when \"update()\" was called!")));
        }
    }

    pub fn close(&mut self) -> Result<(), super::core::IridiumError> {
        self.winit_events_loop = None;
        self.winit_window = None;
        self.valid = false;
        Ok(())
    }
}