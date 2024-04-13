mod api;
mod cmd;
mod proc;

use crate::cmd::Cmd;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cmd = Cmd::parse();
    cmd.run().await
}
