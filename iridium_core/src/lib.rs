
pub extern crate specs;
pub extern crate nalgebra as na;

extern crate specs_hierarchy;

pub mod error;
pub mod components;
pub mod systems;
pub mod event;
pub mod state;

pub use error::IridiumError;
pub use event::IridiumEvent;
pub use state::IridiumState;
pub use state::StateManager;

