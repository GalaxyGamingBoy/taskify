//! This file contains the application log

use std::error;
use ratatui::widgets::Widget;
use crate::keybindings::Keybindings;
use crate::states::{AppState, AppStates};
use crate::states::home::Home;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub state: AppStates
}

impl App{

    pub fn new() -> Self {
        Self {
            state: AppStates::Home(Home {}),
            running: true
        }
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }
}
