//! Taskify: Empowering Productivity with a Simple, Safe, and Integrated Task Management Solution
//!
//! This `taskify` crate contains the whole backend code for all taskify services. Taskify is an open-source and memory-safe Task Management System (TMS) designed to help you stay organized, productive, and connected with your GitHub or not workflow. Built with the power of Rust and leveraging the capabilities of SeaORM, Ratatui, and Tauri, Taskify offers a seamless experience for managing your tasks, projects, notes, and GitHub issues, all from a single, intuitive interface.

use config::Config;

pub mod config;

/// Initializes Taskify by loading the config & any other necessary thing that is neeeded for startup
///
/// # Examples
/// ```
/// # fn main() -> Result<(), std::io::Error> {
/// let config = taskify::init()?;
/// println!("{}", config.database.path); // Should print: "./taskify.db"
/// # Ok(())
/// # }
/// ```
pub fn init() -> Result<Config, std::io::Error> {
    let config = config::Config::load()?;
    config::init_log(&config.logger);

    Ok(config)
}

#[cfg(test)]
mod tests {
    #[test]
    fn init() {
        assert!(crate::init().is_ok())
    }
}
