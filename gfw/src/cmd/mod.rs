mod ip;
mod list;
mod now;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use cli::{
    color::ColorInit,
    log::{DefaultLevel, LogInit},
};

#[derive(Parser)]
#[clap(color = concolor_clap::color_choice())]
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
    Ip(ip::Cmd),
    List(list::Cmd),
    Now(now::Cmd),
}

impl Cmd {
    pub async fn run(&self) -> anyhow::Result<()> {
        self.color.init();
        self.verbose.init();
        match &self.cmd {
            SubCmd::Ip(cmd) => cmd.run().await,
            SubCmd::List(cmd) => cmd.run().await,
            SubCmd::Now(cmd) => cmd.run().await,
        }
    }
}
