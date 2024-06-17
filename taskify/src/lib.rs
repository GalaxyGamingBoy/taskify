//! Taskify: Empowering Productivity with a Simple, Safe, and Integrated Task Management Solution
//!
//! This `taskify` crate contains the whole backend code for all taskify services. Taskify is an open-source and memory-safe Task Management System (TMS) designed to help you stay organized, productive, and connected with your GitHub or not workflow. Built with the power of Rust and leveraging the capabilities of SeaORM, Ratatui, and Tauri, Taskify offers a seamless experience for managing your tasks, projects, notes, and GitHub issues, all from a single, intuitive interface.

use sqlx::SqliteConnection;
use config::Config;

pub mod config;
pub mod db;

/// Initializes Taskify by loading the config & any other necessary thing that is needed for startup
///
/// # Examples
/// ```
/// # #[tokio::test]
/// # async fn test() -> Result<(), Box<dyn std::error::Error>> {
/// let config = taskify::init().await?;
/// println!("{}", config.0.database.path); // Should print: "./taskify.db"
/// # Ok(())
/// # }
/// ```
pub async fn init() -> Result<(Config, SqliteConnection), Box<dyn std::error::Error>> {
    let config = Config::load().expect("Error while loading config.toml");
    config::init_log(&config.logger);
    let db = config::init_db(&config.database).await?;

    Ok((config, db))
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn init() {
        assert!(crate::init().await.is_ok())
    }
}
