//! Config Module
//! This module handles all tasks related to the config, such us loading & providing the config schema.
//!
//! ```
//! # fn main() -> Result<(), std::io::Error> {
//! let config = taskify::config::Config::load()?;
//! # Ok(())
//! # }
//! ```

use std::fs;

use serde::Deserialize;

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
    /// # fn main() -> Result<(), std::io::Error> {
    /// let config = taskify::config::Config::load()?;
    /// println!("{:?}", config);
    /// # Ok(())
    /// # }
    /// ```
    pub fn load() -> Result<Config, std::io::Error> {
        let content = fs::read_to_string("./config.toml");

        match content {
            Ok(content) => Ok(toml::from_str(&content).unwrap()),
            Err(err) => Err(err),
        }
    }
}
