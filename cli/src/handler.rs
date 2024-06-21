//! This file contains all the handlers, i.e., key presses

use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent};
use crate::actions::{Action, Event};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Char('q') => {
            app.quit();
        },
        KeyCode::Esc => { let _ = app.state.action(Action::Escape); },
        _ => {}
    }

    app.state.keybindings().iter_mut().for_each(|key| {
        if key_event.code == KeyCode::Char(key.key) {
            match app.state.action(key.action) {
                Event::Goto(v) => app.set_state(v),
                _ => {}
            }
        }
    });

    Ok(())
}
