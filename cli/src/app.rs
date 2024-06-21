//! This file contains the application log

use std::error;
use crate::states::{AppState, AppStates};
use crate::states::home::Home;
use crate::states::project::Project;

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub state: Box<dyn AppState>
}

impl Default for App {
    fn default() -> Self {
        Self {running: true, state: Box::new(Home::default())}
    }
}

impl App {

    pub fn new() -> Self {
        Default::default()
    }

    pub fn tick(&mut self) {
        self.state.tick()
    }

    pub fn set_state(&mut self, event: AppStates) {
        match event {
            AppStates::Home => self.state = Box::new(Home::default()),
            AppStates::Project => self.state = Box::new(Project::default())
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
