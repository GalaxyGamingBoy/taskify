use std::io::stdout;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    style::Stylize,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

fn ui() -> Result<(), std::io::Error> {
    stdout()
        .execute(EnterAlternateScreen)
        .expect("Error while executing: EnterAlternateScreen");
    enable_raw_mode().expect("Error while enabling raw mode");

    let mut term =
        Terminal::new(CrosstermBackend::new(stdout())).expect("Error while creating a terminal");
    term.clear()?;

    loop {
        term.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hello World!")
                    .block(Block::default().title("Info").borders(Borders::ALL))
                    .white(),
                area,
            )
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }
        }
    }

    stdout()
        .execute(LeaveAlternateScreen)
        .expect("Error while executing: LeaveAlternateScreen");
    disable_raw_mode().expect("Error while disabling raw mode");

    Ok(())
}

fn main() {
    if taskify::init() {
        ui().expect("There was an error running the CLI applcation.");
    }
}
