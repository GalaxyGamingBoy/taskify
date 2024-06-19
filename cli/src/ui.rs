//! This file contains all the UI components

use ratatui::{
    Frame,
};
use ratatui::widgets::Widget;

use crate::app::App;
use crate::keybindings::Keybindings;
use crate::states::{AppState};

pub fn render (app: &mut App, frame: &mut Frame) {
    frame.render_widget(app.state.clone(), frame.size());
}
