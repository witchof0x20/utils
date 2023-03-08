// Copyright 2023 witchof0x20
//
// This file is part of yaru.
//
// yaru is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// yaru is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with yaru. If not, see <https://www.gnu.org/licenses/>.

use crate::{Profiles, YaruSubcommand};
use async_trait::async_trait;
use clap::{Parser, Subcommand as ClapSubcommand};
use git2::{ConfigLevel, Repository};
use serde::Deserialize;
use std::env;
use std::io;

#[derive(Debug, Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Subcommand,
}
#[derive(Debug, ClapSubcommand)]
enum Subcommand {
    /// Configure the user config for the current repository for the given git profile
    #[clap(visible_alias = "u")]
    User {
        /// Profile to use for the user
        profile: String,
    },
}

#[derive(Deserialize)]
pub struct Profile {
    user_name: Option<String>,
    user_email: Option<String>,
    user_signing_key: Option<String>,
}

#[async_trait]
impl YaruSubcommand for Args {
    type Err = Error;
    type Profile = Profile;

    async fn run(self, profiles: Profiles<Self::Profile>) -> Result<(), Self::Err> {
        // Get the current directory
        let cwd = env::current_dir().map_err(Error::CurrentDir)?;
        // All operations require accessing the current directory as a git repository
        let repo = Repository::open(cwd).map_err(Error::GitOpen)?;
        // Run the subcommand
        match self.command {
            Subcommand::User { profile } => {
                if let Some(profiles) = profiles {
                    // Load the profile info for the given user
                    let profile = profiles
                        .get(&profile)
                        .ok_or_else(|| Error::ProfileMissing(profile.to_string()))?;
                    // Get a handle on the local config
                    let mut config = repo
                        .config()
                        .map_err(Error::GitOpenConfig)?
                        .open_level(ConfigLevel::Local)
                        .map_err(Error::GitOpenConfig)?;
                    // Set each parameter
                    if let Some(ref user_name) = profile.user_name {
                        config
                            .set_str("user.name", user_name)
                            .map_err(Error::GitSetConfig)?;
                    }
                    if let Some(ref user_email) = profile.user_email {
                        config
                            .set_str("user.email", user_email)
                            .map_err(Error::GitSetConfig)?;
                    }
                    if let Some(ref user_signing_key) = profile.user_signing_key {
                        config
                            .set_str("user.signingKey", user_signing_key)
                            .map_err(Error::GitSetConfig)?;
                    }
                } else {
                    return Err(Error::ProfileMissing(profile.to_string()));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to get the current directory: {0}")]
    CurrentDir(io::Error),
    #[error("Failed to open the current directory as a Git repository: {0}")]
    GitOpen(git2::Error),
    #[error("Profile {0} missing")]
    ProfileMissing(String),
    #[error("Failed to get a handle on the local git config: {0}")]
    GitOpenConfig(git2::Error),
    #[error("Failed to set a variable in the local git config: {0}")]
    GitSetConfig(git2::Error),
}
