use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct IridiumError {
    what: String,
}

#[allow(dead_code)]
impl IridiumError {
    pub fn new(what: String) -> IridiumError {
        IridiumError {
            what
        }
    }
}

impl Error for IridiumError {
    fn description(&self) -> &str {
        &self.what
    }
}

impl fmt::Display for IridiumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0}", self.what)
    }
}