//! Subcommands of the `tmkms` command-line application

pub mod init;
pub mod start;
pub use self::{init::InitCommand, start::StartCommand};

use crate::config::{KmsConfig, CONFIG_ENV_VAR, CONFIG_FILE_NAME};
use abscissa_core::{Command, Configurable, Runnable};
use clap::Parser;
use std::{env, path::PathBuf};

/// Subcommands of the KMS command-line application
#[derive(Command, Debug, Parser, Runnable)]
pub enum KmsCommand {
    /// initialize KMS configuration
    Init(InitCommand),

    /// start the KMS application"
    Start(StartCommand),

}

impl KmsCommand {
    /// Are we configured for verbose logging?
    pub fn verbose(&self) -> bool {
        match self {
            KmsCommand::Start(run) => run.verbose,
            _ => false,
        }
    }
}

impl Configurable<KmsConfig> for KmsCommand {
    /// Get the path to the configuration file, either from selected subcommand
    /// or the default
    fn config_path(&self) -> Option<PathBuf> {
        let config = match self {
            KmsCommand::Start(start) => start.config.as_ref(),
            _ => return None,
        };

        let path = config
            .cloned()
            .or_else(|| env::var(CONFIG_ENV_VAR).ok().map(PathBuf::from))
            .unwrap_or_else(|| PathBuf::from(CONFIG_FILE_NAME));

        Some(path)
    }
}
