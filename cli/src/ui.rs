use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(_app: &mut App, frame: &mut Frame) {
    frame.render_widget(
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
                " way to handle tasks, timemanage and be productive!".into(),
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
        .alignment(Alignment::Center),
        frame.size(),
    )
}
