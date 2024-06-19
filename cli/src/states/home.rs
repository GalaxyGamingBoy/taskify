use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::{Color, Modifier, Span, Style};
use ratatui::widgets::{Block, Borders, BorderType, Paragraph, Widget};
use crate::keybindings::{Keybinding, Keybindings};
use crate::states::AppState;

#[derive(Debug, Default, Clone)]
pub struct Home {}

impl Keybindings for Home {
    fn keybindings() -> Vec<Box<Keybinding>> {
        vec![Keybinding { key: '?', name: "Help".into() }.into()]
    }
}

impl AppState for Home {}

impl Widget for Home {
    fn render(self, area: Rect, buf: &mut Buffer) {
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
