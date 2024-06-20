//! This file contains all the handlers, i.e., key presses

use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent};
use crate::actions::Action;

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Char('q') => {
            app.quit();
        },
        KeyCode::Esc => app.state.action(Action::Escape),
        _ => {}
    }

    app.state.keybindings().iter_mut().for_each(|key| {
        if key_event.code == KeyCode::Char(key.key) {
            app.state.action(key.action)
        }
    });

    Ok(())
}
