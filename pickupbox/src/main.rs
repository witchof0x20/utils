use clap::Parser;
use dropbox_sdk::default_client::UserAuthDefaultClient;
use dropbox_sdk::files;
use std::path::PathBuf;

mod client;

/// Simple program to upload stuff to dropbox. Designed for use in Overleaf.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, trailing_var_arg=true)]
struct Args {
    /// Long-lived access token for dropbox
    #[clap(long, env = "DROPBOX_TOKEN")]
    dropbox_token: String,
    /// Project to upload the file to
    project: String,
    /// Files to upload
    #[clap(required = true)]
    files: Vec<PathBuf>,
}

fn main() {
    // Parse cli args
    let args = Args::parse();
    // Auth using the token
    let auth = dropbox_sdk::oauth2::Authorization::from_access_token(args.dropbox_token);
    // Create a dropbox client
    let client = UserAuthDefaultClient::new(auth);
    // Upload each file
    for file in files {}
}
