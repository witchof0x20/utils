// This file is part of recently_use.
//
// recently_use is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// recently_use is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with recently_use. If not, see <https://www.gnu.org/licenses/>.
use clap::Parser;
use gtk::prelude::RecentManagerExt;
use gtk::RecentManager;
use std::ffi::OsString;
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
    #[error("Failed to convert the given path ({0:?}) into a manageable string")]
    PathString(OsString),
    #[error("Failed to initialize gtk: {0}")]
    GtkInit(glib::error::BoolError),
    #[error("Failed to get handle on RecentManager")]
    RecentManager,
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
    // Convert path to a string
    let path = path
        .into_os_string()
        .into_string()
        .map_err(MainError::PathString)?;
    // Initialize gtk
    gtk::init().map_err(MainError::GtkInit)?;
    // Get a handle on the recent manager
    let recent_manager = RecentManager::default().ok_or(MainError::RecentManager)?;
    // Convert the path string to a file URI
    let file_uri = format!("file://{}", path);
    // Add the file to the recent manager
    if recent_manager.add_item(&file_uri) {
        Ok(())
    } else {
        Err(MainError::AddItem)
    }
}
