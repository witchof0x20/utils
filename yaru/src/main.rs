// Copyright 2023 witchof0x20
//
// This file is part of yaru.
//
// yaru is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// yaru is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with yaru. If not, see <https://www.gnu.org/licenses/>.

mod bluetoothctl;
mod git;

use async_trait::async_trait;
use clap::{crate_name, Parser, Subcommand as ClapSubcommand};
use serde::Deserialize;
use std::collections::HashMap;
use std::{fs, io};

/// Main arguments
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Subcommand to run
    #[command(subcommand)]
    command: Subcommand,
}

/// Potential subcommands
#[derive(Debug, ClapSubcommand)]
enum Subcommand {
    /// Manipulate bluetooth devices
    #[clap(
        visible_alias = "bluetoothctl",
        visible_alias = "bluetooth",
        visible_alias = "btctl",
        visible_alias = "bt"
    )]
    BluetoothCtl(bluetoothctl::Args),
    /// Set git profile info for the current repository
    Git(git::Args),
}

/// Shortcut type
type Profiles<T> = Option<HashMap<String, T>>;

/// Configuration schema
#[derive(Deserialize)]
struct Config {
    /// Bluetoothctl profiles
    bluetoothctl: Profiles<bluetoothctl::Profile>,
    /// Git profiles
    git: Profiles<git::Profile>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), MainError> {
    // Parse arguments
    let args = Args::parse();
    // Do XDG correctly
    let xdg_dirs =
        xdg::BaseDirectories::with_prefix(crate_name!()).map_err(MainError::FindXdgDirs)?;
    // Read the config file
    let config_path = xdg_dirs.get_config_file("config.toml");
    let config_str = fs::read_to_string(config_path).map_err(MainError::ReadConfig)?;
    // Load the config file
    let config: Config = toml::from_str(&config_str)?;
    // Run the subcommand
    match args.command {
        Subcommand::BluetoothCtl(args) => args.run(config.bluetoothctl).await?,
        Subcommand::Git(args) => args.run(config.git).await?,
    }

    Ok(())
}

/// Interface that all subcommands must implement
#[async_trait]
pub trait YaruSubcommand {
    /// The error type
    type Err;
    /// The type for the config profile
    type Profile;

    /// Run the subcommand
    async fn run(self, profiles: Profiles<Self::Profile>) -> Result<(), Self::Err>;
}

/// Error wrapper for the whole program
#[derive(Debug, thiserror::Error)]
enum MainError {
    #[error("Error finding xdg dirs: {0}")]
    FindXdgDirs(xdg::BaseDirectoriesError),
    #[error("Error reading config file: {0}")]
    ReadConfig(io::Error),
    #[error("Error parsing config file: {0}")]
    ParseConfig(#[from] toml::de::Error),
    #[error("Error running the bluetoothctl subcommand: {0}")]
    Bluetoothctl(#[from] bluetoothctl::Error),
    #[error("Error running the git subcommand: {0}")]
    Git(#[from] git::Error),
}
