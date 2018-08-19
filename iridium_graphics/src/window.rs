#[allow(unused_imports)]
use super::winit as winit;

pub struct Window {
    winit_events_loop: Option<winit::EventsLoop>,
    winit_window: Option<winit::Window>,
    name: String,
    width: u32,
    height: u32,
    valid: bool,
}

impl Window {
    pub fn new(width: Option<u32>, height: Option<u32>, name: Option<String>) -> Self {
        let events_loop = winit::EventsLoop::new();
        let temp_name = match name {
            Some(name_str) => name_str,
            None => String::from("Iridium"),
        };
        let temp_width = match width {
            Some(width_val) => width_val,
            None => 800,
        };
        let temp_height = match height {
            Some(height_val) => height_val,
            None => 600,
        };
        let window = winit::WindowBuilder::new()
            .with_title(temp_name.as_str())
            .with_dimensions((temp_width, temp_height).into())
            .build(&events_loop)
            .unwrap();
        Window{
            winit_events_loop: Some(events_loop),
            winit_window: Some(window),
            name: temp_name,
            width: temp_width,
            height: temp_height,
            valid: true,
        }
    }

    pub fn new_closed() -> Self {
        Window{
            winit_events_loop: None,
            winit_window: None,
            name: String::from("Iridium"),
            width: 0,
            height: 0,
            valid: false,
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

    pub fn open(&mut self, width: Option<u32>, height: Option<u32>, name: Option<String>) -> Result<(), super::core::IridiumError> {
        if !self.valid {
            self.winit_events_loop = Some(winit::EventsLoop::new());
            let temp_name = match name {
                Some(name_str) => name_str,
                None => String::from("Iridium"),
            };
            let temp_height = match height {
                Some(height_val) => height_val,
                None => 600,
            };
            let temp_width = match width {
                Some(width_val) => width_val,
                None => 800,
            };
            self.winit_window = Some(winit::WindowBuilder::new()
                .with_title(temp_name.as_str())
                .with_dimensions((temp_width, temp_height).into())
                .build(match &self.winit_events_loop {
                    Some(win) => &win,
                    None => return Err(super::core::IridiumError::new(String::from("Failed to create events loop!"))),
                })
                .unwrap());
            self.valid = false;
            Ok(())
        }
        else {
            Err(super::core::IridiumError::new(String::from("Window is already opened!")))
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}