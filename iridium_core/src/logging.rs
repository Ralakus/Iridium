pub enum LogType {
    Log,
    Note,
    Warning,
    Error,
    Success
}

pub struct LogBuffer {
    buffer: Vec<(LogType, String)>,
}

impl LogBuffer {
    pub fn new() -> Self {
        LogBuffer {
            buffer: Vec::new()
        }
    }

    pub fn add(&mut self, log_type: LogType, message: String) {
        self.buffer.push((log_type, message.clone()));
    }

    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }
}