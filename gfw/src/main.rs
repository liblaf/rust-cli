mod api;
mod cmd;

use crate::cmd::Cmd;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cmd = Cmd::parse();
    cmd.run().await
}
