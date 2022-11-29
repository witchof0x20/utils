// Copyright 2022 witchof0x20
//
// This file is part of recently_use.
//
// recently_use is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// recently_use is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with recently_use. If not, see <https://www.gnu.org/licenses/>.
use clap::Parser;
use gtk::prelude::RecentManagerExt;
use gtk::{RecentData, RecentManager};
use std::io;
use std::path::PathBuf;
use thiserror::Error;

/// Simple program to add a file to "recently used" so it shows up quickly in the file manager
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the file to add to recently used  
    path: PathBuf,
}

#[derive(Debug, Error)]
enum MainError {
    #[error("Given path does not exist")]
    DoesNotExist,
    #[error("Error canonicalizing the given path: {0}")]
    Canonicalize(io::Error),
    #[error("Failed to convert the given path into a URI: {0}")]
    Uri(glib::Error),
    #[error("Failed to initialize gtk: {0}")]
    GtkInit(glib::error::BoolError),
    #[error("Failed to add item to RecentManager")]
    AddItem,
}

fn main() -> Result<(), MainError> {
    // Parse arguments
    let args = Args::parse();
    // Check for existence of given path
    if !args.path.exists() {
        return Err(MainError::DoesNotExist);
    }
    // Canonicalize the path
    let path = args.path.canonicalize().map_err(MainError::Canonicalize)?;
    let uri = glib::filename_to_uri(&path, None).map_err(MainError::Uri)?;
    // Initialize gtk
    gtk::init().map_err(MainError::GtkInit)?;
    // Get a handle on the recent manager
    let recent_manager = RecentManager::new();
    // Convert the path string to a file URI
    // Add the file to the recent manager
    let recent_data = RecentData {
        display_name: path
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .map(String::from),
        description: None,
        mime_type: "application/octet-stream".into(),
        app_name: clap::crate_name!().into(),
        app_exec: "xdg-open %f".into(),
        groups: Vec::new(),
        is_private: false,
    };
    let res = if recent_manager.add_full(&uri, &recent_data) {
        Ok(())
    } else {
        Err(MainError::AddItem)
    };
    for i in recent_manager.items() {
        dbg!(&i);
    }
    res
}
