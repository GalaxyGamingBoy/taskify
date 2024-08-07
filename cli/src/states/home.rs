//! This file contains the logic for the home state

use crate::actions::{Action, Event};
use crate::keybindings::{Keybinding, Keybindings};
use crate::states::{AppState, AppStates, RenderState};
use async_trait::async_trait;
use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::{Color, Modifier, Span, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Widget};
use sqlx::SqliteConnection;

#[derive(Debug, Default)]
pub struct Home {}

#[async_trait]
impl AppState for Home {
    fn display_name(&self) -> &str {
        "HOME"
    }

    async fn init(&mut self, exec: &mut SqliteConnection) {}

    async fn tick(&mut self, exec: &mut SqliteConnection) {}

    fn action(&mut self, action: Action) -> Event {
        match action {
            Action::Escape => Event::None,
            Action::HomeGotoProjects => Event::Goto(AppStates::Project),
            _ => Event::None,
        }
    }
}
impl Keybindings for Home {
    fn keybindings(&self) -> Vec<Keybinding> {
        vec![Keybinding {
            key: 'p',
            name: "View Projects".into(),
            action: Action::HomeGotoProjects,
        }]
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
        .alignment(Alignment::Center)
        .render(area, buf)
    }
}
