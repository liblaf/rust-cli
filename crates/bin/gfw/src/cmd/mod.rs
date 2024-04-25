mod ip;
mod list;
mod now;

use clap::{CommandFactory, Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use cli::{
    color::ColorInit,
    log::{DefaultLevel, LogInit},
};

#[derive(Parser)]
pub struct Cmd {
    #[command(subcommand)]
    cmd: SubCmd,
    #[command(flatten)]
    color: concolor_clap::Color,
    #[command(flatten)]
    verbose: Verbosity<DefaultLevel>,
}

#[derive(Subcommand)]
enum SubCmd {
    Complete(cli::complete::Cmd),
    Ip(ip::Cmd),
    List(list::Cmd),
    Now(now::Cmd),
}

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        self.color.init();
        self.verbose.init();
        match &self.cmd {
            SubCmd::Complete(cmd) => cmd.run(Cmd::command()),
            SubCmd::Ip(cmd) => cmd.run().await,
            SubCmd::List(cmd) => cmd.run().await,
            SubCmd::Now(cmd) => cmd.run().await,
        }
    }
}
