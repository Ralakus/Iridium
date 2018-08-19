
#[derive(Debug)]
pub enum IridiumEvent {
    Close,
    KeyPress,
    KeyRelease,
    KeyHeld,
    MouseUp,
    MouseDown,
    Data(Box<::std::any::Any>)
}