//! This file contains the application log

use std::error;
use ratatui::widgets::Widget;
use crate::states::{AppStates, State};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct App<T> where T: State + Widget {
    pub running: bool,
    pub state: AppStates<T>
}

impl<T> Default for App<T> where T: State + Widget {
    fn default() -> Self {
        Self { running: true, state: Default::default() }
    }
}

impl<T> App<T> where T: State + Widget {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }
}
