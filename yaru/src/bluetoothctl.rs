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
use bluer::{Address, AddressType};
use clap::{Parser, Subcommand as ClapSubcommand};
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Parser)]
pub struct Args {
    /// The device profile to operate on
    profile: String,
    /// The command to run
    #[command(subcommand)]
    command: Subcommand,
}
#[derive(Debug, ClapSubcommand)]
enum Subcommand {
    /// Connect to a device
    #[clap(visible_alias = "c")]
    Connect,
    /// Disconnect any connected devices
    #[clap(visible_alias = "d")]
    Disconnect,
}

/// Profile of a bluetooth devices
#[serde_as]
#[derive(Deserialize)]
pub struct Profile {
    /// The device's mac address
    #[serde_as(as = "DisplayFromStr")]
    mac: Address,
    /// The device's address type
    #[serde_as(as = "DisplayFromStr")]
    address_type: AddressType,
}

#[async_trait]
impl YaruSubcommand for Args {
    type Err = Error;
    type Profile = Profile;
    async fn run(self, profiles: Profiles<Self::Profile>) -> Result<(), Self::Err> {
        // Split off name
        let profile_name = self.profile;
        if let Some(profiles) = profiles {
            // Load the profile info for the given profile name
            let profile = profiles
                .get(&profile_name)
                .ok_or_else(|| Error::ProfileMissing(profile_name.to_string()))?;
            // Start by getting a handle on the bluetooth session
            let session = bluer::Session::new().await.map_err(Error::BTOpenSession)?;
            // Get the default adapter
            let adapter = session
                .default_adapter()
                .await
                .map_err(Error::BTOpenAdapter)?;
            // Run the subcommand
            match self.command {
                Subcommand::Connect => {
                    // Connect the device and do nothing with it
                    adapter
                        .connect_device(profile.mac, profile.address_type)
                        .await
                        .map(|_| ())
                        .map_err(Error::BTConnect)?;
                    Ok(())
                }
                Subcommand::Disconnect => {
                    // Get a handle on the specific device
                    let device = adapter.device(profile.mac).map_err(Error::BTGetDevice)?;
                    // Disconnect it
                    device.disconnect().await.map_err(Error::BTDisconnect)?;
                    Ok(())
                }
            }
        } else {
            Err(Error::ProfileMissing(profile_name.to_string()))
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Profile {0} missing")]
    ProfileMissing(String),
    #[error("Failed to get bluetooth session: {0}")]
    BTOpenSession(bluer::Error),
    #[error("Failed to get bluetooth adapter: {0}")]
    BTOpenAdapter(bluer::Error),
    #[error("Failed to connect to device: {0}")]
    BTConnect(bluer::Error),
    #[error("Failed to get device handle: {0}")]
    BTGetDevice(bluer::Error),
    #[error("Failed to disconnect from device: {0}")]
    BTDisconnect(bluer::Error),
}
