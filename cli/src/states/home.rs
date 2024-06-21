//! This file contains the logic for the home state

use std::rc::Rc;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::{Color, Modifier, Span, Style};
use ratatui::widgets::{Block, Borders, BorderType, Paragraph, Widget};
use tokio::sync::mpsc;
use crate::actions::{Action, Event};
use crate::app::App;
use crate::keybindings::{Keybinding, Keybindings};
use crate::states::{AppState, RenderState};
use crate::states::project::Project;

#[derive(Debug)]
pub struct Home {}

impl AppState for Home {
    fn display_name(&self) -> &str {
        "HOME"
    }

    fn action(&mut self, action: Action) {
        match action {
            Action::Escape => {},
            Action::Home_GotoProjects => {},
            _ => {}
        }
    }

    fn tick(&mut self)  {}
}
impl Keybindings for Home {
    fn keybindings(&self) -> Vec<Keybinding> {
        vec![Keybinding { key: 'p', name: "View Projects".into(), action: Action::Home_GotoProjects}]
    }
}

impl RenderState for Home {
    fn render(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(vec![
            "".into(),
            vec![
                Span::from("Welcome to "),
                Span::styled(
                    "Taskify!",
                    Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                ),
            ]
                .into(),
            vec![
                "The ".into(),
                Span::styled(
                    "modern CLI",
                    Style::default()
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED | Modifier::ITALIC),
                ),
                " way to handle tasks, time manage and be productive!".into(),
            ]
                .into(),
            "".into(),
            Span::styled(
                "Press q to exit at any time",
                Style::default().add_modifier(Modifier::ITALIC),
            )
                .into(),
        ])
            .block(
                Block::default()
                    .title(" [Taskify CLI] ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain),
            )
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center).render(area, buf)
    }
}