//! This file contains all the UI components

use ratatui::{
    Frame,
};

use crate::app::App;

pub fn render (app: &mut App, frame: &mut Frame) {
    app.state.render(frame.size(), frame.buffer_mut())
}
