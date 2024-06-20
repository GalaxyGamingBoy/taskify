//! This file contains all the UI components

use ratatui::{
    Frame,
};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

use crate::app::App;

//noinspection DuplicatedCode
pub fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn render(app: &mut App, frame: &mut Frame) {
    app.state.render(Rect {height: frame.size().height - 1, ..frame.size()}, frame.buffer_mut());

    let mut bottom_bar = vec![
        Span::styled(" TASKIFY ", Style::new().black().on_white()),
        Span::styled(format!(" {} ", app.state.display_name()), Style::new().white().on_dark_gray()),
        " q: Quit ".into()
    ];

    app.state.keybindings().iter().for_each(|key| {
        bottom_bar.push(format!(" {}: {} ", key.key, key.name).into())
    });

    frame.render_widget(Paragraph::new(Line::from(bottom_bar)), Rect {height: 1, y: frame.size().height - 1, ..frame.size()});
}
