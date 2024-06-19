//! This file contains all the UI components

use ratatui::{
    Frame,
};
use ratatui::widgets::Widget;

use crate::app::App;
use crate::states::{AppStates, State};

pub fn render(app: &mut App<impl State + Widget + ToString>, frame: &mut Frame){
    match app.state.to_string() {
        "home" => frame.render_widget(app.state, frame.size()),
        _ => {}
    }
}
