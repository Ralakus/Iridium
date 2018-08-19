extern crate iridium;
extern crate serde_json;

use iridium::audio::rodio as rodio;
use std::fs::File;
use std::io::BufReader;

/*struct TestShared {
    val: i32
}

struct TestState {
    val1: i32,
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
*/
fn main() {

    let mut time = iridium::core::time::Time::new();

    /*let mut state_manager = iridium::core::StateManager::new();
    let test_state  = TestState  {val1: 0};
    let test_shared = TestShared {val: -0};

    state_manager.set_shared_data(Box::new(test_shared));

    state_manager.register_state(test_state).unwrap();
    state_manager.set_next_state(&String::from("TestState")).unwrap();
    state_manager.switch_to_next_state().unwrap();

    state_manager.update_state(0.0032f32).unwrap();
    state_manager.send_event(iridium::core::IridiumEvent::Close).unwrap();
    state_manager.end_state().unwrap();*/

    let config_json = File::open("config.json").unwrap();
    let json: serde_json::Value = serde_json::from_reader(config_json).unwrap();
    let audio_file : String = format!("{}", json["audio file"].as_str().unwrap());

    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);
    match File::open(audio_file.as_str()) {
        Ok(file) => {
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
            sink.append(source);
        },
        Err(e) => println!("Error opening {1}!\n{0}", e, audio_file.as_str())
    }

    let mut window = iridium::graphics::Window::new(Some(1280u32), Some(720u32), Some(String::from("Iridium_test")));

    let _sleep_dur = std::time::Duration::new(0, 1000);

    while window.is_valid() {
        if let Err(e) = window.update() {
            println!("Iridium error: {0}", e);
        }
        if sink.empty() {
            if let Err(e) = window.close() { 
                panic!("Error closing window! {0}", e);
            }
        }
        //std::thread::sleep(sleep_dur);
        time.update();
        if time.on_second() {
            //window.set_title(format!("FPS: {0}",time.fps()));
            println!("Delta: {0:?}, f64: {1}", time.delta(), iridium::core::duration_to_f64(time.delta()));
        }
    }

}