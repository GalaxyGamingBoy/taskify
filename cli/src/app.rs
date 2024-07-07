//! This file contains the application log

use futures::{Future, FutureExt};
use sqlx::SqliteConnection;
use std::error;
use std::rc::Rc;

use crate::states::home::Home;
use crate::states::project::Project;
use crate::states::{AppState, AppStates};

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub state: Box<dyn AppState + Send>,
    pub db: Option<SqliteConnection>,
    pub runtime: Rc<tokio::runtime::Runtime>
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            state: Box::new(Home::default()),
            db: None,
            runtime: Rc::new(tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap())
        }
    }
}

impl App {
    pub async fn new() -> Self {
        let mut app: App = Default::default();
        let (_, conn) = taskify::init().await.unwrap();

        app.db = Some(conn);
        app
    }

    pub async fn tick(&mut self) {
        self.state.tick(self.db.as_mut().unwrap()).await
    }

    pub async fn set_state(&mut self, event: AppStates) {
        match event {
            AppStates::Home => self.state = Box::new(Home::default()),
            AppStates::Project => self.state = Box::new(Project::default()),
        };

        self.state.init(self.db.as_mut().unwrap()).await
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
