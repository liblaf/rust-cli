mod ip;
mod now;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cmd {
    #[command(subcommand)]
    cmd: SubCmd,
}

#[derive(Subcommand)]
enum SubCmd {
    Ip(ip::Cmd),
    Now(now::Cmd),
}

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        match &self.cmd {
            SubCmd::Ip(cmd) => cmd.run().await,
            SubCmd::Now(cmd) => cmd.run().await,
        }
    }
}
