mod install;
mod list;
mod update;

use clap::{Args, Subcommand};

#[derive(Args)]
pub struct Cmd {
    #[command(subcommand)]
    cmd: SubCmd,
    #[arg(short, long, global(true))]
    zone_id: Option<String>,
}

#[derive(Subcommand)]
enum SubCmd {
    Install(install::Cmd),
    List(list::Cmd),
    Update(update::Cmd),
}

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        match &self.cmd {
            SubCmd::Install(cmd) => cmd.run().await,
            SubCmd::List(cmd) => cmd.run().await,
            SubCmd::Update(cmd) => cmd.run().await,
        }
    }
}
