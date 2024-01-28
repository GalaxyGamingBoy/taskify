//! Config Module
//! This module handles all tasks related to the config, such us loading & providing the config schema or setting up the logger.
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let config = taskify::config::Config::load()?;
//! # Ok(())
//! # }
//! ```

use std::fs::{self, File};

use migration::MigratorTrait;
use sea_orm::{DatabaseConnection, DbErr};
use serde::Deserialize;
use simplelog::{CombinedLogger, SharedLogger, TermLogger, WriteLogger};

/// The database configuration schema & structure.
#[derive(Deserialize, Debug)]
pub struct Database {
    /// The path of the SQLITE database.
    pub path: String,
}

/// The logger configuration schema & structure.
#[derive(Deserialize, Debug)]
pub struct Logger {
    /// Is logging enabled?
    pub enabled: bool,

    /// Should logs be saved & written into a file?
    pub write_logs: bool,

    /// Should logs be printed to stdout?
    pub print_logs: bool,

    /// The log path that the logs will be saved in.
    pub log_path: String,
}

/// The configuration structure & schema that it used for the config.toml
#[derive(Deserialize, Debug)]
pub struct Config {
    pub database: Database,
    pub logger: Logger,
}

impl Config {
    /// Loads the config.toml and parses it into a [`crate::config::Config`]
    ///
    /// # Examples:
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = taskify::config::Config::load()?;
    /// println!("{:?}", config);
    /// # Ok(())
    /// # }
    /// ```
    pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
        let content = fs::read_to_string("./config.toml");

        match content {
            Ok(content) => Ok(toml::from_str(&content).unwrap()),
            Err(err) => Err(err.into()),
        }
    }
}

/// Initialized the logger using the [`crate::config::Logger`]
///
/// # Examples:
/// ```
/// # use taskify::config::{init_log, Logger};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let logger_config = Logger { enabled: true, write_logs: true, log_path: "./taskify.log".into() };
/// init_log(&logger_config);
/// # Ok(())
/// # }
/// ```
pub fn init_log(config: &Logger) {
    let mut loggers: Vec<Box<dyn SharedLogger>> = vec![];

    if config.write_logs {
        loggers.push(WriteLogger::new(
            log::LevelFilter::Info,
            simplelog::Config::default(),
            File::create(config.log_path.clone()).unwrap(),
        ))
    }

    if config.print_logs {
        loggers.push(TermLogger::new(
            log::LevelFilter::Debug,
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ))
    }

    if config.enabled {
        CombinedLogger::init(loggers).expect("Error while initalizing logger!");

        log::info!("Logging enabled and initialized!")
    }
}

/// Initializes the sqlite database & applies migrations
///
/// # Examples:
/// ```
/// # use taskify::config::{Database, init_db};
/// # #[tokio::test]
/// # async fn test() -> Result<(), sea_orm::DbErr> {
/// let config = Database { path: "taskify.db".into() };
/// init_db(&config).await?;
/// # Ok(())
/// # }
/// ```
pub async fn init_db(config: &Database) -> Result<DatabaseConnection, DbErr> {
    let db = sea_orm::Database::connect(format!("sqlite://{}?mode=rwc", config.path)).await?;

    migration::Migrator::up(&db, None).await?;

    Ok(db)
}
