//! This file contains the application log

use std::error;
use crate::states::{AppState};
use crate::states::home::Home;

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

impl App{

    pub fn new() -> Self {
        Default::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }
}
