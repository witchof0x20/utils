// Copyright 2023 Jade
// This file is part of don.
//
// don is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// don is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with don. If not, see <https://www.gnu.org/licenses/>.
use clap::Parser;
use serenity::http::Http;
use serenity::model::channel::Embed;
use serenity::model::webhook::Webhook;
use std::io;

#[derive(Parser)]
#[command(about, author, version)]
struct Args {
    /// Username of the user running this
    #[arg(short, long, env = "USER")]
    user: String,
    /// URL of the discord webhook
    #[arg(long, env = "WEBHOOK_URL")]
    webhook_url: String,
    /// Whether to ping everyone
    #[arg(short, long)]
    everyone: bool,
    /// Message to send
    message: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    // Parse CLI arguments
    let args = Args::parse();
    // Http client, doesn't need a token or initialization
    let http = Http::new("");
    // Initialize a webhook using the url from environment variables
    let webhook = Webhook::from_url(&http, &args.webhook_url).await?;
    // Get the hostname
    let hostname = hostname::get()?;
    let hostname = hostname.to_str().unwrap_or("non-unicode-hostname");
    // Send the message over the webhook
    webhook
        .execute(&http, false, |w| {
            w.username(format!("{}@{}", args.user, hostname))
                .embeds(vec![Embed::fake(|e| {
                    let e = e.title("Alert");
                    let e = if args.everyone {
                        e.field("mention", "@everyone", true)
                    } else {
                        e
                    };
                    e.field("message", &args.message, false)
                })])
        })
        .await?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Error getting info: {0}")]
    Io(#[from] io::Error),
    #[error("Error in serenity: {0}")]
    Serenity(#[from] serenity::Error),
}
